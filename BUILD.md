# LangIndex Language Build Plan

This file tracks language additions that should be built one phase at a time.
Each phase is intended to be small enough for one agentic engineering pass:
add the language page, add the closest comparisons, update relevant guides,
update validation fixtures or counts, and run the checks named below.

This is a planning file, not a substitute for source-backed content work. Every
language page, comparison, and guide update still needs fresh official-source
research, citations in frontmatter, and a current `lastVerified` date when it is
implemented.

## Research Basis

Current repository coverage already includes the mainstream top tier from the
checked popularity sources: Python, JavaScript, TypeScript, Java, C#, C, C++,
Go, Rust, PHP, Swift, SQL, Bash / Shell, R, Kotlin, Ruby, Dart, Lua, Assembly,
MATLAB, Perl, Elixir, Scala, Delphi, Zig, Erlang, Fortran, Ada, F#, OCaml,
COBOL, and related languages.

Selection signals checked on 2026-05-23:

- [Stack Overflow Developer Survey 2025](https://survey.stackoverflow.co/2025/technology) lists missing or under-modeled languages and language-like ecosystems including VBA, GDScript, Lisp, MicroPython, Gleam, Prolog, and Mojo.
- [GitHub Octoverse 2024](https://github.blog/news-insights/octoverse/octoverse-2024/) shows the current top GitHub languages are already covered, while infrastructure-as-code and T-SQL activity suggest HCL and SQL dialect coverage are useful follow-ons.
- [TIOBE Index for May 2026](https://www.tiobe.com/tiobe-index/) lists missing top-50 candidates such as Scratch, Classic Visual Basic, PL/SQL, Prolog, SAS, ML, GML, LabVIEW, ABAP, VBScript, Lisp, X++, D, Ladder Logic, Transact-SQL, and CFML.
- [LangPop](https://langpop.com/) confirms the multi-source top 10 are already covered, so new work should fill ecosystem gaps rather than duplicate existing mainstream pages.
- [Odin official documentation](https://odin-lang.org/docs/) and [Odin FAQ](https://odin-lang.org/docs/faq/) identify Odin-specific areas that the requested Odin phase needs to cover.

## Phase Rules

- [ ] Complete exactly one phase per focused branch or PR unless Stephen expands scope.
- [ ] Add the new slug to `KNOWN_LANGUAGE_SLUGS` in `crates/langindex-site/src/content.rs`.
- [ ] Add or update tests and expected content counts touched by the new page.
- [ ] Prefer official sources: language docs, specifications, governance pages, repositories, release notes, package indexes, and maintainer statements.
- [ ] Keep comparisons dimensional and practical. Do not rank languages as universal winners.
- [ ] Update guide `candidates` and body prose when the language belongs in an existing guide.
- [ ] Run `just fmt`, `just check`, `just test`, and `just build` for language, comparison, or guide changes.

## Phase 1 - Odin

- [x] Add a complete Odin page at `src/content/languages/odin.mdx`.
- [x] Cover origin, Ginger Bill / Bill Hall, design goals, data-oriented programming, manual memory management, explicit allocators, `defer`, `using`, procedure groups, distinct types, array programming, SIMD/vector-oriented features, package layout, `core` and `vendor` library collections, foreign function support, graphics/game-development fit, and current implementation maturity.
- [x] Cover the lack of an official package manager and what that means for dependency management, vendoring, reproducible builds, and project structure.
- [x] Add Odin vs Zig comparison.
- [x] Add Odin vs C comparison.
- [x] Add Odin vs Rust comparison.
- [x] Add Odin to systems, performance-critical, game-development, and embedded-language guides where the comparison is useful and not overstated.
- [x] Update validation slug lists, route/content-count tests, search fixtures if needed, and any language index expectations.
- [x] Run the normal verification commands.

## Phase 2 - GDScript

- [x] Add a complete GDScript page.
- [x] Cover Godot coupling, dynamic typing with optional static annotations, scene tree integration, signals, nodes, resources, tool scripts, editor workflow, runtime model, performance limits, and the distinction between GDScript, C#, C++, and GDExtension in Godot projects.
- [x] Add GDScript vs C# for Godot comparison.
- [x] Add GDScript vs Lua comparison if the page needs a lightweight embedded-scripting comparison.
- [x] Add GDScript to the game-development and embedded-scripting guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 3 - VBA

- [x] Add a complete VBA page.
- [x] Cover Visual Basic for Applications, Office automation, macro security, COM automation, Excel/Access use, legacy business workflows, type system, editor/debugger limitations, Windows and Microsoft 365 coupling, and migration pressure.
- [x] Add VBA vs PowerShell comparison.
- [x] Add VBA vs Visual Basic comparison.
- [x] Add VBA vs Python for Office automation comparison if the scope can be source-backed.
- [x] Add VBA to legacy-maintenance, scripting/automation, and enterprise guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 4 - Prolog

- [x] Add a complete Prolog page.
- [x] Cover logic programming, facts and rules, unification, backtracking, predicates, recursion, declarative search, constraint logic programming, ISO Prolog, SWI-Prolog, common educational and symbolic-AI uses, and operational pitfalls.
- [x] Add Prolog vs Haskell comparison.
- [x] Add Prolog vs Python for rule/search-heavy problems comparison.
- [x] Add Prolog to functional-programming or a new logic-programming guide if the existing guide cannot carry it cleanly.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 5 - Common Lisp

- [x] Add a complete Common Lisp page.
- [x] Cover standardization, dynamic typing, homoiconicity, macros, CLOS, condition system, image-based development, REPL workflow, SBCL and other implementations, ASDF/Quicklisp, performance profile, and ecosystem maturity.
- [x] Add Common Lisp vs Clojure comparison.
- [x] Add Common Lisp vs Scheme comparison if Scheme exists by the time this phase starts; Scheme was not yet published during Phase 5, so this comparison was completed in Phase 6.
- [x] Add Common Lisp to the Lisp-family and functional-programming guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 6 - Scheme

- [x] Add a complete Scheme page.
- [x] Cover minimal core language design, lexical scope, proper tail calls, hygienic macros, continuations, RnRS standards, Racket relationship and distinction, Guile, Chez Scheme, education, language research, and embedding.
- [x] Add Scheme vs Common Lisp comparison.
- [x] Add Scheme vs Clojure comparison.
- [x] Add Scheme to the Lisp-family, functional-programming, and embedded-scripting guides where accurate.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 7 - D

- [x] Add a complete D page.
- [x] Cover native compilation, C/C++ adjacency, templates, compile-time function evaluation, ranges, garbage collection with systems escape hatches, `@safe`, `dub`, Phobos, LDC/GDC/DMD, ecosystem size, and governance.
- [x] Add D vs C++ comparison.
- [x] Add D vs Rust comparison.
- [x] Add D vs Zig comparison.
- [x] Add D to systems and performance-critical guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 8 - HCL

- [x] Add a complete HashiCorp Configuration Language page if the product scope accepts declarative infrastructure languages as language pages.
- [x] Cover Terraform/OpenTofu usage, declarative resource graphs, expressions, modules, variables, providers, state coupling, evaluation model, limits versus general-purpose languages, and HashiCorp/OpenTofu ecosystem split.
- [x] Add HCL vs YAML for infrastructure configuration comparison only if YAML or configuration-language treatment exists; otherwise add HCL vs Bash/Python for infrastructure automation.
- [x] Add HCL to a new or existing infrastructure-as-code guide.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 9 - ABAP

- [x] Add a complete ABAP page.
- [x] Cover SAP coupling, business application development, Open SQL, internal tables, reports, classes/objects, transports, SAP NetWeaver/ABAP Platform, RAP, cloud ABAP restrictions, tooling, and legacy modernization constraints.
- [x] Add ABAP vs Java for enterprise systems comparison.
- [x] Add ABAP vs SQL for business-data logic comparison if the scope is clear.
- [x] Add ABAP to enterprise and legacy-maintenance guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 10 - Verilog / SystemVerilog

- [x] Add a complete Verilog/SystemVerilog page, or split into separate pages if research shows the combined treatment would be misleading.
- [x] Cover hardware description and verification, simulation vs synthesis, modules, always blocks, blocking/nonblocking assignment, testbenches, SystemVerilog verification features, FPGA/ASIC workflows, toolchains, and standards.
- [x] Add Verilog/SystemVerilog vs VHDL comparison once VHDL exists, or stage the comparison until Phase 11 lands. Completed in Phase 11.
- [x] Add Verilog/SystemVerilog to embedded and safety-critical guides, with clear hardware-language framing.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 11 - VHDL

- [x] Add a complete VHDL page.
- [x] Cover hardware description, strong typing, entities/architectures, processes, signals, simulation vs synthesis, FPGA/ASIC use, IEEE standardization, tooling, and defense/aerospace relevance.
- [x] Add VHDL vs Verilog/SystemVerilog comparison.
- [x] Add VHDL to embedded and safety-critical guides, with clear hardware-language framing.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 12 - Scratch

- [x] Add a complete Scratch page if educational visual languages are in scope.
- [x] Cover block-based programming, Scratch Foundation/MIT origins, event-driven model, sprites, costumes, broadcasts, variables/lists, classroom use, limitations, and transition paths to text languages.
- [x] Add Scratch vs Python for first programming language comparison.
- [x] Add Scratch to a new beginner/education guide rather than production-language guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 13 - Gleam

- [x] Add a complete Gleam page.
- [x] Cover BEAM target, JavaScript target, static typing, functional style, OTP interoperability, package manager/build tool, formatter, actor/process model, relationship to Erlang and Elixir, ecosystem maturity, and governance.
- [x] Add Gleam vs Elixir comparison.
- [x] Add Gleam vs Erlang comparison.
- [x] Add Gleam to distributed-systems, concurrency-oriented backend, functional-programming, and JavaScript/compile-to-JavaScript guides where accurate.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 14 - Mojo

- [x] Add a complete Mojo page only after verifying the current license, availability, and implementation status from Modular sources.
- [x] Cover Python relationship, MLIR, MAX/Modular ecosystem, accelerator and AI-kernel goals, type system, ownership/lifetime features if stable enough to document, package/tooling story, platform support, and maturity risks.
- [x] Add Mojo vs Python comparison.
- [x] Add Mojo vs Julia comparison if the AI/numerics scope is strong enough.
- [x] Add Mojo to Python/data/AI, performance-critical, and numerics guides with conservative maturity language.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 15 - MicroPython

- [x] Add a complete MicroPython page.
- [x] Cover Python subset, microcontroller targets, REPL, memory constraints, hardware APIs, `machine` module, board ports, package story, CPython compatibility limits, and embedded education/prototyping.
- [x] Add MicroPython vs Python comparison.
- [x] Add MicroPython vs C for microcontrollers comparison.
- [x] Add MicroPython to embedded-language and embedded-scripting guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 16 - PL/SQL

- [x] Add a complete PL/SQL page.
- [x] Cover Oracle Database coupling, procedural SQL, packages, stored procedures, triggers, exceptions, transaction context, optimizer/database boundary, deployment, tooling, and enterprise maintenance.
- [x] Add PL/SQL vs SQL comparison.
- [x] Add PL/SQL vs Java for database-adjacent business logic comparison if source-backed.
- [x] Add PL/SQL to SQL/data-access, enterprise, and legacy-maintenance guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 17 - Transact-SQL

- [x] Add a complete Transact-SQL page.
- [x] Cover Microsoft SQL Server/Azure SQL coupling, procedural SQL, stored procedures, functions, triggers, temp tables, transactions, error handling, query optimizer boundary, tooling, and operational constraints.
- [x] Add Transact-SQL vs SQL comparison.
- [x] Add Transact-SQL vs PL/SQL comparison once PL/SQL exists.
- [x] Add Transact-SQL to SQL/data-access, enterprise, and legacy-maintenance guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 18 - SAS

- [x] Add a complete SAS language page.
- [x] Cover DATA step, PROC ecosystem, statistical analysis, clinical/regulatory use, macro language, datasets, enterprise tooling, licensing, and migration pressure toward Python/R where relevant.
- [x] Add SAS vs R comparison.
- [x] Add SAS vs Python for analytics comparison.
- [x] Add SAS to data-analysis/statistics and legacy-maintenance guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 19 - LabVIEW

- [x] Add a complete LabVIEW page if visual/dataflow languages are in scope.
- [x] Cover graphical dataflow programming, virtual instruments, measurement/control systems, hardware integration, NI ecosystem, real-time/FPGA targets, versioning challenges, and team-maintenance constraints.
- [x] Add LabVIEW vs Python for test automation comparison.
- [x] Add LabVIEW vs C for hardware/control systems comparison if source-backed.
- [x] Add LabVIEW to embedded, performance-critical where appropriate, and safety-critical guides with careful scope language.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Phase 20 - CFML

- [x] Add a complete CFML page.
- [x] Cover ColdFusion/Lucee ecosystems, tag and script syntax, web application history, JVM deployment, database-heavy server-rendered apps, package/tooling story, and legacy modernization constraints.
- [x] Add CFML vs PHP comparison.
- [x] Add CFML vs Java for legacy web modernization comparison if source-backed.
- [x] Add CFML to backend/web, enterprise, and legacy-maintenance guides.
- [x] Update validation and tests.
- [x] Run the normal verification commands.

## Deferred Or Scope-Question Candidates

- [ ] Decide whether HTML and CSS belong as LangIndex language pages, concept pages, or guide-only material. Stack Overflow tracks them with programming/scripting/markup languages, but they are markup/style languages rather than general-purpose programming languages.
- [ ] Decide whether template languages such as Blade should be pages or framework-adjacent guide material.
- [ ] Decide whether YAML, JSON, TOML, and XML should be content pages. They are important to programming practice but are data/configuration languages, not programming languages in the same sense as most current LangIndex pages.
- [ ] Decide whether vendor-specific enterprise languages such as X++, Apex, AL, Ladder Logic, and RPG should be grouped into an enterprise/industrial backlog after ABAP, PL/SQL, Transact-SQL, SAS, LabVIEW, and CFML are complete.
