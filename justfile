set shell := ["sh", "-cu"]

fmt:
  cargo fmt --all
  pnpm fmt

check:
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo run -p langindex-site -- validate-content
  pnpm check

test:
  cargo test --workspace --all-features
  pnpm test

test-smoke:
  pnpm test:smoke

validate-sources:
  pnpm validate:sources

check-links-internal:
  pnpm check:links:internal

check-links-external:
  pnpm check:links:external

build:
  cargo build -p langindex-site --release
  pnpm build

dev:
  pnpm dev

site-dev:
  LANGINDEX_SITE_LOG=info,tower_http=debug cargo run -p langindex-site

site-validate:
  cargo run -p langindex-site -- validate-content
