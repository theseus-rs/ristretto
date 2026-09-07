import { expect, test, type Page } from '@playwright/test';

async function run(page: Page, body: string) {
  await page
    .getByRole('textbox', { name: 'Java source code' })
    .fill(
      `public class Main { public static void main(String[] args) throws Exception { ${body} } }`,
    );
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
}

test('preserves byte-wise UTF-8, stderr, EOF, empty output, and process exit', async ({ page }) => {
  await page.goto('./');
  await run(
    page,
    'for (byte b : "☕😀".getBytes("UTF-8")) System.out.write(b); System.out.flush(); System.err.print("stderr"); System.out.print(" args=" + args.length + " stdin=" + System.in.read());',
  );
  await expect(page.getByLabel('Console output')).toContainText('☕😀stderr args=0 stdin=-1');
  await expect(page.locator('#output .stderr')).toHaveText('stderr');
  await run(page, '');
  await expect(page.getByLabel('Console output')).toContainText('Program finished without output');
  await run(page, 'System.exit(0);');
  await expect(page.locator('#status')).toContainText('Exited with code 0', { timeout: 1000 });
  await run(page, 'System.exit(7);');
  await expect(page.getByLabel('Console output')).toContainText('exited unsuccessfully', {
    timeout: 1000,
  });
  await expect(page.locator('#status')).toHaveAttribute('data-state', 'error', {
    timeout: 755_000,
  });
});

test('enforces output and execution limits and remains usable', async ({ page }) => {
  await page.goto('./');
  await run(
    page,
    'String block = "x".repeat(8192); for (int i = 0; i < 129; i++) System.out.print(block);',
  );
  await expect(page.getByLabel('Console output')).toContainText('Output exceeded 1 MiB', {
    timeout: 1000,
  });
  expect((await page.getByLabel('Console output').textContent())!.length).toBeLessThan(
    1024 * 1024 + 200,
  );
  await page
    .getByRole('textbox', { name: 'Java source code' })
    .fill('public class Main { public static void main(String[] args) { while (true) {} } }');
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.locator('#status')).toContainText('Running…', { timeout: 755_000 });
  // Exercise the real main-thread watchdog while the worker is blocked in a Java loop.
  await expect(page.locator('#status')).toContainText('Time limit reached', { timeout: 40_000 });
  await run(page, 'System.out.print("recovered");');
  await expect(page.getByLabel('Console output')).toHaveText('recovered', { timeout: 1000 });
});

test('retries failed downloads and runs when persistent storage is unavailable', async ({
  page,
}) => {
  await page.addInitScript(() => {
    Object.defineProperty(window, 'localStorage', {
      get() {
        throw new DOMException('Disabled', 'SecurityError');
      },
    });
    Object.defineProperty(window, 'caches', {
      get() {
        throw new Error('Storage unavailable');
      },
    });
  });
  await page.goto('./');
  await page.route('**/runtime/*.wasm', (route) => route.fulfill({ status: 404, body: 'missing' }));
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.locator('#status')).toHaveAttribute('data-state', 'error', {
    timeout: 755_000,
  });
  await expect(page.getByLabel('Console output')).toContainText('404');
  await page.unroute('**/runtime/*.wasm');
  await run(page, 'System.out.println("without storage");');
  await expect(page.getByLabel('Console output')).toHaveText('without storage\n', {
    timeout: 1000,
  });
});

test('keeps the editor and controls usable on a narrow screen and autosaves', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('./');
  await expect(page.getByRole('button', { name: 'Run', exact: true })).toBeInViewport();
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(
    true,
  );
  await page.getByRole('textbox', { name: 'Java source code' }).fill('public class Saved {}');
  await page.getByRole('textbox', { name: 'Main class' }).fill('Saved');
  await page.getByLabel('Java version').selectOption('11');
  await expect
    .poll(() => page.evaluate(() => localStorage.getItem('ristretto-playground-source-v1')))
    .toContain('public class Saved');
  await page.reload();
  await expect(page.getByRole('textbox', { name: 'Java source code' })).toHaveText(
    'public class Saved {}',
  );
  await expect(page.getByRole('textbox', { name: 'Main class' })).toHaveValue('Saved');
  await expect(page.getByLabel('Java version')).toHaveValue('11');
  await page.screenshot({
    path: `test-results/playground-mobile-${test.info().project.name}.png`,
    fullPage: true,
  });
});
