import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import wasmPack from 'vite-plugin-wasm-pack';

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [
        svelte(),
        wasmPack('../wasm')
    ],
    server: {
        host: "0.0.0.0",
        port: 3000,
        strictPort: true,
    }
})
