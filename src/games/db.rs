use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::errors::TalliiError;
use crate::Result;

#[derive(FromRow, Serialize)]
pub struct Game {
    pub game_id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateGamePayload {
    pub name: String,
}

impl Game {
    /// fetches all games
    pub async fn get_games(conn: &PgPool) -> Result<Vec<Game>> {
        sqlx::query_as::<_, Game>(
            r#"
                select
                    *
                from
                    games
            "#,
        )
        .fetch_all(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// fetches a single game
    pub async fn get_game(conn: &PgPool, game_id: &i32) -> Result<Game> {
        sqlx::query_as::<_, Game>(
            r#"
                select
                    *
                from
                    games
                where
                    game_id = $1
            "#,
        )
        .bind(game_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// creates a game
    pub async fn create_game(conn: &PgPool, payload: &CreateGamePayload) -> Result<Game> {
        sqlx::query_as::<_, Game>(
            r#"
                insert into
                    games (name)
                values
                    ($1)
                returning
                    *
            "#,
        )
        .bind(&payload.name)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}
