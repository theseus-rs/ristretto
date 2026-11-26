# Chromium worker progress on the consolidated collector

Three fresh Chromium 153 workers ran the production WASI guest from revision `849b9f31`. Each run returned Java checksum 799864. The guest reported 13 completed collections, 54,059 swept objects and 10,473,075 bytes freed over each invocation.

| Run and raw receipt | Java window (s) | Worker replies / scheduled pings | p95 reply gap (ms) | p99 (ms) | Maximum (ms) |
| --- | ---: | ---: | ---: | ---: | ---: |
| [1](chromium-1.json.gz) | 3.057 | 191/191 | 20.4 | 20.9 | 21.6 |
| [2](chromium-2.json.gz) | 2.900 | 181/181 | 18.1 | 21.2 | 22.7 |
| [3](chromium-3.json.gz) | 2.792 | 175/175 | 18.7 | 22.7 | 24.3 |

The gaps include guest execution, browser scheduling, and message delivery. GC-attributable global pause duration was unmeasured. The compressed JSON receipts retain every event, ping, GC counter and delivered artifact hash.

Build the playground with `npm run build:runtime` and `npm run build` in `web/`, serve its production assets, then run the probe from the repository root.

```sh
node web/scripts/probe-gc-turns.mjs web http://127.0.0.1:4489/ristretto/playground/ chromium.json --require-worker-progress --require-reclamation
```
