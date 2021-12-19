use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::Result;

/// Representation of a user in the database
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a user without the password for a response
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
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

    /// Creates an email
    pub async fn create_user(conn: &PgPool, username: &str, email: &str, hash: &str) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            insert into
                users (username, email, password)
            values
                ($1, $2, $3)
            returning
                *
        "#,
        )
        .bind(username)
        .bind(email)
        .bind(hash)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))?;

        Ok(user)
    }
}
