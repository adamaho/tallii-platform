use std::sync::Arc;
use std::collections::HashMap;
use futures::future;

use jsonwebtoken::TokenData;
use serde::Serialize;
use sqlx::PgPool;

use itertools::Itertools;

use crate::auth::token::Claims;
use crate::ResponseResult;

use super::db;

use crate::auth;
use crate::teams;

#[derive(Serialize)]
pub struct ScoreboardResponse {
    pub scoreboard_id: i32,
    pub name: String,
    pub created_by: auth::db::UserResponse,
    pub created_at: chrono::NaiveDateTime,
    pub teams: Option<Vec<teams::db::Team>>,
}

/// creates a scoreboard
pub async fn create_scoreboard(
    payload: db::CreateScoreboardPayload,
    pool: Arc<PgPool>,
    token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let scoreboard = db::Scoreboard::create_scoreboard(&pool, &payload, &token.claims.sub).await?;
    Ok(warp::reply::json(&scoreboard))
}

/// gets a single scoreboard
pub async fn get_scoreboard(
    scoreboard_id: i32,
    pool: Arc<PgPool>,
    _token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
  // get the scoreboard future
    let scoreboard_future = db::Scoreboard::get_scoreboard(&pool, &scoreboard_id);

    // get the teams for the scoreboard future
    let teams_future = teams::db::Team::get_teams_by_scoreboard_id(&pool, &scoreboard_id);

    // run the futures in parallel
    let (scoreboard, teams) = future::try_join(scoreboard_future, teams_future).await?;

    // get user that created the scoreboard
    let user = auth::db::User::get_by_user_id(&pool, &scoreboard.created_by).await?;

    // create the response
    let scoreboard_response = ScoreboardResponse {
        scoreboard_id: scoreboard.scoreboard_id,
        name: scoreboard.name,
        created_by: auth::db::UserResponse { user_id: user.user_id, username: user.username, email: user.email, created_at: user.created_at },
        created_at: scoreboard.created_at,
        teams: Some(teams),
    };

    Ok(warp::reply::json(&scoreboard_response))
}

/// gets all scoreboards where the created_by is the current user
pub async fn get_scoreboards(
    pool: Arc<PgPool>,
    token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {

    // get all scoreboards for current user
    let scoreboards_future = db::Scoreboard::get_scoreboards_by_user_id(&pool, &token.claims.sub);

    // get all teams for the scoreboards of the current user
    let teams_future = teams::db::Team::get_teams_by_scoreboard_created_by(&pool, &token.claims.sub);

    // get the current user info
    let user_future = auth::db::User::get_by_user_id(&pool, &token.claims.sub);

    // run the queries in parallel
    let (scoreboards, teams, user) = future::try_join3(scoreboards_future, teams_future, user_future).await?;

    // group the teams into a hashmap
    let mut grouped_teams: HashMap<i32, Vec<teams::db::Team>> = HashMap::new();
    for (scoreboard_id, teams) in &teams.into_iter().group_by(|team| team.scoreboard_id) {
      grouped_teams.insert(scoreboard_id, teams.collect::<Vec<teams::db::Team>>());
    }

    // build the response 
    let mut response: Vec<ScoreboardResponse> = Vec::new();

    for scoreboard in scoreboards.into_iter() {
      response.push(ScoreboardResponse {
        scoreboard_id: scoreboard.scoreboard_id,
        name: scoreboard.name,
        created_at: scoreboard.created_at,
        created_by: auth::db::UserResponse {
          user_id: user.user_id,
          username: user.username.clone(),
          email: user.email.clone(),
          created_at: user.created_at
        },
        // the remove is used to get teh value itself instead of the borrowed reference
        teams: grouped_teams.remove(&scoreboard.scoreboard_id)
      });
    }
  
    // get all teams for the scoreboards
    Ok(warp::reply::json(&response))
}
