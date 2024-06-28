use axum::body::{Body, Bytes};
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use http_body_util::BodyExt;

// #[tracing::instrument(skip(next),ret,err(Debug))]
pub async fn log_layer(req: Request, next: Next) -> Result<impl IntoResponse, (StatusCode, String)> {
    let instant = std::time::Instant::now();
    let url = format!(
        "{} {}",
        req.method(),
        req.uri()
    );
    let (parts, body) = req.into_parts();
    let bytes = buffer_and_print(body, true, &url).await?;
    let req = Request::from_parts(parts, Body::from(bytes));
    let res = next.run(req).await;
    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print(body, false, &url).await?;
    let res = Response::from_parts(parts, Body::from(bytes));
    let time = instant.elapsed().as_millis();
    tracing::warn!(
        "{} {}ms {}",
        url,
        time,
        res.status()
    );
    Ok(res)
}

pub async fn buffer_and_print<B>(body: B, is_request: bool, url: &String) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read body: {err}"),
            ));
        },
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        if is_request {
            tracing::warn!("{url} request  = {body}");
        } else if body.len() > 512 {
            let len = body.char_indices().nth(512).unwrap_or((body.len(), ' '));
            let body = &body[..len.0];
            tracing::info!("{url} response(limit) = {body}");
        } else {
            tracing::info!("{url} response = {body}");
        }
    }
    Ok(bytes)
}
