set shell := ["sh", "-cu"]

fmt:
  cargo fmt --all

check:
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo run -p langindex-site -- validate-content

test:
  cargo test --workspace --all-features

test-smoke:
  pnpm test:smoke

validate-sources:
  cargo run -p langindex-site -- validate-content

check-links-internal:
  cargo test -p langindex-site content::tests::route_manifest_covers_public_content
  cargo run -p langindex-site -- validate-content

check-links-external:
  pnpm check:links:external

build:
  cargo build -p langindex-site --release

dev:
  LANGINDEX_SITE_LOG=info,tower_http=debug cargo run -p langindex-site

site-dev:
  LANGINDEX_SITE_LOG=info,tower_http=debug cargo run -p langindex-site

site-validate:
  cargo run -p langindex-site -- validate-content
