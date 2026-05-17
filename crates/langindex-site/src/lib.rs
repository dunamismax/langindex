use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::Router;
use axum::http::{HeaderName, HeaderValue};
use axum::routing::get;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::trace::TraceLayer;

mod assets;
pub mod content;
mod pages;

pub use content::{ContentError, SiteContent};

#[derive(Clone)]
pub struct AppState {
    content: Arc<SiteContent>,
}

pub fn default_content_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("src/content")
}

pub fn load_site_content(content_root: &Path) -> Result<SiteContent, ContentError> {
    SiteContent::load(content_root)
}

pub fn build_router(content: SiteContent) -> Router {
    let state = AppState {
        content: Arc::new(content),
    };

    Router::new()
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/contribute", get(pages::contribute))
        .route("/languages", get(pages::languages_index))
        .route("/languages/", get(pages::languages_index))
        .route("/languages/{slug}", get(pages::language_detail))
        .route("/languages/{slug}/", get(pages::language_detail))
        .route("/comparisons", get(pages::comparisons_index))
        .route("/comparisons/", get(pages::comparisons_index))
        .route("/comparisons/{slug}", get(pages::comparison_detail))
        .route("/comparisons/{slug}/", get(pages::comparison_detail))
        .route("/guides", get(pages::guides_index))
        .route("/guides/", get(pages::guides_index))
        .route("/guides/{slug}", get(pages::guide_detail))
        .route("/guides/{slug}/", get(pages::guide_detail))
        .route("/concepts", get(pages::concepts_index))
        .route("/concepts/", get(pages::concepts_index))
        .route("/concepts/{slug}", get(pages::concept_detail))
        .route("/concepts/{slug}/", get(pages::concept_detail))
        .route("/languages.json", get(pages::languages_json))
        .route("/rss.xml", get(pages::rss))
        .route("/sitemap.xml", get(pages::sitemap))
        .route("/robots.txt", get(pages::robots))
        .route("/healthz", get(pages::healthz))
        .route("/assets/style.css", get(assets::style_css))
        .route("/favicon.png", get(assets::favicon_png))
        .fallback(pages::not_found)
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(SetResponseHeaderLayer::if_not_present(
                    HeaderName::from_static("x-content-type-options"),
                    HeaderValue::from_static("nosniff"),
                ))
                .layer(SetResponseHeaderLayer::if_not_present(
                    HeaderName::from_static("referrer-policy"),
                    HeaderValue::from_static("strict-origin-when-cross-origin"),
                )),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    async fn get(path: &str) -> (StatusCode, String) {
        let content = load_site_content(&default_content_root()).expect("content loads");
        get_with_router(build_router(content), path).await
    }

    async fn get_with_router(app: Router, path: &str) -> (StatusCode, String) {
        let response = app
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("router call");
        let status = response.status();
        let bytes = response
            .into_body()
            .collect()
            .await
            .expect("collect body")
            .to_bytes();
        (status, String::from_utf8_lossy(&bytes).into_owned())
    }

    #[tokio::test]
    async fn public_routes_render() {
        for path in [
            "/",
            "/about",
            "/contribute",
            "/languages/",
            "/languages/rust",
            "/comparisons/",
            "/comparisons/rust-vs-go",
            "/guides/",
            "/guides/choosing-a-systems-language",
            "/concepts/",
            "/concepts/ownership",
        ] {
            let (status, body) = get(path).await;
            assert_eq!(status, StatusCode::OK, "{path}");
            assert!(body.contains("LangIndex"), "{path}");
        }
    }

    #[tokio::test]
    async fn detail_pages_preserve_sources_and_verified_dates() {
        let (status, body) = get("/languages/rust/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("Rust Programming Language"));
        assert!(body.contains("Last verified"));
        assert!(body.contains("2026-05-15"));

        let (status, body) = get("/comparisons/rust-vs-go/").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("Sources"));
        assert!(body.contains("Rust vs Go"));
    }

    #[tokio::test]
    async fn every_content_detail_route_renders() {
        let content = load_site_content(&default_content_root()).expect("content loads");
        let routes: Vec<_> = content
            .languages
            .iter()
            .map(|page| format!("/languages/{}/", page.data.slug))
            .chain(
                content
                    .comparisons
                    .iter()
                    .map(|page| format!("/comparisons/{}/", page.data.slug)),
            )
            .chain(
                content
                    .guides
                    .iter()
                    .map(|page| format!("/guides/{}/", page.data.slug)),
            )
            .chain(
                content
                    .concepts
                    .iter()
                    .map(|page| format!("/concepts/{}/", page.data.slug)),
            )
            .collect();
        let app = build_router(content);

        for route in routes {
            let (status, body) = get_with_router(app.clone(), &route).await;
            assert_eq!(status, StatusCode::OK, "{route}");
            assert!(body.contains("Sources"), "{route}");
            assert!(body.contains("Last verified"), "{route}");
        }
    }

    #[tokio::test]
    async fn generated_outputs_render() {
        let (status, body) = get("/languages.json").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("\"slug\":\"rust\""));

        let (status, body) = get("/rss.xml").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("<rss"));
        assert!(body.contains("LangIndex"));

        let (status, body) = get("/sitemap.xml").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("/languages/rust"));

        let (status, body) = get("/robots.txt").await;
        assert_eq!(status, StatusCode::OK);
        assert!(body.contains("Sitemap: https://langindex.dev/sitemap.xml"));
    }

    #[tokio::test]
    async fn fallback_and_healthz_work() {
        let (status, body) = get("/healthz").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "ok");

        let (status, body) = get("/missing").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(body.contains("Page not found"));
    }

    #[tokio::test]
    async fn stylesheet_has_expected_content_type() {
        let content = load_site_content(&default_content_root()).expect("content loads");
        let response = build_router(content)
            .oneshot(
                Request::builder()
                    .uri("/assets/style.css")
                    .body(Body::empty())
                    .expect("request"),
            )
            .await
            .expect("router call");
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/css; charset=utf-8"
        );
    }
}
