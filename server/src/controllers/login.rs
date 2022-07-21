use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    headers,
    http::{self, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension, Json, TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::{
    session::{SessionId, AXUM_SESSION_COOKIE_NAME},
    DatabaseConnection,
};

#[derive(sqlx::FromRow, Clone)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub password: String,
    pub salt: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

// TODO: Remove error information before production
pub async fn login(
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
    Json(credentials): Json<LoginRequest>,
    // session_id: SessionIdFromSession,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, impl IntoResponse> {
    // Remove previous session
    let result = logout_internal(&store, &cookies).await;
    if result == StatusCode::OK {
        tracing::debug!("Previous session logged out");
    }

    let account: Account = sqlx::query_as!(
        Account,
        "select * from account where name = $1",
        credentials.name
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Account not found".to_owned()))?;

    let argon2 = Argon2::default();
    let parsed_password = PasswordHash::new(&account.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to parse password hash".to_string(),
        )
    })?;

    if argon2
        .verify_password(&credentials.password.as_bytes(), &parsed_password)
        .is_ok()
    {
        let user_id = SessionId::new();
        let mut session = Session::new();
        session.insert("user_id", user_id).unwrap();
        let cookie = store.store_session(session).await.unwrap().unwrap();

        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::SET_COOKIE,
            HeaderValue::from_str(format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str())
                .unwrap(),
        );

        Ok((
            headers,
            format!(
                "user_id={:?} session_cookie_name={} create_new_session_cookie={}",
                SessionId::new(),
                AXUM_SESSION_COOKIE_NAME,
                true
            ),
        ))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Wrong password!".to_owned()))
    }
}

pub async fn logout(
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> StatusCode {
    logout_internal(&store, &cookies).await
}

async fn logout_internal(store: &MemoryStore, cookies: &headers::Cookie) -> StatusCode {
    let cookie = cookies.get(AXUM_SESSION_COOKIE_NAME).unwrap();
    let session = match store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        // No session active, just redirect
        None => return StatusCode::NOT_FOUND,
    };

    store.destroy_session(session).await.unwrap();
    StatusCode::OK
}

pub async fn protected(_session_id: SessionId) -> StatusCode {
    StatusCode::OK
}
