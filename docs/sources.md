# Source And Citation Guidance

LangIndex should make factual claims traceable without turning pages into
footnote soup.

## Preferred Sources

Use primary sources first:

- Official language websites.
- Specifications and language references.
- Official documentation.
- Release notes and changelogs.
- Governance pages and standards bodies.
- Official repositories and maintainer statements.

Use secondary sources only when no primary source answers the question, and say
what limitation remains.

## Claims That Need Sources

Source claims about:

- Origin, authorship, and release dates.
- Governance, standards status, and compatibility guarantees.
- Runtime, compiler, interpreter, or package-manager behavior.
- Memory safety, type safety, security, and reliability properties.
- Performance, adoption, ecosystem maturity, or production-readiness.
- Comparisons that depend on specific language features.

## Frontmatter Sources

Every language and comparison page needs at least one source in frontmatter:

```yaml
sources:
  - title: Rust Programming Language
    url: https://www.rust-lang.org/
    publisher: Rust Foundation
```

Use stable public URLs. Prefer HTTPS. Avoid links that require login,
tracking parameters, private files, or temporary build artifacts.

## Last Verified

Set `lastVerified` to the date the sources were actually checked. Do not update
it just because nearby prose changed.
