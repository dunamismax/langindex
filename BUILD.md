# BUILD.md

Active build plan for LangIndex. `README.md` introduces the product.
This file tracks stack choices, content model, phases, deployment, and
verification while the site is built.

Treat unchecked boxes as plan. Move stable material into `docs/`,
`README.md`, or runbooks as the implementation matures.

---

## What's Live Now

- Repository exists at `dunamismax/langindex`.
- `origin` fetches from GitHub and pushes to GitHub plus Codeberg.
- Public production domain is [https://langindex.dev](https://langindex.dev).
- Initial `README.md` and `CONTRIBUTING.md` describe the product and
  contribution standard.
- Project code is licensed under GPL-3.0. Content and data are licensed
  under CC-BY-SA-4.0 unless noted otherwise.
- Repo-local `AGENTS.md` and this `BUILD.md` define the initial build
  direction.
- Astro 6 static skeleton exists with Tailwind CSS 4, MDX, typed content
  collections, seed content, and CI.

Current local baseline observed on 2026-05-15:

```text
Node.js: v26.0.0
pnpm: 10.32.1
```

---

## Stack Decisions

- **Astro** for the framework.
- **TypeScript** for app code, content helpers, and validation.
- **MDX** for long-form language, comparison, guide, and concept pages.
- **Astro Content Collections** for typed content loading.
- **Zod schemas** through content collections for structured frontmatter.
- **Pagefind** for static search.
- **Tailwind CSS** for styling.
- **pnpm** for package management.
- **justfile** as the primary command runner once the skeleton exists.
- **Docker Compose** for production build/deploy wiring.
- **Caddy** on Stephen's Ubuntu VM for TLS and static file serving.

No database in v1. Add PostgreSQL only when dynamic product features
earn it: accounts, review queues, moderation, private drafts, analytics,
or authenticated maintainer workflows.

---

## Product Invariants

- LangIndex is a field guide and reference for programming languages.
- Accuracy beats coverage.
- Git is the source of truth for content, review history, and
  contribution workflow.
- Public pages must be useful without login, tracking, or client-heavy
  application code.
- Every language page should be easy for its community to correct.
- Facts should be source-backed. Opinions should be clearly framed as
  tradeoffs.
- Comparisons are dimensional, not tribal.
- Search and browse are primary workflows.
- The site must be self-hostable from a clean checkout.

---

## Editorial Invariants

- Prefer official sources.
- Do not invent citations or imply sources were checked when they were
  not.
- Mark unknowns and disputed details directly.
- Keep examples idiomatic and minimal.
- Avoid benchmark claims until there is a documented benchmark policy.
- Avoid adoption rankings until there is a documented methodology.
- Use current-state wording instead of historical narration unless
  history is the point.
- Every substantial page should include a visible `lastVerified` date.

---

## Content Model

The content model should be strict enough for reliable browsing,
filtering, comparison, and search.

Recommended collections:

```text
languages     -- individual language profiles
comparisons   -- language-to-language or family comparisons
guides        -- use-case and decision guides
concepts      -- glossary/reference pages for cross-language ideas
```

Initial language frontmatter shape:

```yaml
title: Rust
slug: rust
status: active
firstReleased: 2015
creators:
  - Graydon Hoare
paradigms:
  - systems
  - multi-paradigm
typing:
  discipline: static
  strength: strong
memory:
  model: ownership
runtime:
  model: native
officialSite: https://www.rust-lang.org/
repository: https://github.com/rust-lang/rust
packageManagers:
  - Cargo
comparedWith:
  - c
  - cpp
  - zig
lastVerified: 2026-05-15
```

Expect the schema to evolve. Prefer migration scripts or focused PRs over
ad hoc drift once real content exists.

---

## Target Source Layout

```text
src/
  content/
    languages/
    comparisons/
    guides/
    concepts/
  components/
    language/
    search/
    layout/
  layouts/
  pages/
  styles/
  lib/
public/
docs/
  content-model.md
  editorial-standard.md
  deployment.md
  sources.md
deploy/
  caddy/
  docker/
tests/
  playwright/
astro.config.mjs
package.json
pnpm-lock.yaml
tailwind.config.*
tsconfig.json
justfile
Dockerfile
compose.yaml
```

---

## Target Routes

Core public routes:

```text
GET /                         searchable language discovery
GET /languages                full language index
GET /languages/{slug}         language profile
GET /comparisons              comparison index
GET /comparisons/{slug}       comparison page
GET /guides                   guide index
GET /guides/{slug}            decision guide
GET /concepts                 concept/glossary index
GET /concepts/{slug}          concept page
GET /about                    project purpose and governance
GET /contribute               contributor entrypoint
```

Machine-readable routes to consider:

```text
GET /languages.json           public language metadata
GET /search/                  Pagefind assets
GET /sitemap-index.xml
GET /rss.xml                  project updates, optional
```

---

## Page Requirements

Language page:

- Clear summary of what the language is for.
- Origin and design goals.
- Best-fit use cases.
- Poor-fit or risky use cases.
- Highlights and tradeoffs.
- Syntax examples.
- Tooling and ecosystem notes.
- Governance and implementation notes where relevant.
- Compared-with links.
- Official resources.
- Sources and last verified date.

Comparison page:

- Scope: which languages and which use cases are being compared.
- Shared territory.
- Key differences.
- Tradeoff table.
- Migration or interoperability notes where relevant.
- "Choose X when..." and "Choose Y when..." guidance.
- Sources and last verified date.

Guide page:

- Concrete developer problem.
- Evaluation criteria.
- Candidate languages.
- Tradeoffs and constraints.
- Links into language and comparison pages.

---

## Design Direction

- Content-forward, fast, and quiet.
- Reference-site density without clutter.
- Strong typography and scanning hierarchy.
- No marketing-first landing page.
- Homepage should expose real browsing/search value immediately.
- Language cards should be compact and comparable.
- Source quality and last verification should be visible.
- Use color sparingly for categories, status, and affordances.
- Avoid a one-note palette.

---

## Deployment Target

Production target:

```text
Ubuntu VM
Docker Compose
Caddy
langindex.dev
```

Expected production shape:

- Build static site in a container or CI job.
- Serve generated `dist/` through Caddy.
- Caddy owns TLS for `langindex.dev`.
- No production secrets required for v1.
- Deployment should be reproducible from repo docs.

Do not deploy or change production Caddy/DNS without explicit Stephen
approval.

---

## Phases

Ordered intent, not rigid sequence. Each phase should leave the repo in a
state where documented verification passes on a clean checkout.

### Phase 0 - Project Foundation

- [x] Choose project name and domain.
- [x] Configure public domain in Cloudflare.
- [x] Create GitHub and Codeberg repositories.
- [x] Clone repository into `/Users/sawyer/github/langindex`.
- [x] Normalize dual-push remote configuration.
- [x] Write `README.md`.
- [x] Write `CONTRIBUTING.md`.
- [x] Write repo-local `AGENTS.md`.
- [x] Write this `BUILD.md`.
- [x] Decide code license.
- [x] Decide content/data license.
- [x] Add `.gitignore`.
- [x] Add `.editorconfig`.

### Phase 1 - Astro Skeleton

- [x] Initialize Astro with TypeScript.
- [x] Add Tailwind CSS.
- [x] Add MDX integration.
- [x] Add Astro Content Collections.
- [x] Add initial Zod schemas for `languages`, `comparisons`, `guides`,
      and `concepts`.
- [x] Add `justfile` with `fmt`, `check`, `test`, `build`, and `dev`.
- [x] Add basic layout, typography, and navigation.
- [x] Add homepage, language index, language detail route, and about page.
- [x] Add CI for install, type-check, and build.

### Phase 2 - Content Model And Seed Pages

- [x] Document the content model in `docs/content-model.md`.
- [x] Document the editorial standard in `docs/editorial-standard.md`.
- [x] Add first language pages with verified sources.
- [x] Add first comparison pages.
- [x] Add first guide page.
- [x] Validate all content through collection schemas.
- [x] Add visible source and `lastVerified` sections to page templates.

Suggested seed languages:

- C
- C++
- Rust
- Zig
- Go
- Java
- Kotlin
- C#
- JavaScript
- TypeScript
- Python
- Ruby
- PHP
- Swift
- Elixir
- Erlang
- Haskell
- OCaml
- Clojure
- Lua
- Julia
- R
- Nim
- Crystal
- Dart
- SQL

### Phase 3 - Search And Discovery

- [x] Add Pagefind indexing after Astro build.
- [x] Add search UI with keyboard-friendly behavior.
- [x] Add filters for paradigm, typing, runtime target, ecosystem, and
      use case.
- [x] Add language family and related-language browsing.
- [x] Add comparison discovery from language pages.
- [x] Add generated metadata pages or JSON only if useful.

### Phase 4 - Contributor Workflow

- [x] Add issue templates.
- [x] Add pull request template.
- [x] Add language page template.
- [x] Add comparison page template.
- [x] Add source/citation guidance.
- [ ] Add CODEOWNERS only when maintainers or review owners exist.
- [x] Add link checking or source validation checks.
- [x] Add docs for local development.

### Phase 5 - Self-Hosted Deployment

- [x] Add `Dockerfile`.
- [x] Add `compose.yaml`.
- [x] Add Caddy config under `deploy/caddy/`.
- [x] Add `docs/deployment.md`.
- [x] Build production image locally.
- [x] Verify container serves static build.
- [x] Validate Caddy config.
- [ ] Deploy to Ubuntu VM after Stephen approves.
- [ ] Confirm [https://langindex.dev](https://langindex.dev) serves the
      static site.

### Phase 6 - Hardening

- [ ] Add Playwright smoke tests for homepage, search, language page, and
      comparison page.
- [ ] Add internal link checking.
- [ ] Add external link checking with rate limits.
- [ ] Add accessibility checks.
- [ ] Add sitemap.
- [ ] Add robots.txt.
- [ ] Add RSS or changelog feed if useful.
- [ ] Add backup/restore note for repo and deployment artifacts.

---

## Verification

Narrowest useful command first, then broaden.

Docs-only work:

```sh
git diff --check
```

Once the skeleton exists:

```sh
just fmt
just check
just test
just build
```

Expected checks as the site matures:

- TypeScript type checking.
- Astro build.
- Astro content collection validation.
- MDX linting.
- Pagefind index generation.
- Internal link checking.
- External source link checking when relevant.
- Playwright smoke tests.
- Docker build.
- Caddy validation for deployment changes.

If a command cannot run, report why and what was verified instead.

---

## External Sources To Re-check

Trust current primary docs over this file when implementation details may
have changed.

- Astro docs for project setup, MDX, content collections, adapters, and
  build behavior.
- Tailwind CSS docs for current Astro integration.
- Pagefind docs for current indexing and UI integration.
- Caddy docs for static file serving and config validation.
- Docker docs for Compose behavior on Ubuntu.
- GitHub and Codeberg docs for contribution templates and repository
  mirroring behavior.

---

## Recent Work

- 2026-05-15 - Created initial project documentation, normalized dual
  GitHub/Codeberg push configuration, and defined the Astro / TypeScript /
  MDX / Content Collections / Pagefind / Tailwind / Docker Compose /
  Caddy stack for LangIndex.
- 2026-05-15 - Built the initial Astro static-site skeleton with
  Tailwind CSS 4, MDX, strict content collections, seed Rust/Go/TypeScript
  pages, Rust-vs-Go comparison, systems-language guide, ownership concept,
  justfile, CI, and passing local checks.
- 2026-05-15 - Added Pagefind indexing and search UI, filterable language
  discovery, related-language and comparison links on language pages, and
  `/languages.json` metadata output.
- 2026-05-15 - Added CC-BY-SA-4.0 content licensing, contributor issue and
  pull request templates, source validation, content templates, local
  development docs, and Docker/Caddy deployment wiring.
- 2026-05-15 - Built the local production image, validated the Caddy config
  with the Caddy container, and verified the static site from the local
  LangIndex container.
