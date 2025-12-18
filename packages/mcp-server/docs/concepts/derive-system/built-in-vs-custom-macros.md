## Built-in vs Custom Macros

Macroforge comes with built-in macros that work out of the box. You can also create custom macros in Rust and use them via the `import macro` statement.

| Type     | Import Required | Examples                                                                        |
| -------- | --------------- | ------------------------------------------------------------------------------- |
| Built-in | No              | Debug, Clone, Default, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize |
| Custom   | Yes             | Any macro from an external package                                              |

## Next Steps

*   [Explore built-in macros](../../docs/builtin-macros)
*   [Create custom macros](../../docs/custom-macros)