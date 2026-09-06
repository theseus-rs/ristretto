import { expect, test } from '@playwright/test';

// Firefox's interpreter workload is slower than V8/JSC. Keep its release gate focused on
// the compiler/VM boundary; the complete interaction suite also runs in Chromium and WebKit.
test('runs Java 25 features in Firefox, cancels compilation, and diagnoses invalid code offline', async ({
  page,
}) => {
  test.setTimeout(1_200_000);
  await page.goto('./');
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.locator('#status')).toContainText('Compiling…', { timeout: 120_000 });
  await page.getByRole('button', { name: 'Stop', exact: true }).click();
  await expect(page.locator('#status')).toContainText('Stopped');
  await page.context().setOffline(true);
  await page.getByRole('textbox', { name: 'Java source code' }).fill(`package example;
import java.util.List;
public class Main {
  record Coffee(String name, int shots) {}
  static class Nested { static int value = 42; }
  public static void main(String[] args) throws Exception {
    Object coffee = new Coffee("Ristretto", 2);
    System.out.println(switch (coffee) {
      case Coffee(var name, var shots) -> name + " with " + shots + " shots";
      default -> "unknown";
    });
    System.out.println(List.of("java", "ristretto").stream().map(String::toUpperCase).sorted().toList());
    System.out.println("☕😀 nested=" + Nested.value + " args=" + args.length + " stdin=" + System.in.read());
    System.err.println("stderr");
  }
}`);
  await page.getByRole('textbox', { name: 'Main class' }).fill('example.Main');
  await page.getByRole('button', { name: 'Run', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 755_000,
  });
  await expect(page.locator('#status')).toContainText('Finished successfully');
  await expect(page.locator('#output')).toContainText('Ristretto with 2 shots');
  await expect(page.locator('#output')).toContainText('[JAVA, RISTRETTO]');
  await expect(page.locator('#output')).toContainText('☕😀 nested=42 args=0 stdin=-1');
  await expect(page.locator('#output .stderr')).toHaveText('stderr\n');
  await page.screenshot({ path: 'test-results/playground-desktop-firefox.png', fullPage: true });
  await page
    .getByRole('textbox', { name: 'Java source code' })
    .fill('package example; public class Main { invalid Java; }');
  await page.getByRole('button', { name: 'Compile', exact: true }).click();
  await expect(page.getByRole('button', { name: 'Stop', exact: true })).toBeDisabled({
    timeout: 610_000,
  });
  await expect(page.locator('#status')).toHaveAttribute('data-state', 'error');
  await expect(page.locator('#output')).toContainText('Main.java:1:');
});
