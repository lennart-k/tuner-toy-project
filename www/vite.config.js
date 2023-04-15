import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'
import { svelte } from '@sveltejs/vite-plugin-svelte';
import {defineConfig} from 'vite'
import {sveltePreprocess} from 'svelte-preprocess/dist/autoProcess.js'

export default defineConfig({
    plugins: [
        wasm(),
        topLevelAwait(),
        svelte({
            preprocess: [
                sveltePreprocess({ typescript: true })
            ]
        })
    ]
})

