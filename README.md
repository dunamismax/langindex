<p align="center">
  <img src="public/brand/langindex-banner-dark.png" alt="LangIndex" width="1200">
</p>

LangIndex is an open source, self-hosted reference for programming languages.

Live site: [langindex.dev](https://langindex.dev)

The goal is to become the best single place on the internet to understand programming languages: where they came from, what they are good at, what tradeoffs they make, how they compare to nearby languages, and how developers can start evaluating them with accurate, community-maintained information.

## Why This Exists

There are more programming languages than any one developer can reasonably track. Wikipedia is useful, but it is not shaped around the questions developers ask when choosing, learning, comparing, or maintaining languages.

LangIndex is meant to be:

- **Accurate**: factual claims should be sourced and reviewable.
- **Current**: language communities should be able to correct and improve their own pages.
- **Practical**: pages should answer what a developer needs to know, not just preserve historical trivia.
- **Neutral**: language comparisons should explain fit and tradeoffs, not crown winners.
- **Self-hostable**: the public site should be reproducible from the open repository.

## What Each Language Page Should Answer

Each language entry should aim to cover:

- Origin, history, creators, and design goals
- Best-fit use cases
- Poor-fit or risky use cases
- Language highlights and distinguishing features
- Syntax and examples
- Type system, runtime, memory model, and concurrency model where relevant
- Tooling, package management, ecosystem, and deployment story
- Governance, standardization, and implementation status
- Comparisons with related languages
- Official resources and high-quality community resources
- Last verified date

## Contribution Model

LangIndex is intended to be maintained in public. Language authors, maintainers, and community contributors are encouraged to submit corrections, examples, and improvements for their languages.

The contribution standard is simple:

- Separate facts from judgment.
- Cite factual claims when they are not obvious from official documentation or source code.
- Prefer official sources for language design, releases, governance, and tooling.
- Explain tradeoffs clearly instead of writing advocacy copy.
- Keep examples small, idiomatic, and runnable when possible.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the initial contribution rules.

## Project Status

LangIndex is live at [https://langindex.dev](https://langindex.dev).

The repository implementation is a Rust web stack: a Cargo workspace with
Axum serving Leptos server-rendered pages, modeled after Stephen's FileFerry
site architecture. The live self-hosted site may still be running the previous
static deployment until the Rust service is synced to the Ubuntu host and
redeployed.

LangIndex ships four content groups:

- `languages` — individual programming language profiles.
- `comparisons` — dimensional, tradeoff-first comparisons between related
  languages.
- `guides` — cross-language decision guides framed by the problem the
  developer is solving.
- `concepts` — 15 shared programming-language ideas referenced by multiple
  language pages.

The initial milestone — defining the content model and publishing a
source-backed seed set — is complete. See [BUILD.md](BUILD.md) for current
coverage, the Rust rewrite phase, and the later per-language expansion plan.

## License

Project code is licensed under the MIT License. LangIndex content and data are
licensed under Creative Commons Attribution-ShareAlike 4.0 International
(`CC-BY-SA-4.0`) unless noted otherwise. See [LICENSE-CONTENT.md](LICENSE-CONTENT.md).
