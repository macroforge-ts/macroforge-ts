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

<IntegrationFlow
source="Your Code"
sourceDesc="TypeScript with @derive decorators"
branches={[
{
plugin: 'TypeScript Plugin',
pluginDesc: 'Language service integration',
outputs: [
{ label: 'IDE Feedback', desc: 'Errors & completions' }
]
},
{
plugin: 'Vite Plugin',
pluginDesc: 'Build-time transformation',
outputs: [
{ label: 'Dev Server', desc: 'Hot reload' },
{ label: 'Production Build', desc: 'Optimized output' }
]
}
]}
/>

## Detailed Guides

- [TypeScript Plugin setup]({base}/docs/integration/typescript-plugin)

- [Vite Plugin configuration]({base}/docs/integration/vite-plugin)

- [Configuration options]({base}/docs/integration/configuration)