# Browser worker progress during Java collections

One production Chromium worker ran the 50,000-allocation Java fixture using the cooperative WASI reactor. The Java checksum was 799864. Across the invocation the guest reported 12 completed collections, 47,712 swept objects and 2,099,328 bytes freed. Live bytes grew from 371,712 to 878,416, including survivors. The compiler used a separate VM.

The fixture's execution markers were 3,277.1 ms apart. Of 205 pings scheduled in that window, 204 received a worker reply inside it and all 205 eventually received replies. Within the window the 95th, 99th and largest worker-reply gaps were 25.4, 34.5 and 38.3 ms. The gaps include Java execution, yielding, browser scheduling and message delivery. GC-attributable global pause duration was unmeasured. The receipt includes the source, runtime and worker hashes and individual events. [Process receipt](process.json)

To reproduce, build and serve the production playground, then run the following from the repository root with the fixture's JDK 25 runtime installed.

```sh
node web/scripts/probe-gc-turns.mjs web http://127.0.0.1:4394/ristretto/playground/ result.json --require-worker-progress --require-reclamation
```
