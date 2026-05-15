export function GET({ site }: { site: URL }) {
  const sitemapUrl = new URL("/sitemap.xml", site).toString();

  return new Response(`User-agent: *\nAllow: /\n\nSitemap: ${sitemapUrl}\n`, {
    headers: {
      "content-type": "text/plain; charset=utf-8",
    },
  });
}
