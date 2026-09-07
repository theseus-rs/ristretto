import workerScript from './runner.worker?worker&url';

// Keep the bundled script in memory so every run can use a fresh worker even offline.
let workerUrl: Promise<string> | undefined;
export function loadWorker() {
  // Vite's development worker imports other modules from its server.
  if (import.meta.env.DEV) return Promise.resolve(workerScript);
  return (workerUrl ??= fetch(workerScript)
    .then(async (response) => {
      if (!response.ok)
        throw new Error(`Could not load Java worker (${response.status}). Try again.`);
      return URL.createObjectURL(new Blob([await response.text()], { type: 'text/javascript' }));
    })
    .catch((error) => {
      workerUrl = undefined;
      throw error;
    }));
}
