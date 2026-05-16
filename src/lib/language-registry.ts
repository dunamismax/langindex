type LanguageRegistryEntry = {
  title: string;
  state: "published" | "planned";
};

export const languageRegistry = {
  bash: { title: "Bash / Shell", state: "published" },
  c: { title: "C", state: "published" },
  clojure: { title: "Clojure", state: "planned" },
  cpp: { title: "C++", state: "published" },
  crystal: { title: "Crystal", state: "planned" },
  csharp: { title: "C#", state: "published" },
  dart: { title: "Dart", state: "planned" },
  delphi: { title: "Delphi / Object Pascal", state: "planned" },
  elixir: { title: "Elixir", state: "planned" },
  erlang: { title: "Erlang", state: "planned" },
  fortran: { title: "Fortran", state: "planned" },
  fsharp: { title: "F#", state: "planned" },
  go: { title: "Go", state: "published" },
  haskell: { title: "Haskell", state: "planned" },
  java: { title: "Java", state: "published" },
  javascript: { title: "JavaScript", state: "published" },
  julia: { title: "Julia", state: "planned" },
  kotlin: { title: "Kotlin", state: "published" },
  lua: { title: "Lua", state: "planned" },
  nim: { title: "Nim", state: "planned" },
  "objective-c": { title: "Objective-C", state: "planned" },
  ocaml: { title: "OCaml", state: "planned" },
  perl: { title: "Perl", state: "planned" },
  php: { title: "PHP", state: "published" },
  python: { title: "Python", state: "published" },
  r: { title: "R", state: "published" },
  ruby: { title: "Ruby", state: "published" },
  rust: { title: "Rust", state: "published" },
  scala: { title: "Scala", state: "planned" },
  solidity: { title: "Solidity", state: "planned" },
  sql: { title: "SQL", state: "published" },
  swift: { title: "Swift", state: "published" },
  typescript: { title: "TypeScript", state: "published" },
  zig: { title: "Zig", state: "planned" },
} as const satisfies Record<string, LanguageRegistryEntry>;

export type KnownLanguageSlug = keyof typeof languageRegistry;

export function isKnownLanguageSlug(slug: string): slug is KnownLanguageSlug {
  return Object.hasOwn(languageRegistry, slug);
}

export function languageTitleForSlug(slug: string) {
  return isKnownLanguageSlug(slug) ? languageRegistry[slug].title : slug;
}
