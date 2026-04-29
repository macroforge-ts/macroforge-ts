// Reactivity tests for the runes proc macros.
//
// Verifies that the reactive runtime works correctly with the
// macro-expanded code. Results are exposed on window.runesTestResults
// for Playwright assertions.

/** import macro { $state, $derived, $effect } from "@playground/macro" */
import {
  batch,
  createDerived,
  createEffect,
  createSignal,
} from "./runes-runtime.ts";

// The macros expand `$state/$derived/$effect` into `createSignal/
// createDerived/createEffect` calls at build time. The linter can't see
// that indirection, so re-export the runtime helpers to keep the
// imports live.
export { createDerived, createEffect, createSignal };

export interface RunesTestResults {
  passed: number;
  failed: number;
  details: string[];
}

export function runRunesTests(): RunesTestResults {
  const details: string[] = [];
  let passed = 0;
  let failed = 0;

  function assert(condition: boolean, message: string) {
    if (condition) {
      passed++;
      details.push(`PASS: ${message}`);
    } else {
      failed++;
      details.push(`FAIL: ${message}`);
    }
  }

  // ---- $state: basic signal ----
  {
    const count = $state(0);
    assert(count.value === 0, "state: initial value is 0");
    count.value = 42;
    assert(count.value === 42, "state: updates to 42");
  }

  // ---- $state: string type ----
  {
    const name = $state("hello");
    assert(name.value === "hello", "state: string initial");
    name.value = "world";
    assert(name.value === "world", "state: string update");
  }

  // ---- $derived: recomputes ----
  {
    const count = $state(1);
    const doubled = $derived(count.value * 2);
    assert(doubled.value === 2, "derived: initial is 2");
    count.value = 5;
    assert(doubled.value === 10, "derived: recomputes to 10");
  }

  // ---- $derived: chains ----
  {
    const a = $state(1);
    const b = $derived(a.value + 1);
    const c = $derived(b.value * 2);
    assert(c.value === 4, "derived chain: (1+1)*2 = 4");
    a.value = 10;
    assert(b.value === 11, "derived chain: b = 11");
    assert(c.value === 22, "derived chain: c = 22");
  }

  // ---- $effect: runs on change ----
  {
    const log: number[] = [];
    const count = $state(0);
    $effect(log.push(count.value));

    assert(log.length === 1, "effect: runs on creation");
    assert(log[0] === 0, "effect: sees initial value");
    count.value = 1;
    assert(log.length === 2, "effect: re-runs on change");
    assert(log[1] === 1, "effect: sees new value");
  }

  // ---- $effect: skips identical value ----
  {
    const log: number[] = [];
    const count = $state(7);
    $effect(log.push(count.value));
    count.value = 7; // same value
    assert(log.length === 1, "effect: skips identical value");
  }

  // ---- $effect: arrow function syntax ----
  {
    const log: string[] = [];
    const name = $state("a");
    $effect(() => {
      log.push(name.value);
    });
    assert(log[0] === "a", "effect arrow: initial");
    name.value = "b";
    assert(log[1] === "b", "effect arrow: re-runs");
  }

  // ---- batch: coalesces updates ----
  {
    let effectRuns = 0;
    const a = $state(0);
    const b = $state(0);
    const sum = $derived(a.value + b.value);
    $effect(() => {
      sum.value;
      effectRuns++;
    });

    const runsBeforeBatch = effectRuns;
    batch(() => {
      a.value = 10;
      b.value = 20;
    });
    assert(
      effectRuns === runsBeforeBatch + 1,
      "batch: effect runs once for batched update",
    );
    assert(sum.value === 30, "batch: sum is 30");
  }

  return { passed, failed, details };
}
