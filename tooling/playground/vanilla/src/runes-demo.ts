// Svelte-runes-style reactivity via macroforge proc macros.
//
// The $state, $derived, and $effect macros are Rust-implemented
// function-like proc macros in the playground macro crate. They
// expand at build time into calls to the reactive runtime.
//
// Before expansion:
//   const count = $state(0);
//   const doubled = $derived(count.value * 2);
//   $effect(console.log(`count = ${count.value}`));
//
// After expansion:
//   const count = createSignal(0);
//   const doubled = createDerived(() => count.value * 2);
//   createEffect(() => { console.log(`count = ${count.value}`); });

import { $derived, $effect, $state } from '@playground/macro';
import { batch, createDerived, createEffect, createSignal } from './runes-runtime.ts';

// The macros expand `$state/$derived/$effect` into `createSignal/
// createDerived/createEffect` calls at build time. The linter can't see
// that indirection, so re-export the runtime helpers to keep the
// imports live.
export { batch, createDerived, createEffect, createSignal };

// --- Reactive state ---
const count = $state(0);
const name = $state('world');

// --- Derived values (auto-recompute when deps change) ---
const doubled = $derived(count.value * 2);
const greeting = $derived(`Hello, ${name.value}! Count is ${count.value}`);

// --- Effects (re-run when deps change) ---
$effect(console.log('[effect] greeting:', greeting.value));
$effect(console.log('[effect] doubled:', doubled.value));

// --- Drive the reactive graph ---
console.log('--- initial ---');
console.log('count:', count.value, 'doubled:', doubled.value);
console.log('greeting:', greeting.value);

console.log('\n--- count = 5 ---');
count.value = 5;

console.log("\n--- name = 'macroforge' ---");
name.value = 'macroforge';

console.log('\n--- batch update ---');
batch(() => {
    count.value = 10;
    name.value = 'runes';
});

// Export for the test harness.
export const runesDemo = {
    count,
    doubled,
    name,
    greeting
};
