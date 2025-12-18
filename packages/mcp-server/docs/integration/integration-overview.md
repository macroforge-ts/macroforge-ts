# Integration

Macroforge integrates with your development workflow through IDE plugins and build tool integration.

## Overview

| Integration       | Purpose                           | Package                         |
| ----------------- | --------------------------------- | ------------------------------- |
| TypeScript Plugin | IDE support (errors, completions) | `@macroforge/typescript-plugin` |
| Vite Plugin       | Build-time macro expansion        | `@macroforge/vite-plugin`       |

## Recommended Setup

For the best development experience, use both integrations:

1.  **TypeScript Plugin**: Provides real-time feedback in your IDE
2.  **Vite Plugin**: Expands macros during development and production builds

Bash

```
# Install both plugins
npm install -D @macroforge/typescript-plugin @macroforge/vite-plugin
```

## How They Work Together

Your Code

TypeScript with @derive decorators

TypeScript Plugin

Language service integration

IDE Feedback

Errors & completions

Vite Plugin

Build-time transformation

Dev Server

Hot reload

Production Build

Optimized output

## Detailed Guides

*   [TypeScript Plugin setup](../docs/integration/typescript-plugin)
*   [Vite Plugin configuration](../docs/integration/vite-plugin)
*   [Configuration options](../docs/integration/configuration)