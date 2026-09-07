import { defineConfig } from 'vite';

export default defineConfig({
  base: '/ristretto/',
  worker: { format: 'es' },
  build: {
    target: 'es2022',
    rollupOptions: { input: ['playground/index.html', 'jshell/index.html'] },
  },
});
