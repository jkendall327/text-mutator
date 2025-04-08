import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import basicSsl from '@vitejs/plugin-basic-ssl';

// https://vite.dev/config/
export default defineConfig({
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost',
        changeOrigin: true,
      },
    },
  },
  plugins: [react(), basicSsl()],
  build: {
    rollupOptions: {
      output: {
        entryFileNames: 'main-bundle.js',
        assetFileNames: 'assets/[name][extname]',
      },
    },
  },})
