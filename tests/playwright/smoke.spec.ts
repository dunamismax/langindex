import { expect, test } from "@playwright/test";

test("homepage exposes search and language discovery", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByRole("heading", { level: 1 })).toContainText(
    /understand programming languages/i,
  );
  await expect(
    page.getByRole("heading", { name: "Featured languages" }),
  ).toBeVisible();
  await expect(page.getByLabel("Search languages and topics")).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Open Rust" }).first(),
  ).toBeVisible();
});

test("homepage live search finds supporting reference pages", async ({
  page,
}) => {
  await page.goto("/");

  await page.getByLabel("Search languages and topics").fill("ownership");
  await expect(page.getByRole("link", { name: /ownership/i })).toBeVisible();
  await expect(page.getByText(/matches/)).toBeVisible();
});

test("mobile navigation opens and exposes primary links", async ({ page }) => {
  test.skip(
    test.info().project.name !== "mobile-chrome",
    "Mobile navigation is only visible in the mobile project.",
  );

  await page.goto("/");

  const toggle = page.getByRole("button", { name: /navigation menu/i });
  await expect(toggle).toBeVisible();
  await expect(toggle).toHaveAttribute("aria-expanded", "false");
  await expect(
    page.getByRole("navigation", { name: "Primary" }),
  ).toBeHidden();

  await toggle.click();
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
  await expect(page.getByRole("navigation", { name: "Mobile" })).toBeVisible();
  await expect(
    page.getByRole("navigation", { name: "Mobile" }).getByRole("link", {
      name: "Languages",
    }),
  ).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(toggle).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByRole("navigation", { name: "Mobile" })).toBeHidden();
});

test("language filters narrow results and stay visible", async ({ page }) => {
  await page.goto("/languages/");

  const search = page.getByLabel("Search", { exact: true });
  await expect(search).toBeVisible();
  await search.fill("rustup");
  await page.getByRole("button", { name: "Apply filters" }).click();

  await expect(page.getByRole("link", { name: "Open Rust" })).toBeVisible();
  await expect(page.getByRole("link", { name: "Open Go" })).toHaveCount(0);
  await expect(page.getByText("Showing 1 language of")).toBeVisible();
});

test("language profile shows sources and comparison links", async ({
  page,
}) => {
  await page.goto("/languages/rust/");

  await expect(
    page.getByRole("heading", { name: "Rust", level: 1 }),
  ).toBeVisible();
  await expect(page.getByRole("heading", { name: "Sources" })).toBeVisible();
  await expect(
    page.getByRole("link", { name: /Rust vs Go/i }).first(),
  ).toBeVisible();
});

test("language cards and comparison tables fit narrow viewports", async ({
  page,
}) => {
  await page.goto("/languages/");
  await expect(
    page.getByRole("link", { name: "Open TypeScript" }),
  ).toBeVisible();

  await page.goto("/comparisons/rust-vs-go/");
  await expect(
    page.getByRole("heading", { name: "Rust vs Go", level: 1 }),
  ).toBeVisible();

  const hasHorizontalOverflow = await page.evaluate(
    () => document.documentElement.scrollWidth > window.innerWidth,
  );
  expect(hasHorizontalOverflow).toBe(false);
});

test("comparison page renders sourced comparison content", async ({ page }) => {
  await page.goto("/comparisons/rust-vs-go/");

  await expect(
    page.getByRole("heading", { name: "Rust vs Go", level: 1 }),
  ).toBeVisible();
  await expect(page.getByRole("heading", { name: "Sources" })).toBeVisible();
});

test("generated routes are available from the Rust service", async ({
  page,
}) => {
  const languages = await page.request.get("/languages.json");
  expect(languages.ok()).toBe(true);
  expect(JSON.stringify(await languages.json())).toContain('"slug":"rust"');

  const search = await page.request.get("/search.json");
  expect(search.ok()).toBe(true);
  expect(await search.text()).toContain("/languages/rust/");

  const rss = await page.request.get("/rss.xml");
  expect(rss.ok()).toBe(true);
  expect(await rss.text()).toContain("<rss");

  const sitemap = await page.request.get("/sitemap.xml");
  expect(sitemap.ok()).toBe(true);
  expect(await sitemap.text()).toContain("/languages/rust");

  const missing = await page.request.get("/missing");
  expect(missing.status()).toBe(404);
});
