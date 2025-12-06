import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, type PluginOption } from 'vite';
import napiMacros from 'vite-plugin-macroforge';

const macrosPlugin = napiMacros({
	typesOutputDir: '.macroforge/types',
	metadataOutputDir: '.macroforge/meta'
}) as PluginOption;

export default defineConfig({
	plugins: [macrosPlugin, sveltekit()]
});
