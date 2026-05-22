# Local Development

LangIndex runs as a Rust workspace with an Axum site service and Leptos
server-rendered HTML. Git-authored Markdown/MDX content lives in
`src/content/**` and is parsed and validated by Rust before the service starts.

## Requirements

- Rust 1.95.0, pinned by `rust-toolchain.toml`.
- Cargo, provided by the pinned Rust toolchain.
- Node.js 22.12 or newer and pnpm 10.32.1 only for Playwright smoke tests and
  the external link checker.
- Docker 29 or newer for image and Compose checks.

## Setup

```sh
cargo fetch
pnpm install
```

`pnpm install` is only needed when running `just test-smoke` or
`just check-links-external`.

## Common Commands

```sh
just fmt                    # cargo fmt --all
just check                  # Rust fmt check, clippy, content validation
just test                   # Rust unit and route tests
just build                  # release build for langindex-site
just dev                    # local Axum server at http://127.0.0.1:3000
just site-validate          # parse and validate all content
just test-smoke             # Playwright smoke/accessibility against Axum
just check-links-internal   # route manifest plus Rust internal link checks
just check-links-external   # slower network check for external URLs
```

## Content Workflow

1. Copy a template from `docs/templates/`.
2. Put language pages in `src/content/languages/`.
3. Put comparisons in `src/content/comparisons/`.
4. Verify frontmatter fields against `docs/content-model.md`.
5. Check factual claims against `docs/sources.md`.
6. Run `just check`, `just test`, and `just build`.

## Local Server

```sh
just dev
```

The service listens on `127.0.0.1:3000` by default. Override it when needed:

```sh
LANGINDEX_SITE_ADDR=127.0.0.1:4321 cargo run -p langindex-site
```

The content directory defaults to `src/content` from the repository checkout.
Override it for packaged or deployment layouts:

```sh
LANGINDEX_CONTENT_DIR=/path/to/content cargo run -p langindex-site
```

## Docker Check

Build and run the production image locally:

```sh
docker build -t langindex:local .
docker run --rm -p 8080:3000 langindex:local
```

Then verify:

```sh
curl -fsS http://127.0.0.1:8080/healthz
curl -I http://127.0.0.1:8080/languages/
```
