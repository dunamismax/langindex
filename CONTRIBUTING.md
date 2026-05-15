# Contributing To LangIndex

LangIndex exists to give developers accurate, practical, community-maintained information about programming languages.

## Contribution Principles

- Be accurate before being comprehensive.
- Prefer primary sources: official language docs, specifications, release notes, governance pages, repositories, and maintainer statements.
- Avoid hype, dunking, and advocacy copy.
- Explain tradeoffs in terms of use case, constraints, maturity, ecosystem, and operational cost.
- Keep examples idiomatic for the language being described.
- Mark uncertainty directly instead of smoothing it over.

## Good Contributions

Useful contributions include:

- Correcting factual errors
- Adding missing official links
- Improving examples
- Clarifying design goals or historical context
- Adding practical use cases and non-use cases
- Updating stale version, tooling, governance, or ecosystem details
- Improving comparisons between related languages

## Language Page Standard

Each language page should eventually answer:

- What is the origin of the language?
- Why was it created?
- What is it best suited for?
- What is it not well suited for?
- What are its most important language features?
- What does idiomatic code look like?
- What runtime, compiler, or interpreter does it normally use?
- What package manager and build tools are common?
- How mature is the ecosystem?
- How is the language governed?
- Which languages is it most often compared with?
- When was this information last verified?

## Source Standard

Use citations for claims about:

- Language origin and authorship
- Release dates
- Governance or standards status
- Performance claims
- Adoption claims
- Compatibility guarantees
- Security or safety properties
- Comparisons that depend on specific language features

Prefer official sources. If no official source exists, use the most authoritative available source and make the limitation clear.

## Comparison Standard

Comparisons should be dimensional, not tribal.

Good comparison framing:

- "Rust and Zig both target systems programming, but make different tradeoffs around memory safety, language complexity, and compile-time control."
- "Python and Julia overlap heavily in scientific computing, but differ in ecosystem maturity, performance model, and deployment expectations."

Bad comparison framing:

- "Language X is better than language Y."
- "Nobody should use language X."
- "Language Y is dead" without evidence and context.

## Pull Requests

Pull requests should include:

- A concise description of what changed
- Source links for non-obvious factual claims
- Notes about anything uncertain or intentionally omitted

Small, focused pull requests are easier to review than broad rewrites.
