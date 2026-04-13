// Minimal reactive runtime that the runes macros expand into.
// Inspired by Svelte 5's fine-grained reactivity model.

type Subscriber = () => void;

let activeEffect: Subscriber | null = null;
const batchQueue: Set<Subscriber> = new Set();
let isBatching = false;

function batch(fn: () => void) {
    isBatching = true;
    fn();
    isBatching = false;
    for (const sub of batchQueue) sub();
    batchQueue.clear();
}

function notify(subscribers: Set<Subscriber>) {
    for (const sub of subscribers) {
        if (isBatching) {
            batchQueue.add(sub);
        } else {
            sub();
        }
    }
}

export interface Signal<T> {
    get value(): T;
    set value(v: T);
}

export function createSignal<T>(initial: T): Signal<T> {
    let current = initial;
    const subscribers = new Set<Subscriber>();

    return {
        get value() {
            if (activeEffect) subscribers.add(activeEffect);
            return current;
        },
        set value(v: T) {
            if (v !== current) {
                current = v;
                notify(subscribers);
            }
        }
    };
}

export interface Derived<T> {
    readonly value: T;
}

export function createDerived<T>(fn: () => T): Derived<T> {
    let cached: T;
    let dirty = true;
    const subscribers = new Set<Subscriber>();

    const recompute: Subscriber = () => {
        dirty = true;
        notify(subscribers);
    };

    return {
        get value() {
            if (activeEffect) subscribers.add(activeEffect);
            if (dirty) {
                const prev = activeEffect;
                activeEffect = recompute;
                cached = fn();
                activeEffect = prev;
                dirty = false;
            }
            return cached;
        }
    };
}

export function createEffect(fn: () => void | (() => void)): () => void {
    let cleanup: (() => void) | void;

    const execute: Subscriber = () => {
        if (cleanup) cleanup();
        const prev = activeEffect;
        activeEffect = execute;
        cleanup = fn();
        activeEffect = prev;
    };

    execute();

    return () => {
        if (cleanup) cleanup();
    };
}

export { batch };
