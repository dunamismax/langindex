import { getCollection } from "astro:content";

type SitemapEntry = {
  loc: string;
  lastmod?: Date;
};

const staticPaths = [
  "/",
  "/languages/",
  "/comparisons/",
  "/guides/",
  "/concepts/",
  "/about/",
  "/contribute/",
  "/languages.json",
];

function formatDate(date: Date) {
  return date.toISOString().slice(0, 10);
}

function escapeXml(value: string) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");
}

export async function GET({ site }: { site: URL }) {
  const [languages, comparisons, guides, concepts] = await Promise.all([
    getCollection("languages"),
    getCollection("comparisons"),
    getCollection("guides"),
    getCollection("concepts"),
  ]);

  const entries: SitemapEntry[] = [
    ...staticPaths.map((path) => ({ loc: new URL(path, site).toString() })),
    ...languages.map((entry) => ({
      loc: new URL(`/languages/${entry.data.slug}/`, site).toString(),
      lastmod: entry.data.lastVerified,
    })),
    ...comparisons.map((entry) => ({
      loc: new URL(`/comparisons/${entry.data.slug}/`, site).toString(),
      lastmod: entry.data.lastVerified,
    })),
    ...guides.map((entry) => ({
      loc: new URL(`/guides/${entry.data.slug}/`, site).toString(),
      lastmod: entry.data.lastVerified,
    })),
    ...concepts.map((entry) => ({
      loc: new URL(`/concepts/${entry.data.slug}/`, site).toString(),
      lastmod: entry.data.lastVerified,
    })),
  ].sort((a, b) => a.loc.localeCompare(b.loc));

  const body = `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${entries
  .map(
    (entry) => `  <url>
    <loc>${escapeXml(entry.loc)}</loc>${
      entry.lastmod
        ? `\n    <lastmod>${formatDate(entry.lastmod)}</lastmod>`
        : ""
    }
  </url>`,
  )
  .join("\n")}
</urlset>
`;

  return new Response(body, {
    headers: {
      "content-type": "application/xml; charset=utf-8",
    },
  });
}
