# fix(typescript-plugin): declare a file's types once where it is no module

- **Date:** 2026-10-14 13:24
- **Commit:** 1c13aa79
- **Area:** `@macroforge/typescript-plugin`

## Problem

In a file with no import or export, a derive that failed brought the editor a second error the user
had not made: "Duplicate identifier 'A'" at the class.

## Cause

The plugin registers the declarations of each expanded file as a companion file of the project. A
file that is no module declares its types globally, and so did the companion of it, so each type was
declared twice.

## Introduced

`5f86f787` (2025-12-06, "Rename tsserver-plugin and related packages to typescript-plugin") holds
the companion files as they stood.

## Fix

A companion is registered only where its declarations form a module.

## Verified

Ran `pixi run test:packages` (the six packages passed), in which the TypeScript plugin reports the
refused derive of a file that is no module and no other error.
