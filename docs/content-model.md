# Content Model

LangIndex uses Astro Content Collections for typed content validation at build time.

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

Required language fields include:

- `title`
- `slug`
- `status`
- `summary`
- `typing`
- `memory`
- `runtime`
- `officialSite`
- `sources`
- `lastVerified`

## Validation

The canonical schemas live in `src/content.config.ts`. Update this document when schema changes affect contributors.
