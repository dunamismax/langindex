# BUILD.md

Active build plan for LangIndex. `README.md` introduces the product.
`AGENTS.md` defines durable operating rules. Keep this file focused on
unfinished work and near-term sequencing.

The initial language seed set, supporting comparisons/guides, and the
Astro-to-Rust rewrite are complete. Do not re-add completed phase checklists
unless that work becomes active again.

---

## Current Baseline

Observed on 2026-05-21:

- Public site: [https://langindex.dev](https://langindex.dev).
- Implementation: Rust workspace with Axum serving Leptos SSR pages from
  source-backed MDX content.
- Verification entry points: `just fmt`, `just check`, `just test`,
  `just build`, `just test-smoke`, `just check-links-internal`, and
  `just check-links-external`.
- Deployment target: single Rust site service behind Caddy on Stephen's
  Ubuntu VM. Do not deploy or change production config unless Stephen asks.
- Content inventory:
  - 34 language profiles.
  - 48 comparisons.
  - 21 guides.
  - 15 concepts.
- Completed architecture work:
  - Rust/Leptos/Axum site crate.
  - Route-compatible pages, feeds, sitemap, robots, search JSON, and
    `languages.json`.
  - Rust-side content parsing and validation.
  - Cargo/just verification.
  - Docker, Compose, Caddy, local-development, and deployment docs.
- Completed concept work:
  - Ownership.
  - Functional Programming.
  - Static vs Dynamic Typing.
  - Strong vs Weak Typing.
  - Type Inference.
  - Structural vs Nominal Typing.
  - Generics and Parametric Polymorphism.
  - Algebraic Data Types and Pattern Matching.
  - Null Safety.
  - Garbage Collection.
  - Reference Counting.
  - Manual Memory Management.
  - RAII and Deterministic Cleanup.
  - Stack vs Heap Allocation.
  - Memory Safety.

---

## Content Objective

LangIndex should expand one durable reference page at a time. Accuracy beats
coverage, but expansion order should favor languages and concepts developers
are likely to search for, compare, or use in production.

Each content phase should leave the site better in four ways:

- A complete, source-backed page.
- Useful cross-links to languages, comparisons, guides, and concepts.
- Improved browse/search metadata where the current model supports it.
- Clear contributor surface for maintainers or community experts to correct
  the page through normal pull requests.

## Completion Standard

A language page is complete when it has:

- Accurate frontmatter for stable metadata: release date, creators or
  organization, status, paradigms, typing, memory model, runtime model,
  package managers, official site, repository, related languages, and
  `lastVerified`.
- A concise above-the-fold summary that explains what the language is for.
- Origin and design goals.
- Current implementation, runtime, compiler, interpreter, or target story.
- Type system, memory model, concurrency model, module/package system, and
  build/deployment model where relevant.
- Best-fit and poor-fit use cases framed as constraints.
- Small idiomatic examples that are runnable when practical.
- Tooling and ecosystem notes.
- Governance, standards, ownership, release cadence, and compatibility
  policy where those apply.
- Official or high-quality source-backed facts, visible source links, and a
  current `lastVerified` date.

A concept page is complete when it:

- Defines the term neutrally and distinguishes it from commonly conflated
  ideas.
- Compares practical variants across languages without crowning winners.
- Explains watch points and production consequences.
- Links to the relevant language and comparison pages.
- Uses official specifications, docs, standards bodies, maintainer material,
  or clearly labeled secondary sources.

## Source Policy

Prefer, in order:

1. Official language specifications, docs, release notes, governance pages,
   package manager docs, and repositories.
2. Standards bodies and foundation pages.
3. Maintainer-authored posts, project RFCs, and proposals.
4. Carefully labeled secondary sources for historical or ecosystem context.

Do not publish popularity, performance, safety, adoption, production-ready,
or maturity claims without a specific source and methodology. If a source
was not checked during the edit, do not imply it was.

## Expansion Order Methodology

There is no single authoritative popularity ranking. Re-check current signals
before each major language batch:

- Developer usage survey signal: Stack Overflow Developer Survey.
- Repository activity signal: GitHub Octoverse or comparable repository
  ecosystem reporting.
- Search/community signal: TIOBE Index, checked monthly when used.
- LangIndex product judgment: prioritize pages that unlock high-value
  comparisons, common production decisions, and broad contributor interest.

The phase order below is editorial sequencing, not a measured global usage
ranking.

## Per-Phase Workflow

For every content phase:

- Research official sources first.
- Update or create source content under `src/content`.
- Add comparison, guide, and concept cross-links where they improve the
  reference graph.
- Refresh `lastVerified` only for pages whose sources were actually checked.
- Run local verification before commit:

```sh
git diff --check
just fmt
just check
just test
just build
```

Use `just check-links-external` when source URLs, rendered source lists, or
link-checking code changed broadly enough to justify the network time.

---

## Active Phases

### Phase 37 - Runtime And Execution Concepts

- [ ] Add `interpreters-jit-and-aot`.
- [ ] Add `virtual-machines-and-bytecode`.
- [ ] Add `compilation-targets`.
- [ ] Add `foreign-function-interface`.
- [ ] Add `abi-stability`.
- [ ] Add `standard-library-philosophy`.
- [ ] Link these concepts from runtime-heavy pages and relevant guides.

### Phase 38 - Concurrency Concepts

- [ ] Add `threads-and-shared-memory`.
- [ ] Add `async-await-and-event-loops`.
- [ ] Add `goroutines-and-green-threads`.
- [ ] Add `actor-model-and-message-passing`.
- [ ] Add `data-races-and-memory-models`.
- [ ] Add `structured-concurrency`.
- [ ] Link these concepts from Go, Rust, Java, C#, Kotlin, Swift,
      JavaScript, Python, Erlang, Elixir, and concurrency guides.

### Phase 39 - Paradigm And Language Design Concepts

- [ ] Add `object-oriented-programming`.
- [ ] Add `immutability-and-persistent-data-structures`.
- [ ] Add `closures-and-first-class-functions`.
- [ ] Add `errors-as-values-vs-exceptions`.
- [ ] Add `metaprogramming-and-macros`.
- [ ] Add `modules-and-namespacing`.
- [ ] Link these concepts from language and comparison pages where the
      paradigm choice is a central tradeoff.

### Phase 40 - Tooling And Ecosystem Concepts

- [ ] Add `package-managers`.
- [ ] Add `build-systems`.
- [ ] Add `formatters-and-linters`.
- [ ] Add `language-servers-and-editor-tooling`.
- [ ] Add `repl-and-interactive-development`.
- [ ] Add `testing-cultures`.
- [ ] Add `documentation-cultures`.
- [ ] Link these tooling concepts from language pages instead of restating
      ecosystem norms inline.

### Phase 41 - Concepts Hub Polish

- [ ] Re-read concept pages for overlap, drift, and missing cross-links.
- [ ] Add topical grouping to `/concepts/`: type systems, memory, runtime,
      concurrency, paradigms, and tooling.
- [ ] Ensure language pages link to concepts for typing, memory, runtime,
      concurrency, and tooling when those concepts exist.
- [ ] Ensure comparison pages link to the concept most central to the
      tradeoff above the fold when practical.
- [ ] Run `just check-links-external` after the final concept batch.
- [ ] Update `README.md` if the Concepts surface has materially changed.

---

## Next Language Candidates

Add these only after the concept backbone is strong enough to avoid repeating
the same explanations inside every language page.

### Phase 42 - PowerShell

- [ ] Add a complete PowerShell page.
- [ ] Cover shell language vs scripting language boundaries, .NET object
      pipelines, modules, remoting, Windows and cross-platform status,
      security policy, package/module ecosystem, and operations fit.
- [ ] Add PowerShell vs Bash/Shell comparison.
- [ ] Update operations and automation guides.

### Phase 43 - MATLAB

- [ ] Add a complete MATLAB page.
- [ ] Cover numerical computing, matrix-first design, Simulink adjacency,
      toolboxes, licensing constraints, deployment/runtime story, and
      research/engineering fit.
- [ ] Add MATLAB vs Python and MATLAB vs Julia comparisons.
- [ ] Update scientific and numerical computing guides.

### Phase 44 - Groovy

- [ ] Add a complete Groovy page.
- [ ] Cover JVM dynamic/static modes, Gradle history, scripting, DSLs,
      Java interop, testing culture, and modern maintenance fit.
- [ ] Add Groovy vs Kotlin and Groovy vs Java comparisons.
- [ ] Update JVM and build-tooling guide material.

### Phase 45 - Assembly Language

- [ ] Add a complete Assembly page with clear scope across ISA families.
- [ ] Cover machine-code proximity, assemblers, ABIs, calling conventions,
      embedded/firmware, reverse engineering, performance myths, and
      maintainability constraints.
- [ ] Add Assembly vs C comparison.
- [ ] Update systems, embedded, and ABI/FFI concept links.

### Phase 46 - COBOL

- [ ] Add a complete COBOL page.
- [ ] Cover business data processing, mainframe environments, standards,
      file and decimal-data models, modernization paths, and maintenance fit.
- [ ] Add COBOL vs Java for enterprise modernization comparison.
- [ ] Update legacy-maintenance guides.

### Phase 47 - Ada

- [ ] Add a complete Ada page.
- [ ] Cover safety-critical systems, strong typing, packages, tasking,
      SPARK adjacency, compiler/tooling ecosystem, and certification
      constraints.
- [ ] Add Ada vs Rust and Ada vs C++ comparisons.
- [ ] Update systems, embedded, and safety-critical guide material.

### Phase 48 - Visual Basic

- [ ] Add a complete Visual Basic page with scope split between classic VB
      and VB.NET where needed.
- [ ] Cover Windows desktop heritage, COM/Office automation, .NET status,
      modernization pressure, and maintenance fit.
- [ ] Add Visual Basic vs C# comparison.
- [ ] Update desktop and legacy-maintenance guides.

---

## Cross-Cutting Backlog

- [ ] Create a documented popularity/adoption methodology page before
      publishing any public ranking or "most popular" page.
- [ ] Add CODEOWNERS only when maintainers or review owners exist.
- [ ] Add a source-quality badge or field only after the editorial standard
      defines levels clearly.
- [ ] Add benchmark policy before publishing performance comparisons.
- [ ] Add compatibility-policy fields if enough language pages expose
      consistent data.
- [ ] Add machine-readable language metadata fields only when a route or UI
      needs them.
- [ ] Periodically re-run external link checks and refresh `lastVerified`
      dates only when sources were actually checked.

## Verification

Docs-only change:

```sh
git diff --check
```

Normal content or UI change:

```sh
just fmt
just check
just test
just build
```

Broader verification:

```sh
just test-smoke
just check-links-internal
just check-links-external
```

Deployment changes should also validate Docker/Compose and Caddy behavior as
described in `docs/deployment.md`.
