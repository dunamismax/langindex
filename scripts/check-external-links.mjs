import { lstat, readdir, readFile } from "node:fs/promises";
import { execFile } from "node:child_process";
import path from "node:path";
import { promisify } from "node:util";

const repoRoot = process.cwd();
const execFileAsync = promisify(execFile);
const rootEntries = ["src", "docs", "README.md", "CONTRIBUTING.md"];
const roots = rootEntries.map((entry) => path.join(repoRoot, entry));
const args = new Set(process.argv.slice(2));
const mode = args.has("--all")
  ? "all"
  : args.has("--changed")
    ? "changed"
    : (process.env.EXTERNAL_LINK_MODE ?? "changed");
const baseRef = process.env.EXTERNAL_LINK_BASE ?? "origin/main";
const delayMs = Number(process.env.EXTERNAL_LINK_DELAY_MS ?? 300);
const timeoutMs = Number(process.env.EXTERNAL_LINK_TIMEOUT_MS ?? 10000);
const errors = [];

if (args.has("--help") || args.has("-h")) {
  console.log(`Usage: node scripts/check-external-links.mjs [--changed|--all]

Default mode is --changed. It checks only external URLs added by the
current git diff, staged diff, unpushed commits against ${baseRef}, and
untracked content files.

Use --all or EXTERNAL_LINK_MODE=all for a full repository audit.
Use EXTERNAL_LINK_BASE=<ref> to compare committed branch changes against
a different base ref in changed mode.`);
  process.exit(0);
}

if (!["changed", "all"].includes(mode)) {
  console.error(
    `Invalid EXTERNAL_LINK_MODE '${mode}'. Expected 'changed' or 'all'.`,
  );
  process.exit(1);
}

async function listTextFiles(filePath) {
  const stat = await lstat(filePath);
  if (stat.isFile()) {
    return /\.(md|mdx|mjs|ts|js|json|yml|yaml)$/.test(filePath)
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

function addUrlSource(urlSources, url, source) {
  if (shouldSkip(url)) return;
  const sources = urlSources.get(url) ?? [];
  if (!sources.includes(source)) sources.push(source);
  urlSources.set(url, sources);
}

async function git(args, options = {}) {
  const { stdout } = await execFileAsync("git", args, {
    cwd: repoRoot,
    maxBuffer: 20 * 1024 * 1024,
    ...options,
  });
  return stdout;
}

function rootPathspecs() {
  return ["--", ...rootEntries];
}

function addUrlsFromDiff(urlSources, diffText, label) {
  let currentFile = null;
  for (const line of diffText.split("\n")) {
    if (line.startsWith("+++ b/")) {
      currentFile = line.slice("+++ b/".length);
      continue;
    }
    if (line.startsWith("+++ /dev/null")) {
      currentFile = null;
      continue;
    }
    if (!currentFile || !line.startsWith("+") || line.startsWith("+++")) {
      continue;
    }
    for (const url of extractExternalUrls(line.slice(1))) {
      addUrlSource(urlSources, url, `${currentFile} (${label})`);
    }
  }
}

async function addUrlsFromFile(urlSources, relativePath, label) {
  if (!/\.(md|mdx|mjs|ts|js|json|yml|yaml)$/.test(relativePath)) return;
  const text = await readFile(path.join(repoRoot, relativePath), "utf8");
  for (const url of extractExternalUrls(text)) {
    addUrlSource(urlSources, url, `${relativePath} (${label})`);
  }
}

async function committedDiffBase() {
  try {
    return (await git(["merge-base", "HEAD", baseRef])).trim();
  } catch {
    return null;
  }
}

async function collectChangedExternalUrls() {
  const urlSources = new Map();
  const pathspecs = rootPathspecs();
  const base = await committedDiffBase();

  if (base) {
    const diffText = await git([
      "diff",
      "--no-ext-diff",
      "--unified=0",
      `${base}..HEAD`,
      ...pathspecs,
    ]);
    addUrlsFromDiff(urlSources, diffText, `${baseRef}..HEAD`);
  }

  const stagedDiff = await git([
    "diff",
    "--cached",
    "--no-ext-diff",
    "--unified=0",
    ...pathspecs,
  ]);
  addUrlsFromDiff(urlSources, stagedDiff, "staged");

  const workingDiff = await git([
    "diff",
    "--no-ext-diff",
    "--unified=0",
    ...pathspecs,
  ]);
  addUrlsFromDiff(urlSources, workingDiff, "working tree");

  const untracked = await git([
    "ls-files",
    "--others",
    "--exclude-standard",
    ...pathspecs,
  ]);
  for (const relativePath of untracked.split("\n").filter(Boolean)) {
    await addUrlsFromFile(urlSources, relativePath, "untracked");
  }

  return urlSources;
}

async function collectAllExternalUrls() {
  const urlSources = new Map();
  for (const root of roots) {
    for (const file of await listTextFiles(root)) {
      const text = await readFile(file, "utf8");
      const relative = path.relative(repoRoot, file);
      for (const url of extractExternalUrls(text)) {
        addUrlSource(urlSources, url, relative);
      }
    }
  }
  return urlSources;
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
    parsed.hostname === "docwiki.embarcadero.com" ||
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

async function curlHeadWithTimeout(url) {
  await execFileAsync(
    "curl",
    [
      "-fsSIL",
      "--max-time",
      String(Math.max(1, Math.ceil(timeoutMs / 1000))),
      "-A",
      "LangIndex link checker (+https://langindex.dev)",
      url,
    ],
    { timeout: timeoutMs + 1000 },
  );
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
    try {
      await curlHeadWithTimeout(url);
    } catch {
      errors.push(`${url} failed: ${error.message} (${sources.join(", ")})`);
    }
  }
}

const urlSources =
  mode === "all"
    ? await collectAllExternalUrls()
    : await collectChangedExternalUrls();

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
  `Checked ${urlSources.size} external links in ${mode} mode with ${delayMs}ms delay.`,
);
