use std::sync::Arc;

use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::players::db::Player;
use crate::users::db::UserResponse;
use crate::users::token::Claims;
use crate::ResponseResult;

use super::db;

use crate::scoreboards;

#[derive(Serialize)]
pub struct PlayerResponse {
    pub team_player_id: i32,
    pub team_id: i32,
    pub user: UserResponse,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Serialize)]
pub struct TeamResponse {
    pub team_id: i32,
    pub scoreboard_id: i32,
    pub name: String,
    pub score: i32,
    pub players: Vec<PlayerResponse>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Deserialize)]
pub struct UpdateTeamRequest {
    pub name: String,
    pub score: i32,
}

/// gets a single team
pub async fn get_team(
    team_id: i32,
    pool: Arc<PgPool>,
    _claims: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let team = db::Team::get_team(&pool, &team_id).await?;

    let players = Player::get_players_by_team_id(&pool, &team_id).await?;

    let response = TeamResponse {
        team_id: team.team_id,
        scoreboard_id: team.scoreboard_id,
        name: team.name,
        score: team.score,
        created_at: team.created_at,
        players: players.into_iter().map(|p| {
            return PlayerResponse {
                team_player_id: p.team_player_id,
                team_id: p.team_id,
                created_at: p.player_created_at,
                user: UserResponse {
                    user_id: p.user_id,
                    email: p.email,
                    username: p.username,
                    avatar_background: p.avatar_background,
                    avatar_emoji: p.avatar_emoji,
                    created_at: p.user_created_at,
                }
            }
        }).collect()
    };

    Ok(warp::reply::json(&response))
}

/// gets all teams
pub async fn get_teams(
    pool: Arc<PgPool>,
    _claims: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    let teams = db::Team::get_teams(&pool).await?;
    Ok(warp::reply::json(&teams))
}

/// updates a team
pub async fn update_team(
    team_id: i32,
    payload: UpdateTeamRequest,
    pool: Arc<PgPool>,
    token: TokenData<Claims>,
) -> ResponseResult<impl warp::Reply> {
    // get the team
    let team = db::Team::get_team(&pool, &team_id).await?;

    // get the scoreboard for the team
    let scoreboard =
        scoreboards::db::Scoreboard::get_scoreboard(&pool, &team.scoreboard_id).await?;

    // check if the user can perform this action
    if scoreboard.created_by != token.claims.sub {
        return Err(warp::reject::custom(TalliiError::Forbidden));
    }

    // update the team
    let updated_team = db::Team::update_team(&pool, &team_id, &payload).await?;

    Ok(warp::reply::json(&updated_team))
}
