import { readdir, readFile } from "node:fs/promises";
import path from "node:path";
import { parse } from "yaml";

const repoRoot = process.cwd();
const contentRoot = path.join(repoRoot, "src", "content");
const requiredSourceCollections = new Set([
  "languages",
  "comparisons",
  "guides",
  "concepts",
]);
const today = new Date();
today.setHours(23, 59, 59, 999);

const errors = [];

async function listContentFiles(dir) {
  const entries = await readdir(dir, { withFileTypes: true });
  const files = await Promise.all(
    entries.map(async (entry) => {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        return listContentFiles(fullPath);
      }
      if (entry.isFile() && /\.(md|mdx)$/.test(entry.name)) {
        return [fullPath];
      }
      return [];
    }),
  );

  return files.flat();
}

function relative(filePath) {
  return path.relative(repoRoot, filePath);
}

function extractFrontmatter(filePath, text) {
  const match = text.match(/^---\r?\n([\s\S]*?)\r?\n---/);
  if (!match) {
    errors.push(`${relative(filePath)} is missing YAML frontmatter.`);
    return null;
  }

  try {
    return parse(match[1]);
  } catch (error) {
    errors.push(`${relative(filePath)} has invalid YAML: ${error.message}`);
    return null;
  }
}

function validateUrl(filePath, label, value) {
  if (typeof value !== "string" || value.length === 0) {
    errors.push(`${relative(filePath)} has a missing ${label}.`);
    return;
  }

  let url;
  try {
    url = new URL(value);
  } catch {
    errors.push(`${relative(filePath)} has an invalid ${label}: ${value}`);
    return;
  }

  if (!["http:", "https:"].includes(url.protocol)) {
    errors.push(
      `${relative(filePath)} has a non-web ${label}: ${url.toString()}`,
    );
  }
}

function validateLastVerified(filePath, value) {
  if (!value) {
    errors.push(`${relative(filePath)} is missing lastVerified.`);
    return;
  }

  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    errors.push(`${relative(filePath)} has invalid lastVerified: ${value}`);
    return;
  }

  if (date > today) {
    errors.push(`${relative(filePath)} has future lastVerified: ${value}`);
  }
}

function validateSources(filePath, collection, data) {
  const sources = data?.sources ?? [];

  if (requiredSourceCollections.has(collection) && sources.length === 0) {
    errors.push(`${relative(filePath)} needs at least one source.`);
    return;
  }

  if (!Array.isArray(sources)) {
    errors.push(`${relative(filePath)} sources must be a list.`);
    return;
  }

  sources.forEach((source, index) => {
    const prefix = `sources[${index}]`;
    if (!source?.title) {
      errors.push(`${relative(filePath)} is missing ${prefix}.title.`);
    }
    validateUrl(filePath, `${prefix}.url`, source?.url);
  });
}

const files = await listContentFiles(contentRoot);

for (const filePath of files) {
  const text = await readFile(filePath, "utf8");
  const data = extractFrontmatter(filePath, text);
  if (!data) {
    continue;
  }

  const collection = path.relative(contentRoot, filePath).split(path.sep)[0];
  validateLastVerified(filePath, data.lastVerified);
  validateSources(filePath, collection, data);
}

if (errors.length > 0) {
  console.error("Source validation failed:");
  for (const error of errors) {
    console.error(`- ${error}`);
  }
  process.exit(1);
}

console.log(`Validated sources for ${files.length} content files.`);
