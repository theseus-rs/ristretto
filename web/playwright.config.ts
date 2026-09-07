import { defineConfig, devices } from '@playwright/test';

const baseURL = process.env.PLAYGROUND_BASE_URL ?? 'http://127.0.0.1:4173/ristretto/playground/';

export default defineConfig({
  testDir: './tests/browser',
  timeout: 3_600_000,
  expect: { timeout: 5_000 },
  fullyParallel: false,
  workers: 1,
  retries: 0,
  reporter: 'list',
  use: {
    baseURL,
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
  },
  projects: [
    { name: 'chromium', testIgnore: '**/firefox.spec.ts', use: { ...devices['Desktop Chrome'] } },
    {
      name: 'firefox',
      testMatch: ['**/firefox.spec.ts', '**/appearance.spec.ts'],
      use: { ...devices['Desktop Firefox'] },
    },
    { name: 'webkit', testIgnore: '**/firefox.spec.ts', use: { ...devices['Desktop Safari'] } },
  ],
  webServer: process.env.PLAYGROUND_BASE_URL
    ? undefined
    : {
        command: 'npm run preview -- --port 4173 --strictPort',
        url: 'http://127.0.0.1:4173/ristretto/playground/',
        reuseExistingServer: !process.env.CI,
      },
});
