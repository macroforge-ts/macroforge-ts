// The results the vanilla playground publishes for the Playwright specs. The
// specs compile this module, so it references only types whose modules
// type-check on their own, never the harnesses that exercise the macros.
import type { AllMacrosTestClass } from './all-macros-test.ts';
import type { E2eResults } from './e2e-results.ts';
import type {
    EventForm,
    ProductForm,
    UserRegistrationForm,
    ValidationResult
} from './validator-form.ts';

/** What the macro test panel's buttons produce. */
export interface MacroTestResults {
    debug?: string;
    clone?: AllMacrosTestClass;
    equals?: boolean;
    hashCode?: number;
    serialize?: string;
    deserialize?: AllMacrosTestClass;
}

/**
 * Runtime probes of the attributes fixture. `@cfg`-stripped exports are
 * absent at runtime, so their probe is `null`.
 */
export interface AttributesResults {
    keptByFeature: string | null;
    strippedByFeature: string | null;
    keptByTarget: string | null;
    strippedByTarget: string | null;
    deprecatedCall: string;
    nonExhaustiveValue: string;
}

export interface RunesTestResults {
    passed: number;
    failed: number;
    details: string[];
}

/** Values `@buildtime` spliced into buildtime-demo.ts as literals. */
export interface BuildtimeDemoResult {
    answer: number;
    schemaHash: string;
    appName: string;
    appVersion: string;
    routes: string[];
    constantObject: { thirteen: number; label: string; items: number[] };
    derivedSummary: string;
    greetingAlice: string;
    greetingBob: string;
    greetingKeys: string[];
    /**
     * Whether calling the `@macroforge/core/buildtime` runtime stub throws.
     * Every real `@buildtime` use is spliced into a literal, but the stub
     * itself must still throw at runtime.
     */
    runtimeStubThrows: boolean;
}

export interface ValidatorFormResults {
    userRegistration?: ValidationResult<UserRegistrationForm>;
    product?: ValidationResult<ProductForm>;
    event?: ValidationResult<EventForm>;
}

/**
 * Everything the vanilla playground publishes for the Playwright specs. Each
 * page fills in the slices it runs.
 */
export interface VanillaPlayground {
    macroTests?: MacroTestResults;
    runes?: RunesTestResults;
    e2e?: E2eResults;
    buildtime?: BuildtimeDemoResult;
    attributes?: AttributesResults;
    validatorForm?: ValidatorFormResults;
}

declare global {
    var vanillaPlayground: VanillaPlayground | undefined;
}

/** The playground's results object, created on first use. */
export function playgroundResults(): VanillaPlayground {
    globalThis.vanillaPlayground ??= {};
    return globalThis.vanillaPlayground;
}
