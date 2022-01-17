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
    pub avatar_background: String,
    pub avatar_emoji: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
}

/// Representation of a user without the password for a response
#[derive(sqlx::FromRow, serde::Serialize)]
pub struct UserResponse {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub avatar_background: String,
    pub avatar_emoji: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
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
    pub async fn create_user(
        conn: &PgPool,
        username: &str,
        email: &str,
        hash: &str,
    ) -> Result<User> {
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

    /// updates a user
    pub async fn update_user(
        conn: &PgPool,
        user_id: &i32,
        username: &str,
        avatar_background: &str,
        avatar_emoji: &str,
    ) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            r#"
            update
                users
            set
                username = $1,
                avatar_background = $2,
                avatar_emoji = $3
            where
                user_id = $4
            returning
                *
        "#,
        )
        .bind(username)
        .bind(avatar_background)
        .bind(avatar_emoji)
        .bind(user_id)
        .fetch_one(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    /// searches for users matching the string
    pub async fn search_users(
        conn: &PgPool,
        query: &String
    ) -> Result<Vec<UserResponse>> {
        let like_term = format!("%{}%", query);

        let users = sqlx::query_as::<_, UserResponse>(
            r#"
            select
                *
            from
                users
            where
                username
            like
                $1
            order by
                username
        "#,
        )
        .bind(&like_term)
        .fetch_all(conn)
        .await
        .map_err(|e| TalliiError::DatabaseError(e.to_string()))?;

        Ok(users)
    }
}
