# Java collection in the production Chromium worker

Three fresh Chromium 153.0.8010.12 workers compiled and ran `GcTurns.java` on the release WASIp2 playground engine at `6f577187`. Each returned checksum 799864, completed 13 guest collections, swept 54,059 objects and refunded 10,473,075 managed bytes. The worker responded to all 227 scheduled pings during the Java windows. These intervals include guest execution, browser scheduling and message delivery. The probe does not attribute them to collector pauses.

| Fresh worker and recording | Java window (s) | Replies during Java / scheduled | p95 reply gap (ms) | Maximum gap (ms) |
|---|---:|---:|---:|---:|
| [1](chromium-1.json.gz) | 1.182 | 74 / 74 | 21.5 | 22.8 |
| [2](chromium-2.json.gz) | 1.218 | 76 / 76 | 21.7 | 22.4 |
| [3](chromium-3.json.gz) | 1.233 | 77 / 77 | 20.7 | 23.8 |

The probe used CPUs 4 and 6, the pinned JavaScript package versions and a hash-verified cached Java 25 image. The playground worker was bundled from the measured revision. Cargo's artifact records identify the exact GC, class loader, VM and playground engine in the browser component. The three runs matched on worker, component assets, JDK and Java source. The [provenance and process summaries](provenance.json) record their hashes. To repeat on a built site, run the repository's `web/scripts/probe-gc-turns.mjs` with `--require-worker-progress --require-reclamation`.
