# fix/svelte-check-exit-code changes

In commit order.

| Date             | Commit                     | Change                                                                                                                                   | Introduced            |
| ---------------- | -------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | --------------------- |
| 2026-10-09 10:34 | 9a01abae                   | [fix(cli): keep svelte-check's exit code through the output rewrite](2026-10-09-1034-svelte-check-exit-code.md)                          | 2aabc445 (2026-10-08) |
| 2026-10-09 10:34 | 852d0641                   | [chore(testground): record the macro package's proc-macro2 dependency](2026-10-09-1034-testground-proc-macro2-lock.md)                   | 1cc2f3c4 (2026-10-08) |
| 2026-10-09 20:21 | 6eaf9134, 3bb74ef6 (fixup) | [fix(endec): encode an adjacently tagged union's variant content](2026-10-09-2021-endec-adjacent-union-content.md)                       | 1cc2f3c4 (2026-10-08) |
| 2026-10-11 07:22 | e0b8b258                   | [fix(endec): encode and decode a nullable collection's elements](2026-10-11-0722-endec-nullable-collection-elements.md)                  | dafd08ef (2025-12-07) |
| 2026-10-11 07:22 | 6aa6de8b                   | [fix(cli): re-expand a package when the CLI itself was rebuilt](2026-10-11-0722-cli-rebuilt-cli-re-expands.md)                           | 594b8de4 (2026-08-11) |
| 2026-10-11 07:44 | 1c71df91                   | [fix(endec): report a collection field whose value is not that collection](2026-10-11-0744-endec-collection-wrong-type.md)               | dafd08ef (2025-12-07) |
| 2026-10-11 07:44 | 78d6bae6                   | [fix(endec): code an undefined union's member and a nullable wrapper by their own type](2026-10-11-0744-endec-undefined-union-member.md) | dafd08ef (2025-12-07) |
