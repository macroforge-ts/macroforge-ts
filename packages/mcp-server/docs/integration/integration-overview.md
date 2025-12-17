# Integration
 *Macroforge integrates with your development workflow through IDE plugins and build tool integration.*
 ## Overview
 | Integration | Purpose | Package |
| --- | --- | --- |
| TypeScript Plugin | IDE support (errors, completions) | @macroforge/typescript-plugin |
| Vite Plugin | Build-time macro expansion | @macroforge/vite-plugin |
 ## Recommended Setup
 For the best development experience, use both integrations:
 1. **TypeScript Plugin**: Provides real-time feedback in your IDE
 2. **Vite Plugin**: Expands macros during development and production builds
 ```
#&nbsp;Install&nbsp;both&nbsp;plugins
npm&nbsp;install&nbsp;-D&nbsp;@macroforge/typescript-plugin&nbsp;@macroforge/vite-plugin
``` ## How They Work Together
 <div class="flex justify-center"><div class="border-2 border-primary bg-primary/10 rounded-lg px-6 py-3 text-center"><div class="font-semibold text-primary">Your Code TypeScript with @derive decorators  <div class="absolute top-0 h-px bg-border" style="width: 50%; left: 25%;"> <div class="flex flex-col items-center">  <div class="border border-border bg-card rounded-lg px-4 py-3 text-center w-full mt-1"><div class="font-medium text-foreground">TypeScript Plugin Language service integration   <div class="border border-success/30 bg-success/10 rounded-md px-3 py-2 text-center"><div class="text-sm font-medium text-success-foreground">IDE Feedback Errors & completions  <div class="border border-border bg-card rounded-lg px-4 py-3 text-center w-full mt-1"><div class="font-medium text-foreground">Vite Plugin Build-time transformation   <div class="border border-success/30 bg-success/10 rounded-md px-3 py-2 text-center"><div class="text-sm font-medium text-success-foreground">Dev Server Hot reload<div class="text-sm font-medium text-success-foreground">Production Build Optimized output ## Detailed Guides
 - [TypeScript Plugin setup](../docs/integration/typescript-plugin)
 - [Vite Plugin configuration](../docs/integration/vite-plugin)
 - [Configuration options](../docs/integration/configuration)