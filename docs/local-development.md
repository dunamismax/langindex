# Local Development

LangIndex uses Astro, TypeScript, MDX content collections, Tailwind CSS, pnpm,
and Pagefind.

## Requirements

- Node.js 22.12 or newer (CI builds with Node 26).
- pnpm 10 or newer (the repo pins `pnpm@10.32.1`).
- Docker 29 or newer for production-image checks.

## Setup

```sh
pnpm install
```

## Common Commands

```sh
just fmt                    # prettier --write .
just check                  # astro check (TypeScript + content schemas)
just test                   # astro check + scripts/validate-sources.mjs
just build                  # astro build + pagefind index into dist/
just dev                    # astro dev (default http://localhost:4321)
just test-smoke             # playwright smoke + accessibility tests
just validate-sources       # frontmatter source URL sanity checks
just check-links-internal   # internal link audit against the built site
just check-links-external   # external link audit (network, slower)
```

`just test` runs Astro validation plus LangIndex source metadata checks.
`just build` generates the static Astro site and the Pagefind search index
into `dist/`. The Playwright suite expects a built site; run `just build`
before `just test-smoke` if you have not already.

Use `just check-links-external` when source changes are broad enough to
justify the network time — official documentation URLs are the most common
source of late-stage breakage.

## Content Workflow

1. Copy a template from `docs/templates/`.
2. Put language pages in `src/content/languages/`.
3. Put comparisons in `src/content/comparisons/`.
4. Verify all frontmatter fields against `docs/content-model.md`.
5. Check factual claims against `docs/sources.md`.
6. Run `just test` and `just build`.

## Docker Check

Build and run the production image locally:

```sh
docker build -t langindex:local .
docker run --rm -p 8080:80 -e SITE_ADDRESS=:80 langindex:local
```

Open `http://localhost:8080` to inspect the static site.
