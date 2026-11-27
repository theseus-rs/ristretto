# Browser worker progress with concurrent host reference processing

A Chromium 153 worker ran the production WASI guest built at `a62b4fad`. Java completed the 12-cycle fixture with checksum 799864. The collector reported 47,712 objects swept and 2,099,328 bytes freed during the fixture.

The browser scheduled 222 pings and received all 222 replies during the 3.55-second Java window. The 95th percentile gap between worker replies was 29.9 ms, the 99th percentile was 39.4 ms and the longest was 50.7 ms. Reply gaps include browser scheduling and guest execution. The global-pause measurement is unavailable.

Build with `npm run build:runtime` and `npm run build` in `web/`, serve the production assets and run the following from the repository root. Replace the URL with the preview address.

```sh
node web/scripts/probe-gc-turns.mjs web http://127.0.0.1:4397/ristretto/playground/ chromium.json --require-worker-progress --require-reclamation
```

[Raw worker events, pings, artifact hashes and collection counters](chromium.json)
