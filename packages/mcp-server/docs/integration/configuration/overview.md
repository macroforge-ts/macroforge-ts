# Configuration

Macroforge can be configured with a `macroforge.config.ts` (or `.js`) file in your project root.

## Configuration File

Macroforge searches for config files in the following order, walking up from the input file's
directory:

- `macroforge.config.ts`
- `macroforge.config.mts`
- `macroforge.config.js`
- `macroforge.config.mjs`
- `macroforge.config.cjs`

Create a `macroforge.config.ts` file:

macroforge.config.ts

```
export default {
  keepDecorators: false,
  generateConvenienceConst: true,
};
```
