use sqlx::{Postgres, Transaction, PgPool};

use crate::errors::TalliiError;
use crate::Result;

/// Representation of a user in the database
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Player {
    pub team_player_id: i32,
    pub team_id: i32,
    pub user_id: i32,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
}

/// Representation of a user in the database
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct PlayerRow {
    pub team_player_id: i32,
    pub team_id: i32,
    pub player_created_at: chrono::DateTime<chrono::offset::Utc>,
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub avatar_background: String,
    pub avatar_emoji: String,
    pub user_created_at: chrono::DateTime<chrono::offset::Utc>,
}

impl Player {
    /// gets all players for a specific team
    pub async fn get_players_by_team_id(conn: &PgPool, team_id: &i32) -> Result<Vec<PlayerRow>> {
        sqlx::query_as::<_, PlayerRow>(
            r#"
        select
          p.team_player_id,
          p.team_id,
          p.created_at as player_created_at,
          u.user_id,
          u.username,
          u.email,
          u.avatar_background,
          u.avatar_emoji,
          u.created_at as user_created_at
        from
          teams_players p
        inner join
          users u
        on
          p.user_id = u.user_id
        where
          team_id = $1
      "#,
        )
        .bind(team_id)
        .fetch_all(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// create players in a transaction
    pub async fn create_player(tx: &mut Transaction<'_, Postgres>, team_id: &i32, player: &i32) -> Result<Player> {
      sqlx::query_as::<_, Player>(
        r#"
        insert into
            teams_players (team_id, user_id)
        values
          ($1, $2)
        returning
          *
        "#
      )
      .bind(team_id)
      .bind(player)
      .fetch_one(tx)
      .await
      .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}
