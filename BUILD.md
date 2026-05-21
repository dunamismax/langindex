# BUILD.md

Active build plan for LangIndex. `README.md` introduces the product.
`AGENTS.md` defines durable operating rules. This file now tracks the next
stage: first rewriting the implementation from Astro/TypeScript to
Rust/Leptos/Axum, then continuing the source-backed programming language
reference expansion.

The previous foundation phases are complete. Do not re-add old scaffold
work here unless it becomes active again. Move stable technical material
into `docs/`; keep this file focused on the active rewrite and content
expansion sequence.

---

## Current Baseline

Observed on 2026-05-16:

- Public site is live at [https://langindex.dev](https://langindex.dev).
- Astro 6 static site, TypeScript, MDX, Astro Content Collections, Zod,
  Tailwind CSS 4, Pagefind, Playwright, Docker, Compose, and Caddy wiring
  exist.
- Stephen wants the site rewritten to his Rust web stack before the next
  language phase.
- FileFerry at `/Users/sawyer/github/fileferry` is the local reference
  implementation for the target shape: Cargo workspace, Rust 2024,
  Axum 0.8, Leptos 0.8 SSR, Tokio, `tower-http`, route-level tests, embedded
  assets, `just` workflows, and a single site binary behind Caddy.
- `origin` fetches from GitHub and pushes to GitHub plus Codeberg.
- Current language pages:
  - C
  - C#
  - JavaScript
  - Rust
  - Go
  - TypeScript
  - Python
  - Java
  - SQL
  - PHP
  - Kotlin
  - Swift
  - Ruby
  - R
  - Bash / Shell
  - Dart
  - Lua
  - Scala
  - Perl
  - Objective-C
  - Fortran
  - Delphi / Object Pascal
  - Haskell
  - Elixir
  - Erlang
  - Clojure
  - Julia
  - Zig
  - OCaml
- Current supporting content:
  - Rust vs Go comparison
  - Rust vs C comparison
  - C vs C++ comparison
  - SQL vs Python for Data Work comparison
  - Python vs Go comparison
  - C# vs TypeScript comparison
  - Java vs C# comparison
  - Java vs Go comparison
  - Java vs Kotlin comparison
  - PHP vs JavaScript/Node.js comparison
  - PHP vs Ruby comparison
  - Kotlin vs Swift comparison
  - Swift vs Objective-C comparison
  - Python vs Ruby comparison
  - R vs Python for Data Analysis comparison
  - Shell vs Python for Scripts comparison
  - Dart vs TypeScript comparison
  - Dart vs Kotlin comparison
  - Lua vs JavaScript comparison
  - Lua vs Python comparison
  - Scala vs Java comparison
  - Scala vs Kotlin comparison
  - Perl vs Python comparison
  - Perl vs Shell comparison
  - Fortran vs C++ comparison
  - Fortran vs Python for Numerics comparison
  - Delphi vs C# comparison
  - Haskell vs OCaml comparison
  - Haskell vs Scala comparison
  - Elixir vs Erlang comparison
  - Elixir vs Ruby comparison
  - Clojure vs Java comparison
  - Clojure vs Scala comparison
  - Julia vs Python comparison
  - Julia vs R comparison
  - Zig vs C comparison
  - Rust vs Zig comparison
  - OCaml vs Rust comparison
  - Choosing C# for .NET, desktop, games, and enterprise work guide
  - Choosing a systems language guide
  - Choosing an embedded language guide
  - Choosing Python for scripting, backend, data, and AI work guide
  - Choosing Java for backend, enterprise, and JVM work guide
  - Choosing SQL for data access and application architecture guide
  - Choosing PHP for web backends and CMS work guide
  - Choosing Kotlin for Android, JVM, and multiplatform work guide
  - Choosing Swift for Apple-platform and mobile work guide
  - Choosing R for data analysis, statistics, and research guide
  - Choosing Shell for operations and automation guide
  - Choosing an embedded scripting language guide
  - Choosing a legacy maintenance language guide
  - Choosing a functional programming language guide
  - Choosing a concurrency-oriented backend language guide
  - Choosing a distributed systems language guide
  - Choosing a Lisp-family language guide
  - Ownership concept page
  - Functional programming concept page
- Current local baseline:

```text
Node.js: v26.0.0
pnpm: 10.32.1
rustc: 1.95.0
cargo: 1.95.0
```

---

## Content Objective

LangIndex should expand one language at a time, with each language page
treated as a durable reference rather than a stub. Accuracy beats coverage,
but expansion order should still favor languages developers are most likely
to search for, compare, or use in production.

Phase 16.5 pauses new language expansion until the Rust rewrite preserves the
current public reference experience and verification surface. Do not begin
Phase 17 until Phase 16.5 is complete.

Each language phase should leave the site better in four ways:

- A complete, source-backed language profile.
- At least one useful comparison or comparison update.
- Improved browse/search metadata for that language's ecosystem.
- Clear contributor surface for maintainers or community experts to correct
  the page.

---

## Completion Standard

A language page is not considered complete until it has:

- Accurate frontmatter for stable metadata: release date, creators or
  organization, status, paradigms, typing, memory model, runtime model,
  package managers, official site, repository, related languages, and
  `lastVerified`.
- A concise above-the-fold summary that explains what the language is for.
- Origin and design goals.
- Current implementation, runtime, compiler, interpreter, or target story.
- Type system, memory model, concurrency model, module/package system, and
  build/deployment model where relevant.
- Best-fit use cases and poor-fit/risky use cases, framed as constraints.
- Practical idiomatic examples that are small and runnable when possible.
- Tooling and ecosystem notes, including standard package/build/test tools.
- Governance, standards, ownership, release cadence, and compatibility
  policy where those concepts apply.
- Source-backed facts from official sources first, then high-quality
  secondary sources only when official sources do not cover the claim.
- Comparisons with nearby languages.
- Visible sources and a current `lastVerified` date.

For every phase, update comparison, guide, concept, and navigation content
when the language exposes gaps in the surrounding reference graph.

---

## Source Policy

Prefer, in order:

1. Official language specifications, docs, release notes, governance pages,
   package manager docs, and repositories.
2. Standards bodies and foundation pages.
3. Maintainer-authored posts or project RFCs/proposals.
4. Carefully labeled secondary sources for historical or ecosystem context.

Do not publish popularity, performance, safety, adoption, production-ready,
or maturity claims without a specific source and methodology. If a source
was not checked during the edit, do not imply it was.

---

## Expansion Order Methodology

There is no single authoritative popularity ranking. Use a composite order
and re-check it before each major batch:

- Developer usage survey signal: Stack Overflow Developer Survey 2025
  programming/scripting/markup language usage.
- Repository activity signal: GitHub Octoverse 2025 language usage and new
  repository activity.
- Search/community signal: TIOBE Index, checked monthly.
- LangIndex product judgment: prioritize languages that unlock high-value
  comparisons, common production decisions, and broad contributor interest.

Sources checked for this planning rewrite:

- Stack Overflow Developer Survey 2025:
  <https://survey.stackoverflow.co/2025/technology/>
- GitHub Octoverse 2025:
  <https://github.blog/news-insights/octoverse/octoverse-a-new-developer-joins-github-every-second-as-ai-leads-typescript-to-1/>
- TIOBE Index, May 2026:
  <https://www.tiobe.com/tiobe-index/>

The phase order below is therefore an editorial expansion order, not a
claim that LangIndex has measured exact global language usage.

---

## Per-Language Workflow

For each language phase:

- Research official sources first.
- Update or create the language content file in the active content source
  tree. Before Phase 16.5 completes this is
  `src/content/languages/{slug}.mdx`; after the rewrite, use the Rust-backed
  content location documented by that phase.
- Add or update at least one comparison when the language has an obvious
  adjacent choice.
- Add or update concepts when a recurring idea needs explanation.
- Add or update guides when a practical developer decision emerges.
- Run local verification before commit:

```sh
git diff --check
just fmt
just check
just test
just build
```

Use `just check-links-external` when source changes are broad enough to
justify the extra network time.

---

## Phases

### Phase 1 - TypeScript Completion

- [x] Expand the TypeScript page to the completion standard.
- [x] Cover type erasure, structural typing, gradual adoption, compiler
      options, JavaScript runtime boundaries, declaration files, package
      manager reality, and modern framework usage.
- [x] Add or expand TypeScript vs JavaScript comparison.
- [x] Add comparison links for JavaScript, Dart, Kotlin, and C# where
      useful.
- [x] Add a guide section for when teams should choose JavaScript,
      TypeScript, or another compile-to-JavaScript language.

### Phase 2 - Go Completion

- [x] Expand the Go page to the completion standard.
- [x] Cover design goals, goroutines, channels, garbage collection, standard
      library strength, modules, formatting, testing, release policy, and
      toolchain compatibility.
- [x] Expand Rust vs Go with more practical tradeoff dimensions.
- [x] Add or update comparisons for Go vs Java, Go vs Python, and Go vs C#
      as those pages land; Java, Python, and C# now exist.
- [x] Update systems and services guides with Go-specific constraints.

### Phase 3 - Rust Completion

- [x] Expand the Rust page to the completion standard.
- [x] Cover ownership and borrowing, lifetimes, traits, `unsafe`, editions,
      Cargo, crates.io, rustup, embedded/WebAssembly targets, governance,
      and compatibility policy.
- [x] Expand Rust vs Go and prepare Rust vs C, Rust vs C++, and Rust vs Zig.
- [x] Expand the ownership concept page so it supports Rust and future
      systems-language pages.
- [x] Add systems-language guide material for Rust adoption constraints.

### Phase 4 - Python

- [x] Add a complete Python page.
- [x] Cover CPython, Python Enhancement Proposals, package tooling, virtual
      environments, typing, packaging friction, data/AI ecosystem, scripting,
      web services, and deployment constraints.
- [x] Add Python vs JavaScript or Python vs Go comparison.
- [x] Add Python to relevant guides for scripting, backend services, data,
      and AI-adjacent work.

### Phase 5 - JavaScript

- [x] Add a complete JavaScript page.
- [x] Cover ECMAScript, browser runtimes, Node.js and other server runtimes,
      npm ecosystem, prototype model, async model, module systems, and web
      platform coupling.
- [x] Add JavaScript vs TypeScript comparison.
- [x] Add JavaScript to web and scripting guides.

### Phase 6 - Java

- [x] Add a complete Java page.
- [x] Cover JVM, JDK, JEPs, bytecode, garbage collection, generics,
      concurrency, Maven/Gradle, long-term support releases, enterprise
      ecosystem, Android history, and governance through the JCP/OpenJDK.
- [x] Add Java vs C#, Java vs Go, and Java vs Kotlin comparisons.
- [x] Add Java to backend, enterprise, and JVM ecosystem guides.

### Phase 7 - C#

- [x] Add a complete C# page.
- [x] Cover .NET, CLR, Roslyn, NuGet, async/await, LINQ, generics, memory
      management, cross-platform status, game development with Unity, and
      Microsoft stewardship.
- [x] Add C# vs Java and C# vs TypeScript comparisons.
- [x] Add C# to backend, desktop, game, and enterprise guides.

### Phase 8 - C

- [x] Add a complete C page.
- [x] Cover ISO C, compilers, undefined behavior, manual memory management,
      ABI stability, systems programming, embedded use, package/build
      fragmentation, and interoperability.
- [x] Add C vs Rust and C vs C++ comparisons.
- [x] Add C to systems and embedded guides.

### Phase 9 - C++

- [x] Add a complete C++ page.
- [x] Cover ISO standardization, major compilers, templates, RAII, value
      semantics, memory safety tradeoffs, package/build ecosystem, ABI
      issues, performance-critical domains, and modern C++ evolution.
- [x] Add C++ vs Rust, C++ vs C, and C++ vs Java comparisons.
- [x] Add C++ to systems, games, embedded, and performance guides.

### Phase 10 - SQL

- [x] Add a complete SQL page.
- [x] Treat SQL as a language family with standard SQL plus major dialects.
- [x] Cover relational model basics, query semantics, procedural extensions,
      portability limits, database-specific dialects, transactions, and
      tooling.
- [x] Add SQL to data, backend, and application architecture guides.

### Phase 11 - PHP

- [x] Add a complete PHP page.
- [x] Cover PHP runtime, Composer, Packagist, web hosting history, modern
      framework ecosystem, type-system evolution, deployment model, and
      common production constraints.
- [x] Add PHP vs JavaScript/Node.js and PHP vs Ruby comparisons.
- [x] Add PHP to web backend guides.

### Phase 12 - Kotlin

- [x] Add a complete Kotlin page.
- [x] Cover JVM, Android, Kotlin Multiplatform, null safety, coroutines,
      Gradle, interoperability with Java, and JetBrains stewardship.
- [x] Add Kotlin vs Java and Kotlin vs Swift comparisons.
- [x] Add Kotlin to JVM, Android, and multiplatform guides.

### Phase 13 - Swift

- [x] Add a complete Swift page.
- [x] Cover Apple platform role, Swift Evolution, memory management, value
      types, concurrency, package manager, server-side status, and Objective-C
      interoperability.
- [x] Add Swift vs Kotlin and Swift vs Objective-C comparisons.
- [x] Add Swift to mobile and Apple-platform guides.

### Phase 14 - Ruby

- [x] Add a complete Ruby page.
- [x] Cover MRI, RubyGems, Bundler, Rails influence, dynamic typing,
      metaprogramming, concurrency constraints, release policy, and
      productivity tradeoffs.
- [x] Add Ruby vs Python and Ruby vs PHP comparisons.
- [x] Add Ruby to web backend and scripting guides.

### Phase 15 - R

- [x] Add a complete R page.
- [x] Cover statistical computing focus, CRAN, package ecosystem, data frame
      workflows, RStudio/Posit ecosystem, interoperability, performance
      constraints, and academic/research fit.
- [x] Add R vs Python comparison for data analysis.
- [x] Add R to data science and statistics guides.

### Phase 16 - Bash / Shell

- [x] Add a complete Bash or shell scripting page with clear scope.
- [x] Cover POSIX shell boundaries, Bash-specific features, portability,
      pipelines, process model, error handling risks, and automation fit.
- [x] Add Shell vs Python comparison for scripts.
- [x] Add shell scripting to operations and automation guides.

### Phase 16.5 - Rust / Leptos / Axum Rewrite

Rewrite LangIndex from Astro/TypeScript to Stephen's Rust web stack before
adding the next language. Use `/Users/sawyer/github/fileferry` as the
implementation reference, especially `Cargo.toml`, `crates/ferry-site`,
`justfile`, route tests, embedded assets, and the Caddy/systemd deployment
runbook. The goal is not a visual redesign. Preserve the current reference
experience, URLs, source-backed content model, search/browse flows, and
deployment ergonomics while replacing the implementation.

Completion standard for the rewrite:

- The public route set remains link-compatible:
  `/`, `/about`, `/contribute`, `/languages/`, `/languages/{slug}`,
  `/comparisons/`, `/comparisons/{slug}`, `/guides/`, `/guides/{slug}`,
  `/concepts/`, `/concepts/{slug}`, `/languages.json`, `/rss.xml`,
  `/sitemap.xml`, and `/robots.txt`.
- Existing language, comparison, guide, and concept content renders with the
  same factual claims, source links, `lastVerified` dates, and metadata.
- Rust-side validation rejects malformed content, missing required metadata,
  duplicate slugs, broken internal references, missing source fields, and
  invalid dates before a build or release is considered good.
- The homepage, browse pages, detail pages, source lists, examples,
  comparison tables, and contributor surfaces remain usable without accounts
  or client-side application state.
- Search remains self-hosted and privacy-preserving. Prefer a Rust-native
  static/search-index approach if practical; otherwise keep a clearly
  documented local build step until a Rust replacement is finished. Do not
  introduce a hosted search service.
- CSS remains quiet, readable, responsive, and content-forward. Port the
  existing visual system rather than creating a marketing redesign.
- Production ships as a Rust site service behind Caddy unless a later commit
  deliberately keeps static export for a documented reason.
- `just fmt`, `just check`, `just test`, and `just build` work from a clean
  checkout and cover Rust formatting, clippy, tests, content validation, and
  release build output.
- Playwright smoke/accessibility coverage or equivalent browser checks cover
  the homepage, browse pages, representative detail pages, search, RSS,
  sitemap, 404, and responsive layouts.
- Deployment docs explain local development, release build, health checks,
  Docker Compose or systemd operation, Caddy reverse proxying, rollback, and
  verification from a clean checkout.

Suggested session plan:

- [x] Inventory current Astro behavior before deleting anything: route map,
      generated URLs, RSS/sitemap/robots output, Pagefind/search behavior,
      content schemas, source validation, internal/external link checks,
      Docker/Caddy wiring, and Playwright coverage.
- [x] Scaffold the Rust workspace in the LangIndex repo, using FileFerry's
      shape as the starting point: root `Cargo.toml`, `rust-toolchain.toml`,
      a dedicated site crate, optional content/core crate if it keeps parsing
      clean, `xtask` only if it earns its keep, and a Rust-first `justfile`.
- [x] Build the Axum shell: listener env var, tracing, `/healthz`, fallback
      404, static/embedded assets, security headers where appropriate, and
      route tests for status codes and key body markers.
- [x] Port the layout and design system into Leptos SSR components: document
      head metadata, skip link, header, footer, responsive navigation,
      source-list component, language cards, summary/detail blocks,
      comparison tables, code examples, and accessible forms or controls.
- [x] Design and implement Rust content loading: preserve Git-authored source
      files where practical, parse frontmatter and body content with typed
      structs, validate required fields, normalize slugs, expose collections
      to route handlers, and keep prose out of large JSON/YAML blobs.
- [x] Migrate all current content groups: languages, comparisons, guides, and
      concepts. Preserve current URLs, titles, summaries, examples, sources,
      and `lastVerified` values unless a source is re-checked in the same
      session.
- [x] Rebuild derived outputs in Rust or a controlled build task:
      `/languages.json`, RSS, sitemap, robots, internal-link manifest,
      source validation, and search index data.
- [x] Replace the Node/Astro toolchain with Cargo-based verification:
      formatting, clippy with warnings denied, unit tests for parsing and
      routes, fixture tests for generated feeds/maps, and content validation.
      Keep temporary Node tooling only when the replacement is not done yet
      and document the remaining dependency in this phase.
- [x] Port browser verification: update Playwright or equivalent smoke tests
      to run against the Axum site, covering desktop and mobile widths,
      keyboard navigation, search, representative pages, and accessibility.
- [x] Update deployment: Dockerfile, Compose, Caddy snippets, deployment docs,
      health checks, release binary instructions, local run commands, and
      rollback notes. Do not deploy or change production host config without
      Stephen's explicit approval.
- [x] Remove obsolete Astro/TypeScript files only after equivalent Rust
      behavior is verified: `astro.config.mjs`, `src/**/*.astro`,
      TypeScript-only scripts, `package.json`, `pnpm-lock.yaml`,
      `tsconfig.json`, and Node-only config. Keep public brand assets unless
      the Rust site embeds or serves them from a new documented location.
- [x] Run final rewrite verification:

```sh
git diff --check
just fmt
just check
just test
just build
just test-smoke
just check-links-internal
```

Use `just check-links-external` before committing if source URLs, rendered
source lists, or link-checking code changed. After this phase is checked off,
continue with Phase 17.

### Phase 17 - Dart

- [x] Add a complete Dart page.
- [x] Cover Dart VM, AOT/JIT compilation, Flutter, package tooling, null
      safety, web compilation, and ecosystem constraints outside Flutter.
- [x] Add Dart vs TypeScript and Dart vs Kotlin comparisons.
- [x] Add Dart to mobile and cross-platform guides.

### Phase 18 - Lua

- [x] Add a complete Lua page.
- [x] Cover embeddability, Lua VM, tables, metatables, C API, package
      ecosystem, game/plugin use, and implementation variants.
- [x] Add Lua vs JavaScript and Lua vs Python comparisons where useful.
- [x] Add Lua to embedded scripting and game scripting guides.

### Phase 19 - Scala

- [x] Add a complete Scala page.
- [x] Cover JVM target, functional/object-oriented blend, type system,
      implicits/givens, sbt, ecosystem, Akka/Pekko, Spark, and Scala 2 to 3
      migration considerations.
- [x] Add Scala vs Java and Scala vs Kotlin comparisons.
- [x] Add Scala to JVM and data/backend guides.

### Phase 20 - Perl

- [x] Add a complete Perl page.
- [x] Cover CPAN, text processing, Unix heritage, modern Perl status,
      compatibility, regular expressions, package tooling, and maintenance
      tradeoffs.
- [x] Add Perl vs Python and Perl vs Shell comparisons.
- [x] Add Perl to scripting and legacy-maintenance guides.

### Phase 21 - Objective-C

- [x] Add a complete Objective-C page.
- [x] Cover C interoperability, Apple platform history, runtime messaging,
      ARC, legacy codebase relevance, and migration to Swift.
- [x] Add Objective-C vs Swift comparison.
- [x] Add Objective-C to Apple-platform and legacy-maintenance guides.

### Phase 22 - Fortran

- [x] Add a complete Fortran page.
- [x] Cover standards history, numerical computing, compilers, arrays,
      interoperability with C, package/build tooling, and scientific legacy.
- [x] Add Fortran vs C++ and Fortran vs Python-for-numerics comparisons.
- [x] Add Fortran to scientific and high-performance computing guides.

### Phase 23 - Delphi / Object Pascal

- [x] Add a complete Delphi/Object Pascal page.
- [x] Cover Pascal lineage, Delphi tooling, VCL/FMX, Windows desktop
      history, cross-platform status, package ecosystem, and maintenance fit.
- [x] Add Delphi vs C# comparison.
- [x] Add Delphi to desktop and legacy-maintenance guides.

### Phase 24 - Haskell

- [x] Add a complete Haskell page.
- [x] Cover purity, laziness, type classes, GHC, Cabal/Stack, ecosystem,
      concurrency, production constraints, and education/research influence.
- [x] Add Haskell vs OCaml and Haskell vs Scala comparisons.
- [x] Add Haskell to functional programming concepts and guides.

### Phase 25 - Elixir

- [x] Add a complete Elixir page.
- [x] Cover BEAM, OTP, fault tolerance, processes, Phoenix, Mix, Hex,
      Erlang interoperability, and distributed systems fit.
- [x] Add Elixir vs Erlang and Elixir vs Ruby comparisons.
- [x] Add Elixir to concurrency and web/backend guides.

### Phase 26 - Erlang

- [x] Add a complete Erlang page.
- [x] Cover BEAM, OTP, actor-style processes, fault tolerance, hot code
      loading, telecom origins, syntax tradeoffs, and long-running systems.
- [x] Add Erlang vs Elixir comparison.
- [x] Add Erlang to concurrency and distributed systems guides.

### Phase 27 - Clojure

- [x] Add a complete Clojure page.
- [x] Cover Lisp lineage, JVM target, immutability, persistent data
      structures, REPL workflow, macros, dependency tooling, and ecosystem
      constraints.
- [x] Add Clojure vs Java and Clojure vs Scala comparisons.
- [x] Add Clojure to Lisp and JVM guides.

### Phase 28 - Julia

- [x] Add a complete Julia page.
- [x] Cover multiple dispatch, JIT compilation, package manager,
      scientific/numerical computing, performance model, Python/R/C
      interoperability, and deployment constraints.
- [x] Add Julia vs Python and Julia vs R comparisons.
- [x] Add Julia to scientific computing and data guides.

### Phase 29 - Zig

- [x] Add a complete Zig page.
- [x] Cover explicit memory allocation, C interop, build system,
      compile-time execution, cross-compilation, current maturity, package
      manager status, and standardization status.
- [x] Add Zig vs C and Zig vs Rust comparisons.
- [x] Add Zig to systems and embedded guides.

### Phase 30 - OCaml

- [x] Add a complete OCaml page.
- [x] Cover ML lineage, type inference, modules/functors, native and bytecode
      compilation, opam, Dune, multicore effects status, and industrial use.
- [x] Add OCaml vs Haskell and OCaml vs Rust comparisons where useful.
- [x] Add OCaml to functional programming guides.

### Phase 31 - F#

- [ ] Add a complete F# page.
- [ ] Cover .NET target, functional-first design, type providers, async
      workflows, package/tooling story, C# interoperability, and ecosystem
      fit.
- [ ] Add F# vs C# and F# vs OCaml comparisons.
- [ ] Add F# to .NET and functional programming guides.

### Phase 32 - Nim

- [ ] Add a complete Nim page.
- [ ] Cover native compilation, Python-like syntax, macros, memory
      management options, package tooling, C interop, and ecosystem maturity.
- [ ] Add Nim vs Zig and Nim vs Rust comparisons.
- [ ] Add Nim to systems and scripting-adjacent guides.

### Phase 33 - Crystal

- [ ] Add a complete Crystal page.
- [ ] Cover Ruby-like syntax, static typing, native compilation, shards,
      concurrency model, C bindings, and ecosystem maturity.
- [ ] Add Crystal vs Ruby and Crystal vs Go comparisons.
- [ ] Add Crystal to web/backend and compiled-scripting guides.

### Phase 34 - Solidity

- [ ] Add a complete Solidity page.
- [ ] Cover Ethereum smart contracts, EVM target, tooling, security risks,
      language evolution, contract verification, and domain-specific
      constraints.
- [ ] Add Solidity vs Rust-for-smart-contracts comparison only where the
      scope is precise.
- [ ] Add Solidity to domain-specific language and blockchain guides.

### Phase 35 - Type System Concepts

The Concepts hub at <https://langindex.dev/concepts/> currently ships a single
entry (`ownership`). The hub describes itself as "Cross-language ideas: type
systems, runtimes, memory models, and tooling." Phases 35 through 41 grow it
into a complete cross-language reference that language and comparison pages
can link into instead of re-explaining the same ideas inline.

Each concept page should follow the existing `ownership.mdx` shape: a
`title`, `slug`, `summary`, `relatedLanguages`, sourced `sources`, and a
current `lastVerified` date in frontmatter, with body sections that explain
the idea neutrally, contrast common variants, and call out watch points.
Prefer official language specifications, standards bodies, and authoritative
references over secondary commentary.

- [ ] Add a `static-vs-dynamic-typing` concept covering when types are
      checked, gradual systems that straddle the line, and the practical
      tradeoffs for refactoring, tooling, and runtime errors.
- [ ] Add a `strong-vs-weak-typing` concept that distinguishes implicit
      conversion behavior from static/dynamic checking and clears up the
      common conflation between the two axes.
- [ ] Add a `type-inference` concept covering local vs whole-program
      inference, Hindley-Milner lineage, and the readability/error-message
      tradeoffs each language accepts.
- [ ] Add a `structural-vs-nominal-typing` concept contrasting TypeScript,
      Go interfaces, and OCaml objects against nominal systems in Java, C#,
      and Rust.
- [ ] Add a `generics-and-parametric-polymorphism` concept covering
      monomorphization vs erasure, variance, bounded type parameters, and
      higher-kinded types where languages support them.
- [ ] Add an `algebraic-data-types-and-pattern-matching` concept covering
      sum types, product types, exhaustiveness, and how non-ADT languages
      approximate the pattern.
- [ ] Add a `null-safety` concept covering option types, nullable
      annotations, non-nullable defaults, and the runtime cost of retrofits.
- [ ] Cross-link these concepts from the relevant language pages and
      comparison pages (TypeScript, Java, Kotlin, Swift, Rust, Go, C#,
      Haskell, OCaml, F#, Python, Ruby) instead of restating the ideas
      inline.

### Phase 36 - Memory Management Concepts

- [ ] Expand the existing `ownership` page if any current language phase
      exposed a gap, and refresh its `lastVerified` date only if sources are
      re-checked.
- [ ] Add a `garbage-collection` concept covering tracing vs reference
      counting, generational collectors, pause-time tradeoffs, and how the
      JVM, CLR, Go, V8, BEAM, and CPython differ in practice.
- [ ] Add a `reference-counting` concept covering deterministic destruction,
      cycle handling, ARC in Swift and Objective-C, and `Rc`/`Arc` in Rust.
- [ ] Add a `manual-memory-management` concept covering `malloc`/`free`,
      arenas, custom allocators, and the failure modes that motivate
      ownership and GC.
- [ ] Add a `raii-and-deterministic-cleanup` concept covering C++
      destructors, Rust `Drop`, Python context managers, and equivalents
      across languages.
- [ ] Add a `stack-vs-heap-allocation` concept covering escape analysis,
      value vs reference semantics, and the cost model developers should
      carry in their heads.
- [ ] Add a `memory-safety` concept covering use-after-free, buffer
      overflows, data races as a safety issue, and the spectrum from "safe
      by default" to "safe by discipline."
- [ ] Link these concepts from systems-language pages (C, C++, Rust, Zig,
      Go) and from the Choosing a systems language guide.

### Phase 37 - Runtime And Execution Concepts

- [ ] Add an `interpreters-jit-and-aot` concept covering pure interpreters,
      tracing/method JITs, AOT compilation, and tiered execution models
      across CPython, V8, the JVM, CLR, GraalVM, and Dart.
- [ ] Add a `virtual-machines-and-bytecode` concept covering the JVM, CLR,
      BEAM, EVM, and CPython bytecode, with notes on what bytecode actually
      guarantees about portability.
- [ ] Add a `compilation-targets` concept covering native code, bytecode,
      WebAssembly, transpilation to JavaScript, and cross-compilation.
- [ ] Add a `foreign-function-interface` concept covering the C ABI as a
      lingua franca, marshalling cost, and the tradeoffs of safe wrappers
      vs raw bindings.
- [ ] Add an `abi-stability` concept covering C ABI guarantees, C++ ABI
      friction, Rust's deliberate lack of a stable ABI, and what this means
      for libraries and dynamic linking.
- [ ] Add a `standard-library-philosophy` concept contrasting batteries-
      included (Python, Go, .NET) with small-core ecosystems (JavaScript,
      Rust, C) and the maintenance consequences of each choice.
- [ ] Link these concepts from runtime-heavy pages (Java, C#, JavaScript,
      Python, Kotlin, Swift, Erlang, Elixir) and from the relevant guides.

### Phase 38 - Concurrency Concepts

- [ ] Add a `threads-and-shared-memory` concept covering OS threads, the
      memory-visibility problem, and why most language concurrency models
      exist to wrap or replace it.
- [ ] Add an `async-await-and-event-loops` concept covering single-threaded
      cooperative concurrency, the function-color split, and how JavaScript,
      Python, Rust, C#, and Swift each implement it.
- [ ] Add a `goroutines-and-green-threads` concept covering M:N scheduling,
      stack growth, and the tradeoffs vs OS threads and explicit async.
- [ ] Add an `actor-model-and-message-passing` concept covering Erlang/BEAM
      processes, OTP supervision, and channels in Go and Rust as a related
      message-passing pattern.
- [ ] Add a `data-races-and-memory-models` concept covering happens-before
      relationships, the Java and C++ memory models, and how Rust's ownership
      rules statically forbid the cases the others must reason about.
- [ ] Add a `structured-concurrency` concept covering Kotlin coroutines,
      Swift task groups, Trio/Anyio, and why scoped lifetimes change error
      handling and cancellation behavior.
- [ ] Link these concepts from concurrency-heavy pages (Go, Rust, Java, C#,
      Kotlin, Swift, JavaScript, Python, Erlang, Elixir) and from the
      Choosing a systems language guide.

### Phase 39 - Paradigm And Language Design Concepts

- [ ] Add an `object-oriented-programming` concept covering classes vs
      prototypes, inheritance vs composition, and how Smalltalk, Java, C#,
      Python, Ruby, and JavaScript each interpret "OO."
- [ ] Add a `functional-programming` concept covering first-class functions,
      purity, immutability, and the spectrum from "FP-friendly" (Python,
      JavaScript, Kotlin) to "FP-first" (Haskell, OCaml, F#, Elixir).
- [ ] Add an `immutability-and-persistent-data-structures` concept covering
      value semantics, copy-on-write, structural sharing, and the
      ergonomics/perf tradeoff each language picks.
- [ ] Add a `closures-and-first-class-functions` concept covering capture
      semantics, escaping vs non-escaping closures, and how this interacts
      with ownership in Rust and reference cycles in Swift.
- [ ] Add an `errors-as-values-vs-exceptions` concept contrasting Go's
      `error` return, Rust's `Result`, Swift's `throws`, checked exceptions
      in Java, and unchecked exceptions elsewhere.
- [ ] Add a `metaprogramming-and-macros` concept covering Lisp macros, Rust
      declarative and procedural macros, C++ templates, Python decorators,
      and the readability/build-time costs each carries.
- [ ] Add a `modules-and-namespacing` concept covering module systems
      (ES modules, Python packages, Java packages, Go modules, Rust crates,
      OCaml functors) and the visibility rules they enforce.
- [ ] Link these concepts from the language pages whose design they most
      directly explain and from comparison pages where the paradigm choice
      is the central tradeoff.

### Phase 40 - Tooling And Ecosystem Concepts

- [ ] Add a `package-managers` concept covering registry-backed managers
      (npm, PyPI, crates.io, Maven Central, NuGet, RubyGems, Hex), lockfile
      semantics, vendoring, and the security surface each model exposes.
- [ ] Add a `build-systems` concept covering language-native builders
      (Cargo, Go, dotnet, Maven, Gradle, sbt, Mix, swiftpm) and generic
      builders (Make, CMake, Bazel, Buck, Pants).
- [ ] Add a `formatters-and-linters` concept covering opinionated
      auto-formatters (gofmt, rustfmt, Prettier, Black) and lint families
      (Clippy, ESLint, RuboCop, pylint/ruff), with notes on when each is
      tablestakes vs optional.
- [ ] Add a `language-servers-and-editor-tooling` concept covering LSP, the
      tradeoff between language-specific IDEs and editor-agnostic tooling,
      and what a "first-class LSP" actually means.
- [ ] Add a `repl-and-interactive-development` concept covering Lisp/Clojure
      REPL workflows, the Python and Ruby shells, dotnet-script, and the
      Rust evcxr/IRust ecosystem.
- [ ] Add a `testing-cultures` concept covering xUnit, property-based
      testing, doctests, snapshot tests, fuzzing, and which language
      ecosystems push which approach as default.
- [ ] Add a `documentation-cultures` concept covering rustdoc, godoc,
      Javadoc, JSDoc/TSDoc, Doxygen, Sphinx, and how source-derived docs
      shape API design.
- [ ] Link these tooling concepts from every language page's tooling
      section instead of restating ecosystem norms inline.

### Phase 41 - Concepts Hub Polish

- [ ] Re-read every concept page after the prior phases and tighten cross-
      links so each one references neighbors via `[[slug]]`-style anchors or
      explicit relative links.
- [ ] Add a topical grouping to `/concepts/` (type systems, memory, runtime,
      concurrency, paradigms, tooling) instead of the current flat grid,
      while preserving the existing card layout and alphabetical fallback.
- [ ] Ensure each language page's frontmatter and body link out to the
      concepts it depends on (typing, memory model, runtime model,
      concurrency model, tooling) rather than re-explaining them.
- [ ] Ensure every comparison page that turns on a single concept (for
      example, Rust vs Go on memory model) links to that concept page above
      the fold.
- [ ] Refresh `lastVerified` dates only when sources were actually
      re-checked and run `just check-links-external` after the final batch.
- [ ] Update `README.md` if the Concepts surface has materially changed from
      what the current description implies.

---

## Cross-Cutting Content Backlog

- [ ] Create a documented popularity/adoption methodology page before
      publishing any public ranking or "most popular" page.
- [ ] Add CODEOWNERS only when maintainers or review owners exist.
- [ ] Add a source-quality badge or field only after the editorial standard
      defines levels clearly.
- [ ] Add benchmark policy before publishing performance comparisons.
- [ ] Add compatibility-policy fields if enough language pages expose
      consistent data.
- [ ] Add machine-readable language metadata fields only when a real route or
      UI needs them.
- [ ] Periodically re-run external link checks and refresh `lastVerified`
      dates only when the sources were actually checked.

---

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
