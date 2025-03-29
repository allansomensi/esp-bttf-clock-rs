import { defineConfig } from 'vite';

export default defineConfig({
    build: {
        outDir: 'dist',
        rollupOptions: {
            output: {
                entryFileNames: 'assets/js/[name].js',
                chunkFileNames: 'assets/js/[name].js',
                assetFileNames: 'assets/[name][extname]',
            }
        },
    }
});
