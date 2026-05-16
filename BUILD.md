# BUILD.md

Active content build plan for LangIndex. `README.md` introduces the
product. `AGENTS.md` defines durable operating rules. This file now tracks
the next stage: turning the live skeleton into a deep, source-backed
programming language reference.

The previous foundation phases are complete. Do not re-add old scaffold
work here unless it becomes active again. Move stable technical material
into `docs/`; keep this file focused on current content expansion.

---

## Current Baseline

Observed on 2026-05-16:

- Public site is live at [https://langindex.dev](https://langindex.dev).
- Astro 6 static site, TypeScript, MDX, Astro Content Collections, Zod,
  Tailwind CSS 4, Pagefind, Playwright, Docker, Compose, and Caddy wiring
  exist.
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
  - Choosing C# for .NET, desktop, games, and enterprise work guide
  - Choosing a systems language guide
  - Choosing an embedded language guide
  - Choosing Python for scripting, backend, data, and AI work guide
  - Choosing Java for backend, enterprise, and JVM work guide
  - Choosing SQL for data access and application architecture guide
  - Choosing PHP for web backends and CMS work guide
  - Choosing Kotlin for Android, JVM, and multiplatform work guide
  - Choosing Swift for Apple-platform and mobile work guide
  - Ownership concept page
- Current local baseline:

```text
Node.js: v26.0.0
pnpm: 10.32.1
```

---

## Content Objective

LangIndex should expand one language at a time, with each language page
treated as a durable reference rather than a stub. Accuracy beats coverage,
but expansion order should still favor languages developers are most likely
to search for, compare, or use in production.

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
- Update or create `src/content/languages/{slug}.mdx`.
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

- [ ] Add a complete R page.
- [ ] Cover statistical computing focus, CRAN, package ecosystem, data frame
      workflows, RStudio/Posit ecosystem, interoperability, performance
      constraints, and academic/research fit.
- [ ] Add R vs Python comparison for data analysis.
- [ ] Add R to data science and statistics guides.

### Phase 16 - Bash / Shell

- [ ] Add a complete Bash or shell scripting page with clear scope.
- [ ] Cover POSIX shell boundaries, Bash-specific features, portability,
      pipelines, process model, error handling risks, and automation fit.
- [ ] Add Shell vs Python comparison for scripts.
- [ ] Add shell scripting to operations and automation guides.

### Phase 17 - Dart

- [ ] Add a complete Dart page.
- [ ] Cover Dart VM, AOT/JIT compilation, Flutter, package tooling, null
      safety, web compilation, and ecosystem constraints outside Flutter.
- [ ] Add Dart vs TypeScript and Dart vs Kotlin comparisons.
- [ ] Add Dart to mobile and cross-platform guides.

### Phase 18 - Lua

- [ ] Add a complete Lua page.
- [ ] Cover embeddability, Lua VM, tables, metatables, C API, package
      ecosystem, game/plugin use, and implementation variants.
- [ ] Add Lua vs JavaScript and Lua vs Python comparisons where useful.
- [ ] Add Lua to embedded scripting and game scripting guides.

### Phase 19 - Scala

- [ ] Add a complete Scala page.
- [ ] Cover JVM target, functional/object-oriented blend, type system,
      implicits/givens, sbt, ecosystem, Akka/Pekko, Spark, and Scala 2 to 3
      migration considerations.
- [ ] Add Scala vs Java and Scala vs Kotlin comparisons.
- [ ] Add Scala to JVM and data/backend guides.

### Phase 20 - Perl

- [ ] Add a complete Perl page.
- [ ] Cover CPAN, text processing, Unix heritage, modern Perl status,
      compatibility, regular expressions, package tooling, and maintenance
      tradeoffs.
- [ ] Add Perl vs Python and Perl vs Shell comparisons.
- [ ] Add Perl to scripting and legacy-maintenance guides.

### Phase 21 - Objective-C

- [ ] Add a complete Objective-C page.
- [ ] Cover C interoperability, Apple platform history, runtime messaging,
      ARC, legacy codebase relevance, and migration to Swift.
- [ ] Add Objective-C vs Swift comparison.
- [ ] Add Objective-C to Apple-platform and legacy-maintenance guides.

### Phase 22 - Fortran

- [ ] Add a complete Fortran page.
- [ ] Cover standards history, numerical computing, compilers, arrays,
      interoperability with C, package/build tooling, and scientific legacy.
- [ ] Add Fortran vs C++ and Fortran vs Python-for-numerics comparisons.
- [ ] Add Fortran to scientific and high-performance computing guides.

### Phase 23 - Delphi / Object Pascal

- [ ] Add a complete Delphi/Object Pascal page.
- [ ] Cover Pascal lineage, Delphi tooling, VCL/FMX, Windows desktop
      history, cross-platform status, package ecosystem, and maintenance fit.
- [ ] Add Delphi vs C# comparison.
- [ ] Add Delphi to desktop and legacy-maintenance guides.

### Phase 24 - Haskell

- [ ] Add a complete Haskell page.
- [ ] Cover purity, laziness, type classes, GHC, Cabal/Stack, ecosystem,
      concurrency, production constraints, and education/research influence.
- [ ] Add Haskell vs OCaml and Haskell vs Scala comparisons.
- [ ] Add Haskell to functional programming concepts and guides.

### Phase 25 - Elixir

- [ ] Add a complete Elixir page.
- [ ] Cover BEAM, OTP, fault tolerance, processes, Phoenix, Mix, Hex,
      Erlang interoperability, and distributed systems fit.
- [ ] Add Elixir vs Erlang and Elixir vs Ruby comparisons.
- [ ] Add Elixir to concurrency and web/backend guides.

### Phase 26 - Erlang

- [ ] Add a complete Erlang page.
- [ ] Cover BEAM, OTP, actor-style processes, fault tolerance, hot code
      loading, telecom origins, syntax tradeoffs, and long-running systems.
- [ ] Add Erlang vs Elixir comparison.
- [ ] Add Erlang to concurrency and distributed systems guides.

### Phase 27 - Clojure

- [ ] Add a complete Clojure page.
- [ ] Cover Lisp lineage, JVM target, immutability, persistent data
      structures, REPL workflow, macros, dependency tooling, and ecosystem
      constraints.
- [ ] Add Clojure vs Java and Clojure vs Scala comparisons.
- [ ] Add Clojure to Lisp and JVM guides.

### Phase 28 - Julia

- [ ] Add a complete Julia page.
- [ ] Cover multiple dispatch, JIT compilation, package manager,
      scientific/numerical computing, performance model, Python/R/C
      interoperability, and deployment constraints.
- [ ] Add Julia vs Python and Julia vs R comparisons.
- [ ] Add Julia to scientific computing and data guides.

### Phase 29 - Zig

- [ ] Add a complete Zig page.
- [ ] Cover explicit memory allocation, C interop, build system,
      compile-time execution, cross-compilation, current maturity, package
      manager status, and standardization status.
- [ ] Add Zig vs C and Zig vs Rust comparisons.
- [ ] Add Zig to systems and embedded guides.

### Phase 30 - OCaml

- [ ] Add a complete OCaml page.
- [ ] Cover ML lineage, type inference, modules/functors, native and bytecode
      compilation, opam, Dune, multicore effects status, and industrial use.
- [ ] Add OCaml vs Haskell and OCaml vs Rust comparisons where useful.
- [ ] Add OCaml to functional programming guides.

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
