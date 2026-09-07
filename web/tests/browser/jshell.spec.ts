import { expect, test, type Page } from '@playwright/test';

const ready = (page: Page) =>
  expect(page.locator('#shell-status')).toHaveAttribute('data-state', 'ready', {
    timeout: 755_000,
  });
async function send(page: Page, source: string) {
  await ready(page);
  await page.getByRole('textbox', { name: 'JShell input' }).fill(source);
  await page.getByRole('textbox', { name: 'JShell input' }).press('Enter');
  await ready(page);
}

test('retains a live JShell session with multiline input, commands, completion, and error recovery', async ({
  page,
}, testInfo) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));
  await page.goto('../jshell/');
  await ready(page);
  const transcript = page.getByRole('log', { name: 'JShell transcript' });
  const input = page.getByRole('textbox', { name: 'JShell input' });
  await expect(transcript).toContainText('Welcome to JShell');
  await expect(page.getByRole('button', { name: 'Run', exact: true })).toHaveCount(0);
  await send(page, 'int counter = 0;');
  await send(page, 'counter++;');
  await send(page, 'counter');
  await expect(transcript).toContainText('counter ==> 1');
  expect((await transcript.textContent())!.match(/counter ==> 0/g)).toHaveLength(1);
  await send(page, 'var values = new ArrayList<String>();');
  await send(page, 'values.add("coffee");');
  await send(page, 'values.toString()');
  await expect(transcript).toContainText('==> "[coffee]"');

  await send(page, 'int twice(int value) {');
  await expect(page.locator('#shell-prompt')).toHaveText('...>');
  await send(page, 'return value * 2;');
  await expect(page.locator('#shell-prompt')).toHaveText('...>');
  await send(page, '}');
  await expect(page.locator('#shell-prompt')).toHaveText('jshell>');
  await send(page, 'twice(counter)');
  await expect(transcript).toContainText('==> 2');
  await send(page, '/vars');
  await expect(transcript).toContainText('int counter = 1');
  await send(page, '/methods');
  await expect(transcript).toContainText('twice(int)int');
  await send(page, '/list');
  await expect(transcript).toContainText('1 : int counter = 0;');

  await input.fill('coun');
  await input.press('Tab');
  await ready(page);
  await expect(input).toHaveText('counter');
  await input.fill('/var');
  await input.press('Tab');
  await ready(page);
  await expect(input).toHaveText('/vars');
  await send(page, 'int invalid = "wrong";');
  await expect(transcript).toContainText('incompatible types');
  await send(page, 'throw new IllegalStateException("boom");');
  await expect(transcript).toContainText('java.lang.IllegalStateException: boom');
  await send(page, 'counter');
  await expect(transcript).toHaveText(/counter ==> 1\s*$/);
  await send(page, 'class Incomplete {');
  await input.press('Control+c');
  await ready(page);
  await expect(page.locator('#shell-prompt')).toHaveText('jshell>');
  await send(page, 'counter');
  await expect(transcript).toHaveText(/counter ==> 1\s*$/);
  await send(page, '/edit twice');
  await expect(input).toContainText('int twice');
  await input.fill('int twice(int value) { return value * 3; }');
  await input.press('Enter');
  await ready(page);
  await send(page, 'twice(4)');
  await expect(transcript).toHaveText(/==> 12\s*$/);
  await send(page, '/drop counter');
  await send(page, '/reload');
  await send(page, 'counter');
  await expect(transcript).toHaveText(/cannot find symbol[\s\S]*counter[\s\S]*\^\s*$/);

  await input.fill('draft input');
  await input.press('ArrowUp');
  await expect(input).toHaveText('counter');
  await input.press('ArrowDown');
  await expect(input).toHaveText('draft input');
  await input.fill('');
  await page.screenshot({ path: testInfo.outputPath('interactive-jshell.png'), fullPage: true });
  expect(errors).toEqual([]);
});

test('resets state, opens and saves scripts, interrupts execution, and remembers only input history', async ({
  page,
}) => {
  await page.goto('../jshell/');
  await ready(page);
  const transcript = page.getByRole('log', { name: 'JShell transcript' });
  const input = page.getByRole('textbox', { name: 'JShell input' });
  await send(page, 'int saved = 7;');
  const downloadEvent = page.waitForEvent('download');
  await send(page, '/save session.jsh');
  const download = await downloadEvent;
  expect(download.suggestedFilename()).toBe('session.jsh');
  const stream = await download.createReadStream();
  const chunks: Buffer[] = [];
  for await (const chunk of stream!) chunks.push(chunk);
  expect(Buffer.concat(chunks).toString()).toContain('int saved = 7;');
  await page.locator('#shell-file').setInputFiles({
    name: 'hello.jsh',
    mimeType: 'text/plain',
    buffer: Buffer.from('int imported = 42;\nimported'),
  });
  await ready(page);
  await expect(transcript).toContainText('imported ==> 42', { timeout: 755_000 });
  await send(page, '/reset');
  await send(page, '/vars');
  await expect(transcript).toHaveText(/For an introduction type: \/help intro\s*$/);
  await send(page, 'saved');
  await expect(transcript).toHaveText(/cannot find symbol[\s\S]*saved[\s\S]*\^\s*$/);
  await send(page, 'int liveOnly = 9;');
  await page.reload();
  await ready(page);
  await input.press('ArrowUp');
  await expect(input).toHaveText('int liveOnly = 9;');
  await send(page, 'liveOnly');
  await expect(transcript).toContainText('cannot find symbol');

  await input.fill('while (true) {}');
  await input.press('Enter');
  await expect(page.locator('#shell-status-text')).toHaveText('Evaluating…', { timeout: 755_000 });
  await page.getByRole('button', { name: 'Interrupt', exact: false }).click();
  await expect(transcript).toContainText('Execution interrupted');
  await expect(page.getByRole('button', { name: 'Submit input' })).toBeDisabled();
  await page.getByRole('button', { name: 'Start session' }).click();
  await ready(page);
  await send(page, '6 * 7');
  await expect(transcript).toHaveText(/==> 42\s*$/);
  await page.getByRole('button', { name: 'Clear screen' }).click();
  await expect(transcript).toBeEmpty();
  await send(page, '/exit');
  await expect(transcript).toContainText('Goodbye');
  await expect(page.getByRole('button', { name: 'Start session' })).toBeEnabled();
});

test('shares theme and navigation with Java and fits desktop and mobile screens', async ({
  page,
}, testInfo) => {
  await page.goto('../jshell/');
  await ready(page);
  await page.getByRole('button', { name: /^Color theme:/ }).click();
  const theme = await page.locator('html').getAttribute('data-theme');
  for (const width of [1920, 390]) {
    await page.setViewportSize({ width, height: 900 });
    const bounds = await page.getByRole('region', { name: 'JShell terminal' }).boundingBox();
    const margin = width > 900 ? 32 : 16;
    expect(bounds!.x).toBeCloseTo(margin, 0);
    expect(width - bounds!.x - bounds!.width).toBeCloseTo(margin, 0);
    expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(
      width,
    );
    await expect(page.getByRole('textbox', { name: 'JShell input' })).toBeVisible();
    await page.screenshot({ path: testInfo.outputPath(`jshell-${width}.png`), fullPage: true });
  }
  await page.getByRole('combobox', { name: 'Switch playground' }).selectOption('playground');
  await expect(page.getByRole('textbox', { name: 'Java source code' })).toBeVisible();
  await expect(page.locator('html')).toHaveAttribute('data-theme', theme!);
  await page.getByRole('combobox', { name: 'Switch playground' }).selectOption('jshell');
  await ready(page);
  await expect(page).toHaveURL(/\/ristretto\/jshell\/$/);
  await expect(page.locator('html')).toHaveAttribute('data-theme', theme!);
});
