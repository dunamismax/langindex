use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};

const STYLE_CSS: &str = include_str!("../assets/style.css");
const APPLE_TOUCH_ICON: &[u8] = include_bytes!("../../../public/apple-touch-icon.png");
const FAVICON_ICO: &[u8] = include_bytes!("../../../public/favicon.ico");
const FAVICON_16: &[u8] = include_bytes!("../../../public/favicon-16x16.png");
const FAVICON_32: &[u8] = include_bytes!("../../../public/favicon-32x32.png");
const FAVICON_PNG: &[u8] = include_bytes!("../../../public/favicon.png");
const SITE_WEBMANIFEST: &str = include_str!("../../../public/site.webmanifest");

const LOGO: &[u8] = include_bytes!("../../../public/brand/langindex-logo.png");
const LOGO_64: &[u8] = include_bytes!("../../../public/brand/langindex-logo-64.png");
const LOGO_192: &[u8] = include_bytes!("../../../public/brand/langindex-logo-192.png");
const LOGO_512: &[u8] = include_bytes!("../../../public/brand/langindex-logo-512.png");
const BANNER_DARK_PNG: &[u8] = include_bytes!("../../../public/brand/langindex-banner-dark.png");
const BANNER_LIGHT_PNG: &[u8] = include_bytes!("../../../public/brand/langindex-banner-light.png");
const BANNER_DARK_WEBP: &[u8] = include_bytes!("../../../public/brand/langindex-banner-dark.webp");
const BANNER_DARK_724_WEBP: &[u8] =
    include_bytes!("../../../public/brand/langindex-banner-dark-724.webp");
const BANNER_DARK_1448_WEBP: &[u8] =
    include_bytes!("../../../public/brand/langindex-banner-dark-1448.webp");
const BANNER_LIGHT_WEBP: &[u8] =
    include_bytes!("../../../public/brand/langindex-banner-light.webp");
const BANNER_LIGHT_724_WEBP: &[u8] =
    include_bytes!("../../../public/brand/langindex-banner-light-724.webp");
const BANNER_LIGHT_1448_WEBP: &[u8] =
    include_bytes!("../../../public/brand/langindex-banner-light-1448.webp");

pub async fn style_css() -> Response {
    static_text_response(STYLE_CSS, "text/css; charset=utf-8")
}

pub async fn favicon_png() -> Response {
    static_binary_response(FAVICON_PNG, "image/png")
}

pub async fn favicon_ico() -> Response {
    static_binary_response(FAVICON_ICO, "image/x-icon")
}

pub async fn favicon_16() -> Response {
    static_binary_response(FAVICON_16, "image/png")
}

pub async fn favicon_32() -> Response {
    static_binary_response(FAVICON_32, "image/png")
}

pub async fn apple_touch_icon() -> Response {
    static_binary_response(APPLE_TOUCH_ICON, "image/png")
}

pub async fn site_webmanifest() -> Response {
    static_text_response(SITE_WEBMANIFEST, "application/manifest+json; charset=utf-8")
}

pub async fn brand_asset(Path(file): Path<String>) -> Response {
    match brand_file(&file) {
        Some((body, content_type)) => static_binary_response(body, content_type),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

fn brand_file(file: &str) -> Option<(&'static [u8], &'static str)> {
    match file {
        "langindex-logo.png" => Some((LOGO, "image/png")),
        "langindex-logo-64.png" => Some((LOGO_64, "image/png")),
        "langindex-logo-192.png" => Some((LOGO_192, "image/png")),
        "langindex-logo-512.png" => Some((LOGO_512, "image/png")),
        "langindex-banner-dark.png" => Some((BANNER_DARK_PNG, "image/png")),
        "langindex-banner-light.png" => Some((BANNER_LIGHT_PNG, "image/png")),
        "langindex-banner-dark.webp" => Some((BANNER_DARK_WEBP, "image/webp")),
        "langindex-banner-dark-724.webp" => Some((BANNER_DARK_724_WEBP, "image/webp")),
        "langindex-banner-dark-1448.webp" => Some((BANNER_DARK_1448_WEBP, "image/webp")),
        "langindex-banner-light.webp" => Some((BANNER_LIGHT_WEBP, "image/webp")),
        "langindex-banner-light-724.webp" => Some((BANNER_LIGHT_724_WEBP, "image/webp")),
        "langindex-banner-light-1448.webp" => Some((BANNER_LIGHT_1448_WEBP, "image/webp")),
        _ => None,
    }
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

fn static_binary_response(body: &'static [u8], content_type: &'static str) -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, HeaderValue::from_static(content_type)),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=86400"),
            ),
        ],
        Body::from(body),
    )
        .into_response()
}
