import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "tests/playwright",
  webServer: {
    command:
      "LANGINDEX_SITE_ADDR=127.0.0.1:4321 LANGINDEX_SITE_LOG=warn cargo run -p langindex-site",
    url: "http://127.0.0.1:4321",
    reuseExistingServer: false,
  },
  use: {
    baseURL: "http://127.0.0.1:4321",
    trace: "on-first-retry",
  },
  projects: [
    {
      name: "desktop-chrome",
      use: { ...devices["Desktop Chrome"] },
    },
    {
      name: "mobile-chrome",
      use: { ...devices["Pixel 7"] },
    },
  ],
});
