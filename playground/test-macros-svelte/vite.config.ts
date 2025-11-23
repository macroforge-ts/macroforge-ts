import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type PluginOption } from 'vite';
import napiMacros from 'vite-plugin-napi';

const macrosPlugin = napiMacros() as PluginOption;

export default defineConfig({
	plugins: [macrosPlugin, sveltekit()]
});
