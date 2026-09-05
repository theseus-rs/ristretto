import { defineConfig } from 'vite';

export default defineConfig({
  base: '/ristretto/playground/',
  worker: { format: 'es' },
  build: { target: 'es2022' },
});
