# fix(core): count a field a subclass declares again once

- **Date:** 2026-10-13 18:14
- **Commit:** 9dd7da72
- **Area:** Derives on classes with a superclass

## Problem

```ts
export abstract class Shape {
    abstract kind: string;
    id: string = '';
}

/** @derive(Clone, Default, Encode, Decode) */
export class Circle extends Shape {
    kind: string = 'circle';
    radius: number = 1;
}
```

The code generated for `Circle` named `kind` twice, in each object literal and field list, and did
not type-check: "An object literal cannot have multiple properties with the same name", "Duplicate
identifier 'kind'".

## Cause

The fields of an instance are its superclasses' and then its own, and the two lists were joined as
they stood. An interface that declares again a field it inherits had the one take the other's place;
a class did not.

## Introduced

`bb74741f` (2026-10-12, "fix(core): make the value traits and Default reach every field of a type"),
on the branch this one is stacked on, added the fields a class inherits.

## Fix

A field a class declares again takes the place of the one it inherits, down any chain of
superclasses: one field, with the subclass's type and initializer, where the superclass gave it.

## Verified

Probed a subclass of an abstract base through the CLI before the fix, whose expansion had nine type
errors. Ran `cargo test -p macroforge_ts --features test-macros --lib` (756 passed), whose
`a_field_a_subclass_declares_again_takes_the_place_of_the_one_it_inherits` covers a chain of three,
`pixi run test:conformance:update` (10 passed), with the regenerated snapshots reviewed by the lines
that changed, and the three groups on the branch's final tree: `pixi run test:rust` (the workspace
and `tooling/testground/macro` passed), `pixi run test:testground` (43 Deno suites, 13 validator
files of 638 steps and 236 browser tests in Chromium, all passed) and `pixi run test:packages` (the
six packages passed), in which `edge-classes` builds, codes and compares `ClassCircle` and
`exact-types` type-checks it.
