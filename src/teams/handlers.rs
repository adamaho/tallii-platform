use std::sync::Arc;

use jsonwebtoken::TokenData;
use sqlx::PgPool;

use crate::auth::token::Claims;
use crate::ResponseResult;

use super::db;

/// gets a single team
pub async fn get_team(
    team_id: i32,
    pool: Arc<PgPool>,
    _claims: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let team = db::Team::get_team(&pool, &team_id).await?;
    Ok(warp::reply::json(&team))
}

/// gets all teams
pub async fn get_teams(
    pool: Arc<PgPool>,
    _claims: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let teams = db::Team::get_teams(&pool).await?;
    Ok(warp::reply::json(&teams))
}
