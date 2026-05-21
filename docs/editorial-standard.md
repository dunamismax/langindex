# Editorial Standard

LangIndex content should help developers evaluate fit and tradeoffs without advocacy framing.

## Content Objective

LangIndex expands one durable reference page at a time. Accuracy beats
coverage, but expansion order should favor languages and concepts developers
are likely to search for, compare, or use in production.

Each substantial content change should leave the site better in four ways:

- A complete, source-backed page.
- Useful cross-links to languages, comparisons, guides, and concepts.
- Improved browse/search metadata where the content model supports it.
- A clear contributor surface for maintainers or community experts to correct
  the page through normal pull requests.

## Rules

- Prefer official sources.
- Cite factual claims that are not obvious from the linked official resources.
- Separate facts from judgment.
- Mark uncertainty directly.
- Avoid benchmark claims until a benchmark policy exists.
- Avoid adoption rankings until a methodology exists.
- Keep examples small, idiomatic, and runnable when practical.
- Use current-state wording unless history is the point.
- Include a visible `lastVerified` date on substantial pages.

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
- Governance, standards, ownership, release cadence, and compatibility policy
  where those apply.
- Official or high-quality source-backed facts, visible source links, and a
  current `lastVerified` date.

A concept page is complete when it:

- Defines the term neutrally and distinguishes it from commonly conflated
  ideas.
- Compares practical variants across languages without crowning winners.
- Explains watch points and production consequences.
- Links to relevant language and comparison pages.
- Uses official specifications, docs, standards bodies, maintainer material,
  or clearly labeled secondary sources.

## Expansion Method

There is no single authoritative popularity ranking. Re-check current signals
before each major language batch:

- Developer usage survey signal: Stack Overflow Developer Survey.
- Repository activity signal: GitHub Octoverse or comparable repository
  ecosystem reporting.
- Search/community signal: TIOBE Index, checked monthly when used.
- LangIndex product judgment: prioritize pages that unlock high-value
  comparisons, common production decisions, and broad contributor interest.

## Review Checklist

- The page identifies the language or topic clearly above the fold.
- Practical fit and poor-fit cases are framed as constraints, not universal judgments.
- Sources are checked and linked.
- Examples compile or are clearly illustrative.
- The page does not invent citations, adoption claims, or performance claims.
