import { expect, test } from '@playwright/test';

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
