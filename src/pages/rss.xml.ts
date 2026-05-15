import { getCollection } from "astro:content";

function escapeXml(value: string) {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&apos;");
}

type FeedEntry = {
  title: string;
  url: string;
  description: string;
  date: Date;
  category: string;
};

export async function GET({ site }: { site: URL }) {
  const [languages, comparisons, guides, concepts] = await Promise.all([
    getCollection("languages"),
    getCollection("comparisons"),
    getCollection("guides"),
    getCollection("concepts"),
  ]);

  const entries: FeedEntry[] = [
    ...languages.map((entry) => ({
      title: entry.data.title,
      url: new URL(`/languages/${entry.data.slug}/`, site).toString(),
      description: entry.data.summary,
      date: entry.data.lastVerified,
      category: "Language",
    })),
    ...comparisons.map((entry) => ({
      title: entry.data.title,
      url: new URL(`/comparisons/${entry.data.slug}/`, site).toString(),
      description: entry.data.summary,
      date: entry.data.lastVerified,
      category: "Comparison",
    })),
    ...guides.map((entry) => ({
      title: entry.data.title,
      url: new URL(`/guides/${entry.data.slug}/`, site).toString(),
      description: entry.data.summary,
      date: entry.data.lastVerified,
      category: "Guide",
    })),
    ...concepts.map((entry) => ({
      title: entry.data.title,
      url: new URL(`/concepts/${entry.data.slug}/`, site).toString(),
      description: entry.data.summary,
      date: entry.data.lastVerified,
      category: "Concept",
    })),
  ].sort((a, b) => b.date.getTime() - a.date.getTime());

  const buildDate =
    entries[0]?.date.toUTCString() ?? new Date().toUTCString();
  const siteUrl = site.toString().replace(/\/$/, "");

  const body = `<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">
  <channel>
    <title>LangIndex updates</title>
    <link>${siteUrl}</link>
    <atom:link href="${siteUrl}/rss.xml" rel="self" type="application/rss+xml" />
    <description>Verified programming language references, comparisons, guides, and concepts.</description>
    <language>en-us</language>
    <lastBuildDate>${buildDate}</lastBuildDate>
${entries
  .map(
    (entry) => `    <item>
      <title>${escapeXml(`[${entry.category}] ${entry.title}`)}</title>
      <link>${escapeXml(entry.url)}</link>
      <guid isPermaLink="true">${escapeXml(entry.url)}</guid>
      <description>${escapeXml(entry.description)}</description>
      <category>${escapeXml(entry.category)}</category>
      <pubDate>${entry.date.toUTCString()}</pubDate>
    </item>`,
  )
  .join("\n")}
  </channel>
</rss>
`;

  return new Response(body, {
    headers: {
      "content-type": "application/rss+xml; charset=utf-8",
    },
  });
}
