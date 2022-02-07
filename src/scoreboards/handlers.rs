use futures::future;
use std::collections::HashMap;
use std::sync::Arc;
use warp::hyper::StatusCode;

use itertools::Itertools;

use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::teams::db::CreateTeamPayload;
use crate::users::token::Claims;
use crate::{ResponseResult, Result};

use super::db;

use crate::errors::TalliiError;
use crate::teams;
use crate::users;

#[derive(Deserialize)]
pub struct CreateScoreboardPayload {
    pub name: String,
    pub game: String,
    pub teams: Vec<CreateTeamPayload>,
}

#[derive(Serialize)]
pub struct ScoreboardResponse {
    pub scoreboard_id: i32,
    pub name: String,
    pub game: String,
    pub created_by: users::db::UserResponse,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub teams: Option<Vec<teams::db::TeamRow>>,
}

async fn get_scoreboard_response(
    pool: Arc<PgPool>,
    scoreboard_id: &i32,
) -> Result<ScoreboardResponse> {
    // get the scoreboard future
    let scoreboard_future = db::Scoreboard::get_scoreboard(&pool, &scoreboard_id);

    // get the teams for the scoreboard future
    let teams_future = teams::db::Team::get_teams_by_scoreboard_id(&pool, &scoreboard_id);

    // run the futures in parallel
    let (scoreboard, teams) = future::try_join(scoreboard_future, teams_future).await?;

    // get user that created the scoreboard
    let user = users::db::User::get_by_user_id(&pool, &scoreboard.created_by).await?;

    // create the response
    Ok(ScoreboardResponse {
        scoreboard_id: scoreboard.scoreboard_id,
        name: scoreboard.name,
        game: scoreboard.game,
        created_by: users::db::UserResponse {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            avatar_background: user.avatar_background,
            avatar_emoji: user.avatar_emoji,
            created_at: user.created_at,
        },
        created_at: scoreboard.created_at,
        updated_at: scoreboard.updated_at,
        teams: Some(teams),
    })
}

/// creates a scoreboard
pub async fn create_scoreboard(
    payload: CreateScoreboardPayload,
    pool: Arc<PgPool>,
    token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    // get the transaction
    let mut tx = pool
        .begin()
        .await
        .map_err(|_err| warp::reject::custom(TalliiError::SQLXError))?;

    // create scoreboard
    let scoreboard =
        db::Scoreboard::create_scoreboard_tx(&mut tx, &payload, &token.claims.sub).await?;

    // create teams
    teams::db::Team::create_teams(&mut tx, &payload.teams, &scoreboard.scoreboard_id).await?;

    // commit the transaction
    tx.commit()
        .await
        .map_err(|_err| warp::reject::custom(TalliiError::SQLXError))?;

    // create the response
    let response = get_scoreboard_response(pool, &scoreboard.scoreboard_id).await?;

    // this response should be the same as the get scoreboard response
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::CREATED,
    ))
}

/// gets a single scoreboard
pub async fn get_scoreboard(
    scoreboard_id: i32,
    pool: Arc<PgPool>,
    _token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let scoreboard_response = get_scoreboard_response(pool.clone(), &scoreboard_id).await?;

    Ok(warp::reply::json(&scoreboard_response))
}

/// gets all scoreboards where the created_by is the current user
pub async fn get_user_scoreboards(
    user_id: i32,
    pool: Arc<PgPool>,
) -> ResponseResult<impl warp::Reply> {
    // get all scoreboards for current user
    let scoreboards_future = db::Scoreboard::get_scoreboards_by_user_id(&pool, &user_id);

    // get all teams for the scoreboards of the current user
    let teams_future = teams::db::Team::get_teams_by_scoreboard_created_by(&pool, &user_id);

    // get the current user info
    let user_future = users::db::User::get_by_user_id_option(&pool, &user_id);

    // run the queries in parallel
    let (scoreboards, teams, user_option) =
        future::try_join3(scoreboards_future, teams_future, user_future).await?;

    // if the user doesnt exist return with a 404
    if user_option.is_none() {
        return Err(warp::reject::not_found());
    }

    // extract the user from the option
    let user = user_option.unwrap();

    // group the teams into a hashmap
    let mut grouped_teams: HashMap<i32, Vec<teams::db::TeamRow>> = HashMap::new();
    for (scoreboard_id, teams) in &teams.into_iter().group_by(|team| team.scoreboard_id) {
        grouped_teams.insert(scoreboard_id, teams.collect::<Vec<teams::db::TeamRow>>());
    }

    // build the response
    let mut response: Vec<ScoreboardResponse> = Vec::new();

    for scoreboard in scoreboards.into_iter() {
        response.push(ScoreboardResponse {
            scoreboard_id: scoreboard.scoreboard_id,
            name: scoreboard.name,
            game: scoreboard.game,
            created_at: scoreboard.created_at,
            updated_at: scoreboard.updated_at,
            created_by: users::db::UserResponse {
                user_id: user.user_id,
                username: user.username.clone(),
                email: user.email.clone(),
                avatar_background: user.avatar_background.clone(),
                avatar_emoji: user.avatar_emoji.clone(),
                created_at: user.created_at,
            },
            // the remove is used to get the value itself instead of the borrowed reference
            teams: grouped_teams.remove(&scoreboard.scoreboard_id),
        });
    }

    // get all teams for the scoreboards
    Ok(warp::reply::json(&response))
}

/// deletes a specific scorebaord
pub async fn delete_scoreboard(
    scoreboard_id: i32,
    pool: Arc<PgPool>,
    token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    // get the scoreboard
    let scoreboard = db::Scoreboard::get_scoreboard(&pool, &scoreboard_id).await?;

    // if the creator is not the same as the requester, forbid the action
    if scoreboard.created_by != token.claims.sub {
        return Err(warp::reject::custom(TalliiError::Forbidden));
    }

    // delete the scoreboard
    db::Scoreboard::delete_scoreboard(&pool, &scoreboard_id).await?;

    // response with the scoreboard deleted
    Ok(warp::reply::with_status(
        "scoreboard deleted",
        StatusCode::OK,
    ))
}
