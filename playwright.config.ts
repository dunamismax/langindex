import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
  testDir: "tests/playwright",
  webServer: {
    command: "pnpm preview --host 127.0.0.1 --port 4321",
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
