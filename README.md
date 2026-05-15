# LangIndex

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

LangIndex has an initial Astro static-site skeleton with typed content collections, seed language pages, a comparison, a guide, and concept content. The intended public URL is:

[https://langindex.dev](https://langindex.dev)

The first milestone is to define the content model and publish a small, high-quality set of language pages before expanding coverage.

## License

Project code is licensed under GPL-3.0. The content/data license is not finalized yet and should be decided before accepting broad external content contributions.
