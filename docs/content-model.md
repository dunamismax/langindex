# Content Model

LangIndex keeps Git-authored Markdown/MDX content in `src/content/**` and uses
Rust-side typed parsing and validation before the site starts, tests run, or a
release is considered good.

## Collections

- `languages`: individual programming language profiles.
- `comparisons`: dimensional language comparisons.
- `guides`: cross-language decision guides.
- `concepts`: glossary and reference pages for shared programming-language ideas.

## Shared Source Shape

Sources are frontmatter objects:

```yaml
sources:
  - title: Rust Programming Language
    url: https://www.rust-lang.org/
    publisher: Rust Foundation
```

Use official sources when possible: language sites, specifications, official docs, release notes, governance pages, and repositories.

## Language Metadata

Language pages keep stable metadata in frontmatter and explanatory material in MDX body content. Keep prose out of frontmatter unless it is a short summary or list item used for browsing.

Required language fields:

- `title`
- `slug` (must be a known LangIndex language slug)
- `status` — one of `active`, `stable`, `experimental`, `legacy`, `inactive`
- `summary`
- `typing` — object with `discipline` (required) and `strength` (optional)
- `memory` — object with `model`
- `runtime` — object with `model`
- `officialSite`
- `sources` — at least one entry
- `lastVerified` — ISO date

Optional language fields:

- `firstReleased` — integer year
- `creators`
- `paradigms`
- `repository`
- `packageManagers`
- `comparedWith` — language slugs (validated against known LangIndex slugs)
- `bestFor`
- `poorFit`

## Comparisons, Guides, And Concepts

Comparison frontmatter requires `title`, `slug`, `summary`, `languages` (at
least two registry-valid slugs), `sources`, and `lastVerified`. `useCases`
is optional.

Guide frontmatter requires `title`, `slug`, `summary`, `sources`, and
`lastVerified`. `audience` and `candidates` (language slugs) are optional.

Concept frontmatter requires `title`, `slug`, `summary`, `sources`, and
`lastVerified`. `relatedLanguages` (language slugs) is optional.

Starter frontmatter for language and comparison pages lives in
`docs/templates/`.

## Validation

The canonical schemas, slug checks, source checks, date checks, and
internal-link checks live in `crates/langindex-site/src/content.rs`.
Frontmatter is validated by
`cargo run -p langindex-site -- validate-content`, `just check`, and the Rust
test suite. Update this document when schema changes affect contributors.
