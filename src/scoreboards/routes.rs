use std::sync::Arc;

use sqlx::PgPool;
use warp::Filter;

// use crate::config::Config;
use super::handlers;
use crate::wrappers::{with_auth, with_pool};

// pub struct ScoreboardRoutes;

// impl ScoreboardRoutes {
//     /// Init the scoreboard routes
//     pub fn init(
//         pool: Arc<PgPool>,
//     ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//         let scoreboard_routes = create_scoreboard(pool.clone());

//         scoreboard_routes
//     }
// }

// / Creates a scoreboard in the backend
// pub fn create_scoreboard(
//     pool: Arc<PgPool>
// ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("v1" / "signup")
//         .and(warp::post())
//         .and(warp::body::json())
//         .and(with_pool(pool.clone()))
//         .and(with_auth())
//         .and_then(handlers::create_scoreboard)
// }
