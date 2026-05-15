import { lstat, readdir, readFile } from "node:fs/promises";
import path from "node:path";

const repoRoot = process.cwd();
const roots = ["src", "docs", "README.md", "CONTRIBUTING.md"].map((entry) =>
  path.join(repoRoot, entry),
);
const delayMs = Number(process.env.EXTERNAL_LINK_DELAY_MS ?? 300);
const timeoutMs = Number(process.env.EXTERNAL_LINK_TIMEOUT_MS ?? 10000);
const errors = [];

async function listTextFiles(filePath) {
  const stat = await lstat(filePath);
  if (stat.isFile()) {
    return /\.(astro|md|mdx|mjs|ts|js|json|yml|yaml)$/.test(filePath)
      ? [filePath]
      : [];
  }

  const entries = await readdir(filePath, { withFileTypes: true });
  const files = await Promise.all(
    entries
      .filter((entry) => !["node_modules", "dist", ".git"].includes(entry.name))
      .map((entry) => listTextFiles(path.join(filePath, entry.name))),
  );

  return files.flat();
}

function extractExternalUrls(text) {
  const urls = new Set();
  const patterns = [
    /https?:\/\/[^\s<>"'`)\]}]+/g,
    /\b(?:href|src)=["'](https?:\/\/[^"']+)["']/gi,
  ];

  for (const pattern of patterns) {
    for (const match of text.matchAll(pattern)) {
      urls.add((match[1] ?? match[0]).replace(/[.,;:]+$/, ""));
    }
  }

  return urls;
}

function shouldSkip(url) {
  let parsed;
  try {
    parsed = new URL(url);
  } catch {
    return false;
  }

  return (
    ["localhost", "127.0.0.1"].includes(parsed.hostname) ||
    parsed.hostname === "example.org" ||
    parsed.hostname.endsWith(".example.org") ||
    parsed.hostname === "example.com" ||
    parsed.hostname.endsWith(".example.com") ||
    parsed.hostname === "langindex.dev" ||
    parsed.hostname.endsWith(".langindex.dev") ||
    parsed.hostname === "sitemaps.org" ||
    parsed.hostname.endsWith(".sitemaps.org") ||
    url.includes("/example/") ||
    url.includes("{") ||
    url.includes("}")
  );
}

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function fetchWithTimeout(url, method) {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    return await fetch(url, {
      method,
      redirect: "follow",
      signal: controller.signal,
      headers: {
        "user-agent": "LangIndex link checker (+https://langindex.dev)",
      },
    });
  } finally {
    clearTimeout(timeout);
  }
}

async function checkUrl(url, sources) {
  await sleep(delayMs);

  try {
    let response = await fetchWithTimeout(url, "HEAD");
    if ([403, 405].includes(response.status)) {
      response = await fetchWithTimeout(url, "GET");
    }
    if (response.status >= 400) {
      errors.push(`${url} returned ${response.status} (${sources.join(", ")})`);
    }
  } catch (error) {
    errors.push(`${url} failed: ${error.message} (${sources.join(", ")})`);
  }
}

const urlSources = new Map();
for (const root of roots) {
  for (const file of await listTextFiles(root)) {
    const text = await readFile(file, "utf8");
    for (const url of extractExternalUrls(text)) {
      if (shouldSkip(url)) continue;
      const relative = path.relative(repoRoot, file);
      const sources = urlSources.get(url) ?? [];
      sources.push(relative);
      urlSources.set(url, sources);
    }
  }
}

for (const [url, sources] of [...urlSources].sort(([a], [b]) =>
  a.localeCompare(b),
)) {
  await checkUrl(url, sources);
}

if (errors.length > 0) {
  console.error("External link check failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log(
  `Checked ${urlSources.size} external links with ${delayMs}ms delay.`,
);
