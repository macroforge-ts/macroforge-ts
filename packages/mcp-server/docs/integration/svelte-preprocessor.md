# Svelte Preprocessor

*The Svelte preprocessor expands Macroforge macros in `<script>` blocks before Svelte compilation, enabling seamless macro usage in Svelte components.*

## Installation

```bash
npm install -D @macroforge/svelte-preprocessor
```

## Configuration

Add the preprocessor to your `svelte.config.js`:

`svelte.config.js`
```javascript
import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { macroforgePreprocess } from '@macroforge/svelte-preprocessor';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: [
    macroforgePreprocess(),  // Expand macros FIRST
    vitePreprocess()          // Then handle TypeScript/CSS
  ],

  kit: {
    adapter: adapter()
  }
};

export default config;
```

> **Warning:**
> Always place `macroforgePreprocess()` **before** other preprocessors like `vitePreprocess()`. This ensures macros are expanded before TypeScript compilation.

## Usage

Use `@derive` decorators directly in your Svelte component scripts:

`UserCard.svelte`
```svelte
User: {user.name}

```

## Options

```typescript
macroforgePreprocess({
  // Keep @derive decorators in output (for debugging)
  keepDecorators: false,

  // Process JavaScript files (not just TypeScript)
  processJavaScript: false
})
```

### Option Reference

| `keepDecorators` 
| `boolean` 
| `false` 
| Keep decorators in output 

| `processJavaScript` 
| `boolean` 
| `false` 
| Process `<script>` blocks without `lang="ts"`

## How It Works

The preprocessor:

1. Intercepts `<script lang="ts">` blocks in `.svelte` files

2. Checks for `@derive` decorators (skips files without them)

3. Expands macros using the native Macroforge binary

4. Returns the transformed code for Svelte compilation

>
> Files without `@derive` decorators are passed through unchanged with zero overhead.

## SvelteKit Integration

For SvelteKit projects, you can use both the preprocessor (for `.svelte` files) and the Vite plugin (for standalone `.ts` files):

`svelte.config.js`
```javascript
// svelte.config.js
import { macroforgePreprocess } from '@macroforge/svelte-preprocessor';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  preprocess: [
    macroforgePreprocess(),
    vitePreprocess()
  ]
};
```

`vite.config.ts`
```typescript
// vite.config.ts
import macroforge from '@macroforge/vite-plugin';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  plugins: [
    macroforge(),  // For .ts files
    sveltekit()
  ]
});
```

## Using with Vitest

The preprocessor works seamlessly with Vitest for testing Svelte components:

`vitest.config.ts`
```typescript
// vitest.config.ts
import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import { svelteTesting } from '@testing-library/svelte/vite';
import macroforge from '@macroforge/vite-plugin';

export default defineConfig({
  plugins: [
    macroforge(),
    sveltekit(),
    svelteTesting()
  ],
  test: {
    environment: 'jsdom',
    include: ['src/**/*.{test,spec}.{js,ts}']
  }
});
```

## Svelte 5 Runes Compatibility

The preprocessor is fully compatible with Svelte 5 runes (`$state`, `$derived`, `$props`, etc.). Files using runes but without `@derive` decorators are skipped entirely.

```svelte

```