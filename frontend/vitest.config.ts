import { defineConfig } from 'vitest/config'                                                                                                                                            
import react from '@vitejs/plugin-react-swc' // Import react plugin if needed for transforms                                                                                            
                                                                                                                                                                                        
export default defineConfig({
    test: {
        globals: true,
        environment: 'jsdom',
        setupFiles: './src/setupTests.ts',
        css: true,
    },
    plugins: [react()]
})     