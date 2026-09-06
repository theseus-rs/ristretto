import type { BrowserContext } from '@playwright/test';

export async function disconnectNetwork(context: BrowserContext, browserName: string) {
  // WebKit's offline emulation rejects even a Blob worker containing only
  // postMessage("ready"). Block all HTTP(S) instead, allowing in-memory Blob URLs.
  // The same minimal worker works in a Linux container with --network none.
  if (browserName === 'webkit') {
    await context.route(/^https?:\/\//, (route) => route.abort('internetdisconnected'));
  } else {
    await context.setOffline(true);
  }
}
