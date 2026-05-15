# Local Development

LangIndex uses Astro, TypeScript, MDX content collections, Tailwind CSS, pnpm,
and Pagefind.

## Requirements

- Node.js 22.12 or newer.
- pnpm 10 or newer.
- Docker 29 or newer for production-image checks.

## Setup

```sh
pnpm install
```

## Common Commands

```sh
just fmt
just check
just test
just build
just dev
```

`just test` runs Astro validation plus LangIndex source metadata checks.
`just build` generates the static Astro site and Pagefind search index.

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
