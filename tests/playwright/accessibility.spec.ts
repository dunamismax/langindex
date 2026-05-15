import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

const routes = [
  { name: "homepage", url: "/" },
  { name: "languages index", url: "/languages/" },
  { name: "language profile", url: "/languages/rust/" },
  { name: "comparisons index", url: "/comparisons/" },
  { name: "comparison page", url: "/comparisons/rust-vs-go/" },
  { name: "guides index", url: "/guides/" },
  { name: "concepts index", url: "/concepts/" },
  { name: "about", url: "/about/" },
  { name: "contribute", url: "/contribute/" },
];

for (const route of routes) {
  test(`${route.name} has no detectable axe violations`, async ({ page }) => {
    await page.goto(route.url);
    const results = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa"])
      .analyze();
    expect(results.violations, JSON.stringify(results.violations, null, 2)).toEqual([]);
  });
}

test("theme toggle persists across reload", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
  await page.getByRole("button", { name: /switch to light theme/i }).click();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
  await page.reload();
  await expect(page.locator("html")).toHaveAttribute("data-theme", "light");
});
