import { expect, test, type Page } from '@playwright/test';
import { disconnectNetwork } from './offline';

async function source(page: Page, code: string, className = 'Main') {
  const editor = page.getByRole('textbox', { name: 'Java source code' });
  await editor.fill(code);
  await page.getByRole('textbox', { name: 'Main class' }).fill(className);
}

test('compiles and runs Java 25 locally, including examples and cached offline execution', async ({
  page,
  browserName,
}) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));
  const requests: string[] = [];
  page.on('request', (request) => requests.push(`${request.method()} ${request.url()}`));
  await page.goto('./');
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
  await expect(page.locator('#status')).toContainText('Finished successfully', { timeout: 1000 });
  await expect(page.getByLabel('Console output')).toContainText('Hello, world! ☕');
  await expect(page.getByLabel('Console output')).toContainText('Cup 3 of Java');
  await page.screenshot({
    path: `test-results/playground-desktop-${test.info().project.name}.png`,
    fullPage: true,
  });

  for (const [example, output] of [
    ['collections', 'RISTRETTO'],
    ['records', 'Ristretto with 2 shots'],
  ]) {
    await page.getByLabel('Start with').selectOption(example);
    await page.getByRole('button', { name: 'Run', exact: true }).click();
    await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
      timeout: 755_000,
    });
    await expect(page.locator('#status')).toContainText('Finished successfully', { timeout: 1000 });
    await expect(page.getByLabel('Console output')).toContainText(output);
  }

  await disconnectNetwork(page.context(), browserName);
  await source(
    page,
    'package example; public class Main { static class Nested { static int value = 42; } public static void main(String[] args) { System.out.println("cached " + Nested.value); } }',
    'example.Main',
  );
  await page.getByRole('button', { name: 'Compile', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
  await expect(page.locator('#status')).toContainText('Compiled successfully', { timeout: 1000 });
  await expect(page.getByLabel('Console output')).toContainText('2 classes generated');
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
  await expect(page.locator('#status')).toContainText('Finished successfully', { timeout: 1000 });
  await expect(page.getByLabel('Console output')).toContainText('cached 42');
  expect(errors).toEqual([]);
  expect(
    requests
      .filter((request) => !request.startsWith('GET blob:'))
      .every((request) => request.startsWith(`GET ${test.info().project.use.baseURL}`)),
  ).toBe(true);
});

test('reports compilation and runtime errors and recovers after stopping a program', async ({
  page,
}) => {
  await page.goto('./');
  await source(page, 'public class Main { this is invalid Java; }');
  await page.getByRole('button', { name: 'Compile', exact: true }).click();
  await expect(page.locator('#status')).toHaveAttribute('data-state', 'error', {
    timeout: 755_000,
  });
  await expect(page.getByLabel('Console output')).toContainText('Main.java:1:');
  await source(
    page,
    'public class Main { public static void main(String[] args) { throw new IllegalStateException("test failure"); } }',
  );
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.locator('#status')).toHaveAttribute('data-state', 'error', {
    timeout: 755_000,
  });
  await expect(page.getByLabel('Console output')).toContainText('test failure');

  await source(
    page,
    'public class Main { public static void main(String[] args) { System.out.print("started"); while (true) {} } }',
  );
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.getByLabel('Console output')).toContainText('started', { timeout: 755_000 });
  await page.getByRole('button', { name: 'Stop', exact: true }).click();
  await expect(page.locator('#status')).toContainText('Stopped');
  await source(
    page,
    'public class Main { public static void main(String[] args) { System.out.print("fresh"); } }',
  );
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
  await expect(page.locator('#status')).toContainText('Finished successfully', { timeout: 1000 });
  await expect(page.getByLabel('Console output')).toHaveText('fresh');
});
