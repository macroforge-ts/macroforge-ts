# sv

Everything you need to build a Svelte project, powered by [`sv`](https://github.com/sveltejs/cli).

## Creating a project

If you're seeing this, you've probably already done this step. Congrats!

```sh
# create a new project in the current directory
npx sv create

# create a new project in my-app
npx sv create my-app
```

## Macro demo

This playground is wired to the local `@ts-macros/swc-napi` transformer via `vite-plugin-napi`. The `@ts-macros/ts-derive-plugin` TypeScript plugin automatically augments every class decorated with `@Derive` so editors know `toString()` and `toJSON()` exist without manual type assertions.

Key files:

- `vite.config.ts` – registers the Rust transformer before `sveltekit()`
- `src/lib/macros.ts` – ambient definitions that mirror the Rust macros
- `src/lib/demo/macro-user.ts` – demo class using `@Derive('Debug', 'JSON')`
- `src/lib/demo/macro-snippet.md` – included at build time via `IncludeStr`

## Developing

Install deps, build the TypeScript plugin, then start the dev server:

```sh
npm install            # run once in this directory
npm run build --workspaces=@ts-macros/ts-derive-plugin  # from repo root if plugin changes
npm run dev -- --open
```

## Building

```sh
npm run build
npm run preview
```

> To deploy your app, install an [adapter](https://svelte.dev/docs/kit/adapters) for your target runtime.
