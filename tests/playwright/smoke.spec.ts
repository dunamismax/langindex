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

test("language filters narrow results", async ({ page }) => {
  await page.goto("/languages/");

  await page.getByLabel("Search", { exact: true }).fill("ownership");

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

test("comparison page renders sourced comparison content", async ({ page }) => {
  await page.goto("/comparisons/rust-vs-go/");

  await expect(
    page.getByRole("heading", { name: "Rust vs Go", level: 1 }),
  ).toBeVisible();
  await expect(page.getByRole("heading", { name: "Sources" })).toBeVisible();
});
