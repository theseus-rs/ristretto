import { expect, test } from '@playwright/test';

test('follows the system and remembers a local theme override without changing code', async ({
  page,
}) => {
  await page.emulateMedia({ colorScheme: 'light' });
  await page.goto('./');
  const root = page.locator('html');
  const picker = page.getByRole('combobox', { name: 'Color theme' });
  const editor = page.getByRole('textbox', { name: 'Java source code' });
  await expect(picker).toHaveValue('system');
  await expect(root).toHaveAttribute('data-theme', 'light');
  await expect(page.locator('.cm-editor')).toHaveCSS('background-color', 'rgb(255, 255, 255)');
  const source = 'public class Main { int value = 42; String text = "coffee"; }';
  await editor.fill(source);
  await page.emulateMedia({ colorScheme: 'dark' });
  await expect(root).toHaveAttribute('data-theme', 'dark');
  await expect(page.locator('.cm-editor')).toHaveCSS('background-color', 'rgb(30, 31, 34)');
  await expect(editor).toHaveText(source);
  await picker.selectOption('light');
  await expect
    .poll(() => page.evaluate(() => localStorage.getItem('ristretto-playground-source-v1')))
    .toContain('coffee');
  await page.reload();
  await expect(picker).toHaveValue('light');
  await expect(root).toHaveAttribute('data-theme', 'light');
  await expect(editor).toHaveText(source);
  await picker.selectOption('dark');
  await page.emulateMedia({ colorScheme: 'light' });
  await expect(root).toHaveAttribute('data-theme', 'dark');
  await page.reload();
  await expect(picker).toHaveValue('dark');
  await picker.selectOption('system');
  await expect(root).toHaveAttribute('data-theme', 'light');
  expect(await page.evaluate(() => localStorage.getItem('ristretto-playground-theme'))).toBeNull();
  await page.emulateMedia({ colorScheme: 'dark' });
  await expect(root).toHaveAttribute('data-theme', 'dark');
});

test('uses minimal side margins on wide and mobile screens in both themes', async ({
  page,
}, testInfo) => {
  await page.setViewportSize({ width: 1920, height: 1080 });
  await page.goto('./');
  for (const width of [1920, 390]) {
    await page.setViewportSize({ width, height: width === 390 ? 844 : 1080 });
    for (const colorScheme of ['light', 'dark'] as const) {
      await page.emulateMedia({ colorScheme });
      const bounds = await page.getByRole('region', { name: 'Java playground' }).boundingBox();
      expect(bounds).not.toBeNull();
      expect(bounds!.x).toBeLessThanOrEqual(10);
      expect(width - bounds!.x - bounds!.width).toBeLessThanOrEqual(10);
      expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(
        width,
      );
      await page.screenshot({
        path: testInfo.outputPath(`appearance-${width}-${colorScheme}.png`),
        fullPage: true,
        animations: 'disabled',
      });
    }
  }
});

test('can switch themes when browser storage is unavailable', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));
  await page.addInitScript(() => {
    for (const method of ['getItem', 'setItem', 'removeItem']) {
      Object.defineProperty(Storage.prototype, method, {
        value() {
          throw new DOMException('Storage unavailable', 'SecurityError');
        },
      });
    }
  });
  await page.emulateMedia({ colorScheme: 'light' });
  await page.goto('./');
  await page.getByRole('combobox', { name: 'Color theme' }).selectOption('dark');
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
  await page.reload();
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'light');
  expect(errors).toEqual([]);
});
