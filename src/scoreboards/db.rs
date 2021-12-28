use serde::Serialize;
use sqlx::{FromRow, PgPool, Postgres, Transaction};

use crate::errors::TalliiError;
use crate::Result;

use super::handlers::CreateScoreboardPayload;

#[derive(FromRow, Serialize)]
pub struct Scoreboard {
    pub scoreboard_id: i32,
    pub name: String,
    pub game: String,
    pub created_by: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl Scoreboard {
    /// fetches all scoreboards
    pub async fn get_scoreboards_by_user_id(
        conn: &PgPool,
        user_id: &i32,
    ) -> Result<Vec<Scoreboard>> {
        sqlx::query_as::<_, Scoreboard>(
            r#"
                select
                    *
                from
                    scoreboards
                where
                    created_by = $1
                
            "#,
        )
        .bind(user_id)
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
            "#,
        )
        .bind(scoreboard_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// creates a scoreboard
    pub async fn create_scoreboard_tx(
        tx: &mut Transaction<'_, Postgres>,
        payload: &CreateScoreboardPayload,
        user_id: &i32,
    ) -> Result<Scoreboard> {
        sqlx::query_as::<_, Scoreboard>(
            r#"
                insert into
                    scoreboards (name, game, created_by)
                values
                    ($1, $2, $3)
                returning
                    *
            "#,
        )
        .bind(&payload.name)
        .bind(&payload.game)
        .bind(user_id)
        .fetch_one(tx)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}
