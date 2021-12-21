use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

use crate::errors::TalliiError;
use crate::Result;

#[derive(FromRow, Serialize)]
pub struct Scoreboard {
    pub scoreboard_id: i32,
    pub name: String,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateScoreboardPayload {
    pub name: String,
    pub game_id: i32
}

impl Scoreboard {

    /// fetches all scoreboards
    pub async fn get_scoreboards(conn: &PgPool) -> Result<Vec<Scoreboard>> {
        sqlx::query_as::<_, Scoreboard>(
            r#"
                select
                    *
                from
                    scoreboards
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }


    /// fetches a single scoreboard
    pub async fn get_scoreboard(conn: &PgPool, scoreboard_id: &i32) -> Result<Scoreboard> {
        sqlx::query_as::<_, Scoreboard>(
            r#"
                select
                    *
                from
                    scoreboards
                where
                    scoreboard_id = $1
            "#
        )
        .bind(scoreboard_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// creates a scoreboard
    pub async fn create_scoreboard(conn: &PgPool, payload: &CreateScoreboardPayload, user_id: &i32) -> Result<Scoreboard> {
        sqlx::query_as::<_, Scoreboard>(
            r#"
                insert into
                    scoreboards (name, game_id, created_by)
                values
                    ($1, $2, $3)
                returning
                    *
            "#
        )
        .bind(&payload.name)
        .bind(&payload.game_id)
        .bind(user_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}