use async_session::{async_trait, MemoryStore, SessionStore};
use axum::{
    extract::{rejection::TypedHeaderRejectionReason, FromRequestParts},
    headers,
    http::{header, request::Parts, StatusCode},
    Extension, TypedHeader,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const AXUM_SESSION_COOKIE_NAME: &str = "serious_session";

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for SessionId
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        tracing::debug!("From req cookies!");

        let Extension(store) = Extension::<MemoryStore>::from_request_parts(req, state)
            .await
            .expect("`MemoryStore` extension missing");

        let cookies = TypedHeader::<headers::Cookie>::from_request_parts(req, state)
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => {
                        (StatusCode::UNAUTHORIZED, "Cookie header missing")
                    }
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;

        let session_cookie = cookies.get(AXUM_SESSION_COOKIE_NAME).ok_or((
            StatusCode::UNAUTHORIZED,
            "Could not get axum cookie from cookie",
        ))?;

        tracing::debug!("Session cookie: {:?}", session_cookie);

        if let Some(session) = store.load_session(session_cookie.to_owned()).await.unwrap() {
            if let Some(user_id) = session.get::<SessionId>("user_id") {
                tracing::debug!(
                    "UserIdFromSession: session decoded success, user_id={:?}",
                    user_id
                );
                Ok(user_id)
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No `user_id` found in session",
                ))
            }
        } else {
            Err((StatusCode::UNAUTHORIZED, "Failed to find active cookie"))
        }
    }
}
