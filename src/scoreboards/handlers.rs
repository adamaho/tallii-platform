use std::sync::Arc;

use serde::{Deserialize, Serialize};
use jsonwebtoken::TokenData;
use sqlx::PgPool;
use validator::Validate;

use crate::auth::token::Claims;
use crate::ResponseResult;

// #[derive(Deserialize, Validate)]
// pub struct CreateTeamPayload {
//     pub name: String
// }

// #[derive(Deserialize, Validate)]
// pub struct CreateScoreboardPayload {
//     pub name: String,
//     pub game: CreateGamePayload,
//     pub teams: Vec<CreateTeamPayload>
// }

// #[derive(Serialize, Validate)]
// pub struct CreateScoreboardResponse {
//     pub name: String,
// }


// pub async fn create_scoreboard(payload: CreateScoreboardPayload, pool: Arc<PgPool>, claims: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {

//     let scoreboard = CreateScoreboardResponse {
//         name: String::from("hello")
//     };

//     Ok(warp::reply::json(&scoreboard))
// }

// /// updates a scoreboard
// pub async fn update_scoreboard(payload: CreateScoreboardPayload, pool: Arc<PgPool>, claims: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {

//     let scoreboard = CreateScoreboardResponse {
//         name: String::from("hello")
//     };

//     Ok(warp::reply::json(&scoreboard))
// }


// /// deletes a scoreboard
// pub async fn delete_scoreboard(payload: CreateScoreboardPayload, pool: Arc<PgPool>, claims: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {

//     let scoreboard = CreateScoreboardResponse {
//         name: String::from("hello")
//     };

//     Ok(warp::reply::json(&scoreboard))
// }

// /// gets a scoreboard
// pub async fn get_scoreboard(scoreboard_id: i32, pool: Arc<PgPool>, claims: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {

//     let scoreboard = CreateScoreboardResponse {
//         name: String::from("hello")
//     };

//     Ok(warp::reply::json(&scoreboard))
// }


// /// gets all scoreboards for a specific user
// pub async fn get_scoreboards(pool: Arc<PgPool>, claims: TokenData<Claims>) -> ResponseResult<impl warp::Reply> {

//     let scoreboard = CreateScoreboardResponse {
//         name: String::from("hello")
//     };

//     Ok(warp::reply::json(&scoreboard))
// }