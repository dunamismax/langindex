# Phase 16.5 Rewrite Inventory

Observed locally during the Rust rewrite setup on 2026-05-17.

## Current Route Set

Astro currently defines these public routes:

- `/`
- `/about`
- `/contribute`
- `/languages/`
- `/languages/{slug}`
- `/comparisons/`
- `/comparisons/{slug}`
- `/guides/`
- `/guides/{slug}`
- `/concepts/`
- `/concepts/{slug}`
- `/languages.json`
- `/rss.xml`
- `/sitemap.xml`
- `/robots.txt`

The Rust scaffold now registers the same route set, with both trailing-slash
and no-trailing-slash variants for collection indexes and detail pages.

## Content And Schemas

Astro Content Collections live in `src/content.config.ts` and validate four
groups:

- `languages`
- `comparisons`
- `guides`
- `concepts`

The source files remain in `src/content/**`. Frontmatter contains stable
metadata such as `slug`, `summary`, language relationships, `sources`, and
`lastVerified`; body content remains prose, examples, and cited discussion in
Markdown/MDX.

The Rust scaffold parses the same frontmatter and body files into typed Rust
structs. It validates malformed frontmatter, missing required metadata,
duplicate slugs, filename/slug mismatches, future `lastVerified` dates,
missing or invalid source URLs, unknown language slugs, and broken internal
Markdown links.

## Derived Outputs

Existing generated outputs are:

- `/languages.json` from `src/pages/languages.json.ts`
- `/rss.xml` from `src/pages/rss.xml.ts`
- `/sitemap.xml` from `src/pages/sitemap.xml.ts`
- `/robots.txt` from `src/pages/robots.txt.ts`
- Pagefind index output under `dist/pagefind` after `astro build`

The Rust scaffold provides initial Rust handlers for `/languages.json`,
`/rss.xml`, `/sitemap.xml`, and `/robots.txt`. Pagefind remains a temporary
Node/Astro build dependency until the search replacement is finished.

## Search Behavior

The current site uses Pagefind through `src/components/search/PagefindSearch.astro`.
The homepage lazy-loads Pagefind UI assets from `/pagefind`, and the production
build runs `pagefind --site dist`. Language and comparison pages include
Pagefind body/meta/filter attributes.

The Rust scaffold preserves a search form and keeps Pagefind documented as
temporary migration tooling. A Rust-native or controlled static search index is
still required before the search checkbox can be considered done.

## Existing Validation And Checks

Current Node checks:

- `astro check`
- `scripts/validate-sources.mjs`
- `scripts/check-internal-links.mjs`
- `scripts/check-external-links.mjs`
- Playwright smoke and axe accessibility tests under `tests/playwright`

The Rust scaffold adds:

- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo run -p langindex-site -- validate-content`

Temporary Astro checks remain in the `justfile` during migration.

## Docker, Compose, And Caddy

Current deployment files are:

- `Dockerfile`
- `compose.yaml`
- `deploy/caddy/Caddyfile`
- `deploy/caddy/langindex.host.caddy`
- `docs/deployment.md`
- `docs/local-development.md`

They still describe and build the Astro static site served by Caddy. The Rust
service deployment work remains incomplete and must not be checked off until
Docker/Compose/Caddy docs and health checks are updated and verified.

## Browser Coverage

Current Playwright coverage includes:

- Homepage search and language discovery
- Mobile navigation
- Language filters
- Representative language and comparison detail pages
- Narrow viewport overflow checks
- Axe checks for homepage, indexes, and representative detail pages

These tests still target the Astro server through `playwright.config.ts`. Axum
browser coverage remains incomplete until Playwright or an equivalent browser
smoke suite runs against the Rust service.
