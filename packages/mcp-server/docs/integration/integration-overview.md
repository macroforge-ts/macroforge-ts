# Integration

*Macroforge integrates with your development workflow through IDE plugins and build tool integration.*

## Overview

| TypeScript Plugin 
| IDE support (errors, completions) 
| `@macroforge/typescript-plugin` 

| Vite Plugin 
| Build-time macro expansion 
| `@macroforge/vite-plugin`

## Recommended Setup

For the best development experience, use both integrations:

1. **TypeScript Plugin**: Provides real-time feedback in your IDE

2. **Vite Plugin**: Expands macros during development and production builds

```bash
# Install both plugins
npm install -D @macroforge/typescript-plugin @macroforge/vite-plugin
```

## How They Work Together

```text
┌────────────────────────────────────────────────────────┐
│                   Development Flow                      │
├────────────────────────────────────────────────────────┤
│                                                         │
│  Your Code ──► TypeScript Plugin ──► IDE Feedback       │
│      │                                                  │
│      │                                                  │
│      └──────► Vite Plugin ────────► Dev Server          │
│                     │                                   │
│                     └─────────────► Production Build    │
│                                                         │
└────────────────────────────────────────────────────────┘
```

## Detailed Guides

- [TypeScript Plugin setup](/docs/integration/typescript-plugin)

- [Vite Plugin configuration](/docs/integration/vite-plugin)

- [Configuration options](/docs/integration/configuration)