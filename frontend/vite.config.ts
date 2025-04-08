import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc'

// https://vite.dev/config/
export default defineConfig({
  server: {
    cors: true
  },
  plugins: [react()],
  build: {
    rollupOptions: {
      output: {
        entryFileNames: 'main-bundle.js',
        assetFileNames: 'assets/[name][extname]',
      },
    },
  },})
