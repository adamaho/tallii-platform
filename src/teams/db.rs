use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

use crate::errors::TalliiError;
use crate::Result;

#[derive(FromRow, Serialize)]
pub struct Team {
    pub team_id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateTeamPayload {
    pub name: String,
    pub scoreboard_id: i32,
}

impl Team {

    /// fetches all teams
    pub async fn get_teams(conn: &PgPool) -> Result<Vec<Team>> {
        sqlx::query_as::<_, Team>(
            r#"
                select
                    *
                from
                    teams
            "#
        )
        .fetch_all(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }


    /// fetches a single team
    pub async fn get_team(conn: &PgPool, team_id: &i32) -> Result<Team> {
        sqlx::query_as::<_, Team>(
            r#"
                select
                    *
                from
                    teams
                where
                    team_id = $1
            "#
        )
        .bind(team_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// creates a team
    pub async fn create_team(conn: &PgPool, payload: &CreateTeamPayload) -> Result<Team> {
        sqlx::query_as::<_, Team>(
            r#"
                insert into
                    teams (name, scoreboard_id)
                values
                    ($1, $2)
                returning
                    *
            "#
        )
        .bind(&payload.name)
        .bind(&payload.scoreboard_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}