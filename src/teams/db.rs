use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::errors::TalliiError;
use crate::Result;

#[derive(FromRow, Serialize, Debug)]
pub struct Team {
    pub team_id: i32,
    pub scoreboard_id: i32,
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
            "#,
        )
        .fetch_all(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// fetches all teams for a specific scoreboard
    pub async fn get_teams_by_scoreboard_id(
        conn: &PgPool,
        scoreboard_id: &i32,
    ) -> Result<Vec<Team>> {
        sqlx::query_as::<_, Team>(
            r#"
                select
                    *
                from
                    teams
                where
                    scoreboard_id = $1
                "#,
        )
        .bind(scoreboard_id)
        .fetch_all(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// fetches all teams for many scoreboard ids
    pub async fn get_teams_by_scoreboard_created_by(
        conn: &PgPool,
        user_id: &i32,
    ) -> Result<Vec<Team>> {
        sqlx::query_as::<_, Team>(
            r#"
              select
                  team_id, t.scoreboard_id, t.name, t.created_at
              from
                  teams t
              inner join
                  scoreboards s
              on
                  t.scoreboard_id = s.scoreboard_id
              where
                  s.created_by = $1
              "#,
        )
        .bind(user_id)
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
                "#,
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
            "#,
        )
        .bind(&payload.name)
        .bind(&payload.scoreboard_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}
