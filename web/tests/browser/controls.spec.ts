import { expect, test } from '@playwright/test';
import { JAVA_8, JAVA_21, JAVA_25 } from '../../shared/protocol';

test('Java 25 examples use compact main methods and preserve edited drafts', async ({ page }) => {
  await page.goto('./');
  const version = page.getByLabel('Java version');
  const editor = page.getByRole('textbox', { name: 'Java source code' });
  await expect(version).toHaveValue(String(JAVA_25));
  for (const example of ['hello', 'collections', 'records']) {
    await page.getByLabel('Start with').selectOption(example);
    await expect(editor).toContainText('void main()');
    await expect(editor).not.toContainText('public class Main');
    await version.selectOption(String(JAVA_21));
    await expect(editor).toContainText('public static void main(String[] args)');
    await version.selectOption(String(JAVA_25));
    await expect(editor).toContainText('void main()');
  }
  await expect(page.locator('#main-help')).toContainText('compact source files');
  const custom = 'void main() { IO.println("saved draft"); }';
  await editor.fill(custom);
  await version.selectOption(String(JAVA_8));
  await expect(editor).toHaveText(custom);
  await page.getByLabel('Language').selectOption('kotlin');
  await page.getByLabel('Language').selectOption('java');
  await expect(editor).toHaveText(custom);
  await version.selectOption(String(JAVA_25));
  await page.reload();
  await expect(version).toHaveValue(String(JAVA_25));
  await expect(editor).toHaveText(custom);
});

test('Cmd and Ctrl Enter run once without changing the source', async ({ page }) => {
  await page.goto('./');
  const editor = page.getByRole('textbox', { name: 'Java source code' });
  const code =
    'public class Main { public static void main(String[] args) { System.out.println("shortcut works"); } }';
  await editor.fill(code);
  let workers = 0;
  page.on('worker', () => workers++);
  for (const modifier of ['Control', 'Meta']) {
    const before = workers;
    await editor.press(`${modifier}+Enter`);
    await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeEnabled();
    await editor.press(`${modifier}+Enter`);
    await expect(editor).toHaveText(code);
    await expect(editor.locator('.cm-line')).toHaveCount(1);
    await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
      timeout: 755_000,
    });
    await expect(page.locator('#status')).toContainText('Finished successfully');
    await expect(page.getByLabel('Console output')).toHaveText('shortcut works\n');
    expect(workers - before).toBe(1);
    await expect(editor).toHaveText(code);
  }
  await editor.press('End');
  await editor.press('Enter');
  await expect(editor.locator('.cm-line')).toHaveCount(2);
  expect(workers).toBe(2);
});

test('JShell elapsed time advances during input, freezes, and survives completion and interruption', async ({
  page,
}) => {
  await page.goto('../jshell/');
  const status = page.locator('#shell-status');
  const elapsed = page.locator('#shell-elapsed');
  const input = page.getByRole('textbox', { name: 'JShell input' });
  const seconds = async () => Number.parseFloat((await elapsed.textContent())!);
  await expect(elapsed).toHaveText(/^\d+\.\d{2}s$/);
  await expect(status).toHaveAttribute('data-state', 'ready', { timeout: 755_000 });
  await input.fill('Thread.sleep(1200);');
  await input.press('Enter');
  await expect(status).toHaveAttribute('data-state', 'busy');
  const first = await seconds();
  await expect.poll(seconds).toBeGreaterThan(first + 0.3);
  await expect(status).toHaveAttribute('data-state', 'ready', { timeout: 755_000 });
  const final = await elapsed.textContent();
  await page.waitForTimeout(250);
  await expect(elapsed).toHaveText(final!);
  await input.fill('/var');
  await input.press('Tab');
  await expect(input).toHaveText('/vars', { timeout: 755_000 });
  await expect(elapsed).toHaveText(final!);
  await input.fill('while (true) {}');
  await input.press('Enter');
  expect(await seconds()).toBeLessThan(Number.parseFloat(final!));
  await expect.poll(seconds).toBeGreaterThan(0.2);
  await page.getByRole('button', { name: /Interrupt/ }).click();
  const interrupted = await elapsed.textContent();
  await page.waitForTimeout(250);
  await expect(elapsed).toHaveText(interrupted!);
  await page.getByRole('button', { name: 'Start session' }).click();
  await expect(status).toHaveAttribute('data-state', 'ready', { timeout: 755_000 });
  await input.fill('6 * 7');
  await input.press('Enter');
  await expect(status).toHaveAttribute('data-state', 'ready', { timeout: 755_000 });
  await expect(page.getByRole('log', { name: 'JShell transcript' })).toHaveText(/==> 42\s*$/);
});
