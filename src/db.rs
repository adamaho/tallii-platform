use log::info;
use sqlx::PgPool;

use crate::errors::{TalliiError};
use crate::Result;

/// Representation of a user in the database
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    /// Creates a user in the database
    pub async fn get_or_create(conn: &PgPool, email: &str) -> Result<User> {
        if let Some(user) = User::get_by_email_option(conn, email).await? {
            return Ok(user);
        } else {
            let user = sqlx::query_as::<_, User>(
                r#"
                insert into
                    users (email)
                values
                    ($1)
                returning
                    *
            "#,
            )
            .bind(email)
            .fetch_one(conn)
            .await
            .map_err(|e| TalliiError::DatabaseError(e.to_string()))?;

            return Ok(user);
        }
    }

    /// Gets a user by their email
    pub async fn get_by_email_option(conn: &PgPool, email: &str) -> Result<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
            select
                *
            from
                users
            where
                users.email = $1
        "#,
        )
        .bind(email)
        .fetch_optional(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// Gets a user by their email
    pub async fn get_by_email(conn: &PgPool, email: &str) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            select
                *
            from
                users
            where
                users.email = $1
        "#,
        )
        .bind(email)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }

    /// Gets a user by their email
    pub async fn get_by_user_id(conn: &PgPool, user_id: &i32) -> Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            select
                *
            from
                users
            where
                users.user_id = $1
        "#,
        )
        .bind(user_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))
    }
}
