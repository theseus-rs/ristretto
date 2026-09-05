import { expect, test } from '@playwright/test';
import { disconnectNetwork } from './offline';

// Exercise each actual JDK, including its compiler's accepted source level and cached switching.
test('selects Java 8, 11, 17, 21, and 25 compilers and runtimes', async ({ page, browserName }) => {
  await page.goto('./');
  const picker = page.getByLabel('Java version');
  await expect(picker.locator('option')).toHaveText(['8', '11', '17', '21', '25']);
  const editor = page.getByRole('textbox', { name: 'Java source code' });
  const downloaded: string[] = [];
  page.on('request', (request) => {
    if (request.url().endsWith('.zip')) downloaded.push(request.url());
  });
  for (const version of [8, 11, 17, 21, 25]) {
    await picker.selectOption(String(version));
    await expect(page.locator('#example option[value="records"]')).toHaveJSProperty(
      'disabled',
      version < 21,
    );
    const feature =
      version === 8
        ? 'String value = "8";'
        : version === 11
          ? 'var value = "11";'
          : version === 17
            ? 'record Value(int n) {} String value = "" + new Value(17).n();'
            : `record Value(int n) {} Object v = new Value(${version}); String value = switch(v) { case Value(var n) -> "" + n; default -> "bad"; };`;
    await editor.fill(
      `public class Main { public static void main(String[] args) { ${feature} System.out.print("feature=" + value + " runtime=" + System.getProperty("java.version") + " hash=" + "😀".hashCode()); } }`,
    );
    await page.getByRole('button', { name: 'Run', exact: true }).click();
    await expect(picker).toBeDisabled();
    await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
      timeout: 755_000,
    });
    await expect(
      page.locator('#status'),
      `Java ${version}: ${await page.locator('#output').textContent()}`,
    ).toContainText('Finished successfully');
    await expect(page.locator('#output')).toContainText(
      `feature=${version} runtime=${version === 8 ? '1.8.' : version + '.'}`,
    );
    await expect(page.locator('#output')).toContainText('hash=1772899');
    expect(downloaded).toHaveLength([8, 11, 17, 21, 25].indexOf(version) + 1);
    expect(downloaded.at(-1)).toContain(`jdk-${version}.zip`);
  }
  const saved = await editor.textContent();
  await picker.selectOption('8');
  await expect(editor).toHaveText(saved!);
  await disconnectNetwork(page.context(), browserName);
  await editor.fill(
    'public class Main { public static void main(String[] args) { var value = 1; } }',
  );
  await page.getByRole('button', { name: 'Compile', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
  await expect(page.locator('#status')).toHaveAttribute('data-state', 'error');
  await expect(page.locator('#output')).toContainText('Main.java:1:');
  await editor.fill(
    'public class Main { public static void main(String[] args) { System.out.print("cached " + System.getProperty("java.version")); } }',
  );
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
  await expect(page.locator('#status')).toContainText('Finished successfully');
  await expect(page.locator('#output')).toContainText('cached 1.8.');
  expect(downloaded).toHaveLength(5);
});
