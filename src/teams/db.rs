use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, Transaction};

use crate::errors::TalliiError;
use crate::Result;

use super::handlers::UpdateTeamRequest;

#[derive(FromRow, Serialize, Debug)]
pub struct Team {
    pub team_id: i32,
    pub scoreboard_id: i32,
    pub name: String,
    pub score: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Deserialize)]
pub struct CreateTeamPayload {
    pub name: String,
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
                  team_id, t.scoreboard_id, t.name, t.score, t.created_at
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

    /// creates many teams
    pub async fn create_teams(
        tx: &mut Transaction<'_, Postgres>,
        teams: &Vec<CreateTeamPayload>,
        scoreboard_id: &i32,
    ) -> Result<Team> {
        let mut names: Vec<&str> = Vec::new();
        let mut scoreboard_ids: Vec<i32> = Vec::new();
        let owned_scoreboard_id = scoreboard_id.to_owned();

        // create the values
        for team in teams.iter() {
            names.push(&team.name);
            scoreboard_ids.push(owned_scoreboard_id);
        }

        sqlx::query_as::<_, Team>(
            r#"
                insert into
                    teams (name, scoreboard_id)
                select
                    *
                from
                    unnest($1, $2)
                returning
                    *
            "#,
        )
        .bind(names)
        .bind(scoreboard_ids)
        .fetch_one(tx)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// updates a specific team
    pub async fn update_team(
        pool: &PgPool,
        team_id: &i32,
        payload: &UpdateTeamRequest
    ) -> Result<Team> {
        sqlx::query_as::<_, Team>(
            r#"
                update
                    teams
                set
                    name = $1,
                    score = $2
                where
                    team_id = $3 
                returning
                    *
            "#
        )
        .bind(&payload.name)
        .bind(&payload.score)
        .bind(team_id)
        .fetch_one(pool)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}
