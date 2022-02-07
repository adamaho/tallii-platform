use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, Transaction};

use crate::errors::TalliiError;
use crate::players::db::Player;
use crate::Result;

use super::handlers::UpdateTeamRequest;

#[derive(FromRow, Serialize, Debug)]
pub struct TeamRow {
    pub team_id: i32,
    pub scoreboard_id: i32,
    pub name: String,
    pub score: i32,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
}

#[derive(Deserialize)]
pub struct CreateTeamPayload {
    pub name: String,
    pub players: Vec<i32>,
}

pub struct Team;

impl Team {
    /// fetches all teams
    pub async fn get_teams(conn: &PgPool) -> Result<Vec<TeamRow>> {
        sqlx::query_as::<_, TeamRow>(
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
    ) -> Result<Vec<TeamRow>> {
        sqlx::query_as::<_, TeamRow>(
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
    ) -> Result<Vec<TeamRow>> {
        sqlx::query_as::<_, TeamRow>(
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
              order by
                s.scoreboard_id
              "#,
        )
        .bind(user_id)
        .fetch_all(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// fetches a single team
    pub async fn get_team(conn: &PgPool, team_id: &i32) -> Result<TeamRow> {
        sqlx::query_as::<_, TeamRow>(
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
    ) -> Result<()> {
        // create each team one by one
        for team in teams.iter() {
            let created_team = sqlx::query_as::<_, TeamRow>(
                r#"
                    insert into
                        teams (name, scoreboard_id)
                    values
                        ($1, $2)
                    returning
                        *
                "#,
            )
            .bind(&team.name)
            .bind(scoreboard_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| TalliiError::DatabaseError(e.to_string()))?;

            for player in team.players.clone().into_iter() {
                // create each player one by one
                Player::create_player(&mut *tx, &created_team.team_id, &player).await?;
            }
        }

        Ok(())
    }

    /// updates a specific team
    pub async fn update_team(
        pool: &PgPool,
        team_id: &i32,
        payload: &UpdateTeamRequest,
    ) -> Result<TeamRow> {
        sqlx::query_as::<_, TeamRow>(
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
            "#,
        )
        .bind(&payload.name)
        .bind(&payload.score)
        .bind(team_id)
        .fetch_one(pool)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}
