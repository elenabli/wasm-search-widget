import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: 'playwright-tests',
  retries: 1,
  workers: 1,
  reporter: 'html',
  timeout: 5000, // 5 seconds

  use: {
    // Base URL to use in actions like `await page.goto('/')`.
    baseURL: 'http://127.0.0.1:8899',

    // Collect trace when retrying the failed test.
    trace: 'on-first-retry',
  },
  // Configure projects for major browsers.
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
  ],
  // Run your local dev server before starting the tests.
  webServer: {
    command: 'python3 -m http.server 8899',
    url: 'http://127.0.0.1:8899',
  },
});