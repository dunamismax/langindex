import { lstat, readdir, readFile } from "node:fs/promises";
import path from "node:path";

const repoRoot = process.cwd();
const distRoot = path.join(repoRoot, "dist");
const errors = [];

async function pathExists(filePath) {
  try {
    await lstat(filePath);
    return true;
  } catch {
    return false;
  }
}

async function listFiles(dir) {
  const entries = await readdir(dir, { withFileTypes: true });
  const files = await Promise.all(
    entries.map(async (entry) => {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) return listFiles(fullPath);
      if (entry.isFile() && /\.(html|json|xml|txt|css)$/.test(entry.name)) {
        return [fullPath];
      }
      return [];
    }),
  );

  return files.flat();
}

function routeToFile(pathname) {
  const decoded = decodeURIComponent(pathname);
  if (decoded.endsWith("/")) return path.join(distRoot, decoded, "index.html");
  const withExtension = path.join(distRoot, decoded);
  const withoutExtension = path.join(distRoot, decoded, "index.html");
  return { withExtension, withoutExtension };
}

function fileToPathname(filePath) {
  const relativePath = path
    .relative(distRoot, filePath)
    .split(path.sep)
    .join("/");
  if (relativePath === "index.html") return "/";
  if (relativePath.endsWith("/index.html")) {
    return `/${relativePath.slice(0, -"index.html".length)}`;
  }
  return `/${relativePath}`;
}

function extractLinks(text) {
  const links = [];
  const patterns = [
    /\b(?:href|src)=["']([^"']+)["']/gi,
    /\burl\(["']?([^"')]+)["']?\)/gi,
  ];

  for (const pattern of patterns) {
    for (const match of text.matchAll(pattern)) links.push(match[1]);
  }

  return links;
}

function extractAnchors(text) {
  const anchors = new Set();
  for (const match of text.matchAll(/\bid=["']([^"']+)["']/gi)) {
    anchors.add(match[1]);
  }
  for (const match of text.matchAll(/\bname=["']([^"']+)["']/gi)) {
    anchors.add(match[1]);
  }
  return anchors;
}

async function resolveTargetFile(pathname) {
  const target = routeToFile(pathname);
  if (typeof target === "string") {
    return (await pathExists(target)) ? target : null;
  }

  if (await pathExists(target.withExtension)) return target.withExtension;
  if (await pathExists(target.withoutExtension)) return target.withoutExtension;
  return null;
}

async function validateHash(sourceFile, targetFile, url) {
  if (!url.hash) return;
  if (path.extname(targetFile) && path.extname(targetFile) !== ".html") return;

  const id = decodeURIComponent(url.hash.slice(1));
  if (!id) return;

  const targetText = await readFile(targetFile, "utf8");
  if (!extractAnchors(targetText).has(id)) {
    errors.push(
      `${path.relative(repoRoot, sourceFile)} links to missing anchor ${url.pathname}${url.hash}`,
    );
  }
}

async function validateInternalLink(sourceFile, rawLink) {
  if (
    rawLink.startsWith("mailto:") ||
    rawLink.startsWith("tel:") ||
    rawLink.startsWith("data:")
  ) {
    return;
  }

  let url;
  try {
    url = new URL(
      rawLink,
      `https://langindex.dev${fileToPathname(sourceFile)}`,
    );
  } catch {
    errors.push(
      `${path.relative(repoRoot, sourceFile)} has invalid link: ${rawLink}`,
    );
    return;
  }

  if (url.origin !== "https://langindex.dev") return;

  const targetFile = await resolveTargetFile(url.pathname);
  if (!targetFile) {
    errors.push(
      `${path.relative(repoRoot, sourceFile)} links to missing ${url.pathname}`,
    );
    return;
  }

  await validateHash(sourceFile, targetFile, url);
}

if (!(await pathExists(distRoot))) {
  console.error(
    "dist/ is missing. Run `pnpm build` before checking internal links.",
  );
  process.exit(1);
}

const files = await listFiles(distRoot);
for (const file of files) {
  const text = await readFile(file, "utf8");
  for (const link of extractLinks(text)) await validateInternalLink(file, link);
}

if (errors.length > 0) {
  console.error("Internal link check failed:");
  for (const error of errors) console.error(`- ${error}`);
  process.exit(1);
}

console.log(`Checked internal links in ${files.length} built files.`);
