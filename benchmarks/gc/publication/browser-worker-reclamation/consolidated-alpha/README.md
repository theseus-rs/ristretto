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

## Clean buffered barriers at `74275197`

Three fresh Chromium 153 workers returned Java checksum 799864. Each guest completed 13 collections, swept 54,059 objects and freed 10,473,075 bytes. The worker answered 256 of 257 scheduled pings during the Java window. The remaining ping received its reply after that window. Global GC pause duration is unmeasured.

| Run and raw receipt | Java window (s) | Worker replies during Java / scheduled pings | p95 reply gap (ms) | p99 (ms) | Maximum (ms) |
| --- | ---: | ---: | ---: | ---: | ---: |
| [1](74275197-chromium-1.json.gz) | 1.333 | 84/84 | 20.3 | 21.2 | 21.2 |
| [2](74275197-chromium-2.json.gz) | 1.504 | 94/94 | 19.3 | 21.6 | 21.6 |
| [3](74275197-chromium-3.json.gz) | 1.264 | 78/79 | 22.1 | 22.5 | 22.5 |

## Primitive array writes at `3d882623`

Three fresh production Chromium workers compiled and ran the same Java fixture.
Each completed 13 guest collections, swept 54,059 objects, freed 10,473,075
bytes and returned checksum 799864. Workers answered 357/357 scheduled
pings during Java execution. Each process used the same two assigned CPUs.
The gaps include guest execution, scheduling and message delivery. The probe
does not attribute any interval to a global GC pause.

| Process | Java window (s) | Replies / scheduled | p95 gap (ms) | p99 (ms) | Maximum (ms) |
|---|---:|---:|---:|---:|---:|
| [1](3d882623-chromium-1.json.gz) | 2.180 | 136/136 | 19.0 | 23.5 | 25.0 |
| [2](3d882623-chromium-2.json.gz) | 2.057 | 129/129 | 20.4 | 20.7 | 20.8 |
| [3](3d882623-chromium-3.json.gz) | 1.477 | 92/92 | 20.3 | 23.9 | 23.9 |

## Inline WASM guards at `09b6207d`

Three fresh Chromium 153.0.8010.12 workers ran the production WASI playground on CPUs 4 and 6. Each completed 13 guest collections, swept 54,059 objects, freed 10,473,075 bytes and returned checksum 799864. The workers replied to 225 of 225 scheduled pings within the Java measurement windows. All scheduled pings eventually received a response. These intervals include browser scheduling, Java and collector execution, and message delivery. GC-attributable global pause duration remains unmeasured.

| Process | Java window (s) | Replies during Java / scheduled | p95 gap (ms) | p99 (ms) | Maximum (ms) |
|---|---:|---:|---:|---:|---:|
| [1](09b6207d-chromium-1.json.gz) | 1.174 | 73/73 | 20.5 | 24.0 | 24.0 |
| [2](09b6207d-chromium-2.json.gz) | 1.235 | 77/77 | 21.5 | 22.9 | 22.9 |
| [3](09b6207d-chromium-3.json.gz) | 1.202 | 75/75 | 20.6 | 23.2 | 23.2 |

## Epoch-owned numeric deletion at `6f577187`

Three production Chromium workers each completed 13 Java collections and reclaimed 10,473,075 managed bytes. Each returned checksum 799864. The workers answered all 227 scheduled pings during Java execution. [Per-process gaps and linked collector identity](../../typed-root-collection/consolidated-collection-followups/protected-raw-java-point-writes/epoch-visible-deletions/browser-jvm/README.md).
