use std::sync::Arc;

use jsonwebtoken::TokenData;
use serde::Deserialize;
use sqlx::PgPool;

use crate::auth::token::Claims;
use crate::ResponseResult;
use crate::errors::TalliiError;

use super::db;

use crate::scoreboards;

#[derive(Deserialize)]
pub struct UpdateTeamRequest {
    pub name: String,
    pub score: i32
}

/// gets a single team
pub async fn get_team(
    team_id: i32,
    pool: Arc<PgPool>,
    _claims: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let team = db::Team::get_team(&pool, &team_id).await?;

    // TODO check if the user has perms to get the team

    Ok(warp::reply::json(&team))
}

/// gets all teams
pub async fn get_teams(
    pool: Arc<PgPool>,
    _claims: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let teams = db::Team::get_teams(&pool).await?;

    // TODO: check if the user has perms to get the teams
    Ok(warp::reply::json(&teams))
}

/// updates a team
pub async fn update_team(
    team_id: i32,
    payload: UpdateTeamRequest,
    pool: Arc<PgPool>,
    token: TokenData<Claims>
) -> ResponseResult<impl warp::Reply> {
    // get the team
    let team = db::Team::get_team(&pool, &team_id).await?;

    // get the scoreboard for the team
    let scoreboard = scoreboards::db::Scoreboard::get_scoreboard(&pool, &team.scoreboard_id).await?;

    // check if the user can perform this action
    if scoreboard.created_by != token.claims.sub {
        return Err(warp::reject::custom(TalliiError::Forbidden));
    }

    // update the team
    let updated_team = db::Team::update_team(&pool, &team_id, &payload).await?;

    Ok(warp::reply::json(&updated_team))
}
