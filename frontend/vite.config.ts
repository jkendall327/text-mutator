import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc'
import basicSsl from '@vitejs/plugin-basic-ssl';

// https://vite.dev/config/
export default defineConfig({
  server: {
    // proxy: {
    //   '/api': {
    //     target: 'http://localhost:8080/api',
    //     changeOrigin: true,
    //   },
    // },
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
