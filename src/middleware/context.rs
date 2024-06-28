use axum::{extract::FromRequestParts, http::request::Parts};

#[derive(Debug, Clone)]
pub struct UserContext {
    pub id: u64,
}

// impl FromRequest 
#[axum::async_trait]
impl<S> FromRequestParts<S> for UserContext
where
    S: Send + Sync,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // info!("UserContext from_request_parts");
        if let Some(auth) = parts.extensions.get::<Self>() {
            return Ok(auth.clone());
        }

        Err("UserContext not found".into())
        
    }
}