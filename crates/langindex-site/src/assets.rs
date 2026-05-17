use axum::body::Body;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};

const STYLE_CSS: &str = include_str!("../assets/style.css");
const FAVICON_PNG: &[u8] = include_bytes!("../../../public/favicon.png");

pub async fn style_css() -> Response {
    static_text_response(STYLE_CSS, "text/css; charset=utf-8")
}

pub async fn favicon_png() -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, HeaderValue::from_static("image/png")),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=86400"),
            ),
        ],
        Body::from(FAVICON_PNG),
    )
        .into_response()
}

fn static_text_response(body: &'static str, content_type: &'static str) -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, HeaderValue::from_static(content_type)),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=3600"),
            ),
        ],
        body,
    )
        .into_response()
}
