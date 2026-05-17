use std::env;
use std::net::SocketAddr;

use langindex_site::{build_router, default_content_root, load_site_content};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

const DEFAULT_ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    init_tracing();

    let content_root = env::var("LANGINDEX_CONTENT_DIR")
        .map(Into::into)
        .unwrap_or_else(|_| default_content_root());
    let content = load_site_content(&content_root).unwrap_or_else(|err| {
        panic!(
            "failed to load LangIndex content from {}: {err}",
            content_root.display()
        )
    });

    if env::args().nth(1).as_deref() == Some("validate-content") {
        println!(
            "Validated {} languages, {} comparisons, {} guides, and {} concepts.",
            content.languages.len(),
            content.comparisons.len(),
            content.guides.len(),
            content.concepts.len()
        );
        return;
    }

    let addr: SocketAddr = match env::var("LANGINDEX_SITE_ADDR") {
        Ok(value) => value.parse().unwrap_or_else(|err| {
            panic!("LANGINDEX_SITE_ADDR must be a valid socket address: {err}")
        }),
        Err(_) => DEFAULT_ADDR.parse().expect("default address parses"),
    };

    let app = build_router(content);
    let listener = TcpListener::bind(addr)
        .await
        .expect("bind langindex-site listener");

    tracing::info!(%addr, "langindex-site listening");

    axum::serve(listener, app.into_make_service())
        .await
        .expect("langindex-site server stopped unexpectedly");
}

fn init_tracing() {
    let filter =
        EnvFilter::try_from_env("LANGINDEX_SITE_LOG").unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}
