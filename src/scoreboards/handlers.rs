use std::sync::Arc;
use futures::future;

use jsonwebtoken::TokenData;
use serde::Serialize;
use sqlx::PgPool;

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
    pub teams: Vec<teams::db::Team>,
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
        teams,
    };

    Ok(warp::reply::json(&scoreboard_response))
}

/// gets all scoreboards
pub async fn get_scoreboards(
    pool: Arc<PgPool>,
    _token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let scoreboards = db::Scoreboard::get_scoreboards(&pool).await?;
    Ok(warp::reply::json(&scoreboards))
}
