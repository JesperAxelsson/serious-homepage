use async_session::{MemoryStore, Session, SessionStore};
use axum::{
    headers,
    http::{self, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Extension, Json, TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::{DatabaseConnection, SessionId, SessionIdFromSession, AXUM_SESSION_COOKIE_NAME};

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

// use crate::error::Error;
// use crate::auth::{self, Role};

// TODO: Remove error information before production
// TODO: Handle user already logged in as other user or self
pub async fn login(
    Json(credentials): Json<LoginRequest>,
    Extension(store): Extension<MemoryStore>,
    // session_id: SessionIdFromSession,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let account = sqlx::query_as!(
        Account,
        "select * from account where name = $1",
        credentials.name
    )
    .fetch_one(&mut conn)
    .await;

    match account {
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Account not found".to_owned())),
        Ok(user) => {
            if credentials.password == user.password {
                let user_id = SessionId::new();
                let mut session = Session::new();
                session.insert("user_id", user_id).unwrap();
                let cookie = store.store_session(session).await.unwrap().unwrap();

                let mut headers = HeaderMap::new();
                headers.insert(
                    http::header::SET_COOKIE,
                    HeaderValue::from_str(
                        format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str(),
                    )
                    .unwrap(),
                );
                // (headers, new_user.user_id, true)
                // (StatusCode::OK, "Logged in".to_owned())
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
    }

    // let user_id = UserId::new();
    // let mut session = Session::new();
    // session.insert("user_id", user_id).unwrap();
    // let cookie = store.store_session(session).await.unwrap().unwrap();
    // return Ok(Self::CreatedFreshUserId(FreshUserId {
    //     user_id,
    //     cookie: HeaderValue::from_str(
    //         format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str(),
    //     )
    //     .unwrap(),
    // }));
}

pub async fn logout(
    Extension(store): Extension<MemoryStore>,
    TypedHeader(cookies): TypedHeader<headers::Cookie>,
) -> StatusCode {
    let cookie = cookies.get(AXUM_SESSION_COOKIE_NAME).unwrap();
    let session = match store.load_session(cookie.to_string()).await.unwrap() {
        Some(s) => s,
        // No session active, just redirect
        None => return StatusCode::NOT_FOUND,
    };

    store.destroy_session(session).await.unwrap();
    StatusCode::OK
}

pub async fn protected(session_id: SessionIdFromSession) -> StatusCode {
    match session_id {
        SessionIdFromSession::FoundUserId(_) => StatusCode::OK,
        SessionIdFromSession::NotFound => StatusCode::IM_A_TEAPOT,
    }
}
