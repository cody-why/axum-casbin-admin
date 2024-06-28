use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response;
use tracing::info;

use crate::context;
use crate::middleware::context::UserContext;
use crate::utils::jwt_util::JWTToken;

pub async fn auth_layer(
    jwt_token: Result<JWTToken, String>, mut req: Request, next: Next,
) -> Result<response::Response, StatusCode> {
    // tracing::warn!("auth_layer req {:?} {:?}", req.method(), req.uri());
    let path = req.uri().to_string();
    if context().config.white_list_api.contains(&path) {
        return Ok(next.run(req).await);
    }
    let mut jwt_token = match jwt_token {
        Ok(token) => token,
        Err(err) => {
            info!("auth failed: {}", err);
            return Err(StatusCode::UNAUTHORIZED);
        },
    };

    let is_qm = &path == "/admin/query_user_menu";

    let auth = is_qm
        || jwt_token.permissions.first() == Some(&"*".to_string())
        || jwt_token.permissions.iter().any(|permission| permission == &path);
    if !auth {
        tracing::warn!("auth_layer req {:?} {:?} auth={}", req.method(), req.uri(), auth);
        return Err(StatusCode::UNAUTHORIZED);
    }

    let context = UserContext { id: jwt_token.id };
    req.extensions_mut().insert(context);
    let mut rep = next.run(req).await;

    // 刷新token
    let new_token = jwt_token.check_refresh();
    if let Ok(token) = new_token {
        tracing::debug!("refresh token: {}", token);
        let token = format!("Bearer {}", token); //Authorization: Bearer <token>
        rep.headers_mut()
            .insert("Authorization", HeaderValue::from_str(&token).unwrap());
        rep.headers_mut()
            .insert("Access-Control-Expose-Headers", HeaderValue::from_str("authorization").unwrap());
    }
    Ok(rep)
}
