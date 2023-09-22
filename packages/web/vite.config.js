import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';
import Icons from 'unplugin-icons/vite'

export default defineConfig({
    plugins: [wasm(), topLevelAwait(), sveltekit(), Icons({ compiler: 'svelte', })]
});
