import { expect, test } from "@playwright/test";

test("homepage exposes search and language discovery", async ({ page }) => {
  await page.goto("/");

  await expect(page.getByRole("heading", { level: 1 })).toContainText(
    /field guide/i,
  );
  await expect(
    page.getByRole("heading", { name: "Seed languages" }),
  ).toBeVisible();
  await expect(page.locator("pagefind-searchbox")).toBeVisible();
  await expect(
    page.getByRole("link", { name: "Open Rust" }).first(),
  ).toBeVisible();
});

test("mobile navigation opens and exposes primary links", async ({ page }) => {
  test.skip(
    test.info().project.name !== "mobile-chrome",
    "Mobile navigation is only visible in the mobile project.",
  );

  await page.goto("/");

  const toggle = page.getByRole("button", { name: /toggle navigation/i });
  await expect(toggle).toBeVisible();
  await expect(toggle).toHaveAttribute("aria-expanded", "false");

  await toggle.click();
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
  await expect(page.getByRole("navigation", { name: "Mobile" })).toBeVisible();
  await expect(
    page.getByRole("navigation", { name: "Mobile" }).getByRole("link", {
      name: "Languages",
    }),
  ).toBeVisible();
});

test("language filters narrow results and stay visible", async ({ page }) => {
  await page.goto("/languages/");

  const search = page.getByLabel("Search", { exact: true });
  await expect(search).toBeVisible();
  await search.fill("borrowing");

  await expect(page.getByRole("link", { name: "Open Rust" })).toBeVisible();
  await expect(page.getByRole("link", { name: "Open Go" })).toHaveCount(0);
  await expect(page.getByText("Showing 1 language")).toBeVisible();
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
