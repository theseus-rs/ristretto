import { expect, test, type Page } from '@playwright/test';
import { disconnectNetwork } from './offline';

const targets = [
  {
    language: 'kotlin',
    name: 'Kotlin',
    file: 'Main.kts',
    feature: 'Null safety',
    invalid: 'val =',
    throwing: 'error("script failure")',
  },
  {
    language: 'groovy',
    name: 'Groovy',
    file: 'Main.groovy',
    feature: 'Closures',
    invalid: 'def =',
    throwing: 'throw new IllegalStateException("script failure")',
  },
  {
    language: 'scala',
    version: '2.13',
    name: 'Scala',
    file: 'Main.sc',
    feature: 'Case classes & pattern matching',
    invalid: 'val =',
    throwing: 'throw new IllegalStateException("script failure")',
  },
  {
    language: 'scala',
    version: '3',
    name: 'Scala',
    file: 'Main.sc',
    feature: 'Enums & pattern matching',
    invalid: 'val =',
    throwing: 'throw new IllegalStateException("script failure")',
  },
  {
    language: 'clojure',
    name: 'Clojure',
    file: 'Main.clj',
    feature: 'Functions & sequences',
    invalid: '(println',
    throwing: '(throw (Exception. "script failure"))',
  },
];
async function select(page: Page, target: (typeof targets)[number]) {
  await page.getByLabel('Language').selectOption(target.language);
  if (target.version) await page.getByLabel('Scala version').selectOption(target.version);
}
async function finish(page: Page) {
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
}

test('language controls, examples, saved drafts, and mobile layout', async ({ page }) => {
  await page.goto('./');
  await expect(page.locator('.workspace-footer')).not.toContainText('Powered by Ristretto');
  await expect(page.getByLabel('Language').locator('option')).toHaveText([
    'Java',
    'Kotlin',
    'Groovy',
    'Scala',
    'Clojure',
  ]);
  await page.getByLabel('Java version').selectOption('11');
  await expect(page.getByLabel('Java version')).toBeVisible();
  await page.getByRole('textbox', { name: 'Java source code' }).fill('class Saved {}');
  for (const target of targets) {
    await select(page, target);
    await expect(page.getByLabel('Java version')).toBeHidden();
    await expect(page.getByLabel('Java version')).toBeDisabled();
    await expect(page.getByLabel('Java version')).toHaveValue('11');
    await expect(page.locator('#filename')).toHaveText(target.file);
    await expect(page.getByRole('textbox', { name: 'Main class' })).toBeHidden();
    await expect(page.getByRole('button', { name: 'Check', exact: true })).toBeEnabled();
    await expect(page.getByLabel('Start with').locator('option')).toHaveText([
      'Hello, world',
      'Collections & transformations',
      target.feature,
    ]);
    await page.getByLabel('Start with').selectOption('features');
    const code = await page
      .getByRole('textbox', { name: `${target.name} source code` })
      .innerText();
    expect(code).not.toContain('public static void main');
    await page
      .getByRole('textbox', { name: `${target.name} source code` })
      .fill(
        code +
          '\n' +
          (target.language === 'clojure' ? ';' : '//') +
          ' saved ' +
          (target.version ?? target.language),
      );
  }
  await page.reload();
  await expect(page.getByLabel('Language')).toHaveValue('clojure');
  await expect(page.getByLabel('Start with')).toHaveValue('features');
  for (const target of targets) {
    await select(page, target);
    await expect(page.getByRole('textbox', { name: `${target.name} source code` })).toContainText(
      ' saved ' + (target.version ?? target.language),
    );
  }
  for (const width of [390, 900, 1280]) {
    await page.setViewportSize({ width, height: 900 });
    await expect(page.getByRole('button', { name: 'Run', exact: true })).toBeInViewport();
    expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(
      width,
    );
  }
  await page.getByLabel('Language').selectOption('java');
  await expect(page.getByLabel('Java version')).toBeEnabled();
  await expect(page.getByLabel('Java version')).toBeVisible();
  await expect(page.getByLabel('Java version')).toHaveValue('11');
  await expect(page.getByRole('textbox', { name: 'Java source code' })).toHaveText(
    'class Saved {}',
  );
  await expect(page.getByRole('button', { name: 'Compile', exact: true })).toBeEnabled();
});

for (const target of targets) {
  test(`${target.name} ${target.version ?? ''} script runs in the browser`, async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', (error) => errors.push(error.message));
    await page.goto('./');
    await select(page, target);
    await page.getByRole('button', { name: 'Run', exact: true }).click();
    await expect(page.getByLabel('Language')).toBeDisabled();
    await finish(page);
    await expect(page.locator('#status')).toContainText('Finished successfully');
    await expect(page.getByLabel('Console output')).toContainText('Hello, world! ☕');
    await expect(page.getByLabel('Console output')).toContainText(`Cup 3 of ${target.name}`);
    await expect(page.locator('#elapsed')).toHaveText(/^Run: \d+\.\d{2}s$/);
    await expect(page.getByLabel('Java version')).toBeHidden();
    await expect(page.getByLabel('Java version')).toBeDisabled();
    expect(errors).toEqual([]);
  });

  test(`${target.name} ${target.version ?? ''} checking, examples, errors, and offline recovery`, async ({
    page,
    browserName,
  }) => {
    test.skip(
      browserName === 'firefox',
      'The focused Firefox gate executes each actual language; full interactions run in Chromium and WebKit.',
    );
    await page.goto('./');
    await select(page, target);
    const editor = page.getByRole('textbox', { name: `${target.name} source code` });
    const output = page.getByLabel('Console output');
    await editor.fill(target.throwing);
    await page.getByRole('button', { name: 'Check', exact: true }).click();
    await finish(page);
    await expect(page.locator('#status')).toHaveAttribute('data-state', 'ready');
    await expect(output).not.toContainText('script failure');
    await expect(page.locator('#elapsed')).toHaveText(/^Check: \d+\.\d{2}s$/);
    await disconnectNetwork(page.context(), browserName);
    await page.getByRole('button', { name: 'Run', exact: true }).click();
    await finish(page);
    await expect(page.locator('#status')).toHaveAttribute('data-state', 'error');
    await expect(output).toContainText('script failure');
    await expect(page.locator('#elapsed')).toHaveText(/^Run: \d+\.\d{2}s$/);
    await editor.fill(target.invalid);
    await page.getByRole('button', { name: 'Check', exact: true }).click();
    await finish(page);
    await expect(page.locator('#status')).toHaveAttribute('data-state', 'error');
    for (const example of ['collections', 'features']) {
      await page.getByLabel('Start with').selectOption(example);
      await page.getByRole('button', { name: 'Run', exact: true }).click();
      await finish(page);
      await expect(page.locator('#status')).toContainText('Finished successfully');
      await expect(output).toContainText(example === 'collections' ? 'RISTRETTO' : 'Ristretto');
    }
  });
}

test('Clojure Run evaluates forms without a preliminary syntax check', async ({ page }) => {
  await page.goto('./');
  await page.getByLabel('Language').selectOption('clojure');
  await page
    .getByRole('textbox', { name: 'Clojure source code' })
    .fill('(println "executed before the syntax error")\n(');
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await finish(page);
  await expect(page.getByLabel('Console output')).toContainText('executed before the syntax error');
  await expect(page.locator('#status')).toHaveAttribute('data-state', 'error');
  await expect(page.locator('#elapsed')).toHaveText(/^Run: \d+\.\d{2}s$/);
  await page.getByRole('button', { name: 'Check', exact: true }).click();
  await finish(page);
  await expect(page.getByLabel('Console output')).not.toContainText(
    'executed before the syntax error',
  );
  await expect(page.locator('#status')).toHaveAttribute('data-state', 'error');
  await expect(page.locator('#elapsed')).toHaveText(/^Check: \d+\.\d{2}s$/);
});
