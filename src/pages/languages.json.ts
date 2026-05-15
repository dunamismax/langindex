import { getCollection } from "astro:content";

export async function GET() {
  const languages = (await getCollection("languages"))
    .sort((a, b) => a.data.title.localeCompare(b.data.title))
    .map((language) => ({
      title: language.data.title,
      slug: language.data.slug,
      status: language.data.status,
      summary: language.data.summary,
      firstReleased: language.data.firstReleased ?? null,
      creators: language.data.creators,
      paradigms: language.data.paradigms,
      typing: language.data.typing,
      memory: language.data.memory,
      runtime: language.data.runtime,
      officialSite: language.data.officialSite,
      repository: language.data.repository ?? null,
      packageManagers: language.data.packageManagers,
      comparedWith: language.data.comparedWith,
      bestFor: language.data.bestFor,
      poorFit: language.data.poorFit,
      sources: language.data.sources,
      lastVerified: language.data.lastVerified.toISOString().slice(0, 10),
      url: `/languages/${language.data.slug}`,
    }));

  return new Response(JSON.stringify({ languages }, null, 2), {
    headers: {
      "content-type": "application/json; charset=utf-8",
    },
  });
}
