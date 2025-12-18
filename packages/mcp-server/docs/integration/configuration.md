# Configuration

Macroforge can be configured with a `macroforge.json` file in your project root.

## Configuration File

Create a `macroforge.json` file:

macroforge.json 

```
{
  "allowNativeMacros": true,
  "macroPackages": [],
  "keepDecorators": false,
  "limits": {
    "maxExecutionTimeMs": 5000,
    "maxMemoryBytes": 104857600,
    "maxOutputSize": 10485760,
    "maxDiagnostics": 100
  }
}
```

## Options Reference

### allowNativeMacros

| Type    | `boolean` |
| Default | `true`    |

Enable or disable native (Rust) macro packages. Set to `false` to only allow built-in macros.

### macroPackages

| Type    | `string[]` |
| Default | `[]`       |

List of npm packages that provide macros. Macroforge will look for macros in these packages.

JSON

```
{
  "macroPackages": [
    "@my-org/custom-macros",
    "community-macros"
  ]
}
```

### keepDecorators

| Type    | `boolean` |
| Default | `false`   |

Keep `@derive` decorators in the output. Useful for debugging.

### limits

Configure resource limits for macro expansion:

JSON

```
{
  "limits": {
    // Maximum time for a single macro expansion (ms)
    "maxExecutionTimeMs": 5000,

    // Maximum memory usage (bytes)
    "maxMemoryBytes": 104857600,  // 100MB

    // Maximum size of generated code (bytes)
    "maxOutputSize": 10485760,    // 10MB

    // Maximum number of diagnostics per file
    "maxDiagnostics": 100
  }
}
```

## Macro Runtime Overrides

Override settings for specific macros:

JSON

```
{
  "macroRuntimeOverrides": {
    "@my-org/macros": {
      "maxExecutionTimeMs": 10000
    }
  }
}
```

Warning

Be careful when increasing limits, as this could allow malicious macros to consume excessive resources.

## Environment Variables

Some settings can be overridden with environment variables:

| Variable              | Description          |
| --------------------- | -------------------- |
| `MACROFORGE_DEBUG`    | Enable debug logging |
| `MACROFORGE_LOG_FILE` | Write logs to a file |

Bash

```
MACROFORGE_DEBUG=1 npm run dev
```