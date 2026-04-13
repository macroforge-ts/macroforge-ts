// Comprehensive e2e test harness.
//
// Imports all macro types and exposes their results on `window` so
// Playwright specs can assert correctness without touching the DOM.

import {
    declarativeMacrosErased,
    emptyVec,
    exprVec,
    identityCall,
    threeVec,
    withTempResult
} from './declarative-macros.ts';

import { crossFileEmpty, crossFileExpr, crossFileId, crossFileThree } from './cross-file-decl.ts';

import {
    declarativeComplexErased,
    definitelyHello,
    hygieneCheck,
    minNegative,
    minOfTwo,
    minOne,
    minTwo,
    orElseNoDefault,
    orElseWithDefault,
    pow4Two,
    samplePatch,
    sqPlus1,
    squaredFive,
    sumFive,
    sumTriple
} from './declarative-complex.ts';

import { Color, Priority, Status, user } from './enum-type-examples.ts';
import type { Point } from './enum-type-examples.ts';

// The derive macros emit `export function statusToString(...)` etc. into the
// expanded source at runtime. TypeScript can't see them in the .ts file, so
// we pull the whole module as `unknown`-typed and pluck the generated helpers
// off with a tightly-scoped cast.
import * as enumMod from './enum-type-examples.ts';
const m = enumMod as unknown as Record<string, (...args: unknown[]) => unknown>;

import { FormModel } from './form-model.ts';

import {
    testMissingFields,
    testMixedElements,
    testNormal,
    testNullElement,
    testRecursiveActual
} from './runtime-deser-test.ts';

import { AllMacrosTestClass, testInstance } from './all-macros-test.ts';

import {
    concatDbHost,
    concatUserName,
    stringifiedExpr,
    stringifiedIdent,
    tracedAdd,
    tracedGreet
} from './attr-macro-test.ts';

// ── Declarative macros ──────────────────────────────────────────

export interface DeclarativeMacroResults {
    erased: boolean;
    emptyVec: unknown;
    threeVec: unknown;
    exprVec: unknown;
    identityCall: unknown;
    withTempResult: unknown;
    crossFileEmpty: unknown;
    crossFileThree: unknown;
    crossFileExpr: unknown;
    crossFileId: unknown;
}

function collectDeclarativeMacros(): DeclarativeMacroResults {
    return {
        erased: declarativeMacrosErased,
        emptyVec,
        threeVec,
        exprVec,
        identityCall,
        withTempResult,
        crossFileEmpty,
        crossFileThree,
        crossFileExpr,
        crossFileId
    };
}

// ── Complex declarative macros ─────────────────────────────────

export interface DeclarativeComplexResults {
    erased: boolean;
    minOne: unknown;
    minTwo: unknown;
    minOfTwo: unknown;
    minNegative: unknown;
    orElseNoDefault: unknown;
    orElseWithDefault: unknown;
    squaredFive: unknown;
    pow4Two: unknown;
    sumTriple: unknown;
    sumFive: unknown;
    hygieneCheck: unknown;
    sqPlus1: unknown;
    samplePatch: unknown;
    definitelyHello: unknown;
}

function collectDeclarativeComplex(): DeclarativeComplexResults {
    return {
        erased: declarativeComplexErased,
        minOne,
        minTwo,
        minOfTwo,
        minNegative,
        orElseNoDefault,
        orElseWithDefault,
        squaredFive,
        pow4Two,
        sumTriple,
        sumFive,
        hygieneCheck: hygieneCheck(),
        sqPlus1,
        samplePatch,
        definitelyHello
    };
}

// ── Enum & type alias derives ──────────────────────────────────

export interface EnumDeriveResults {
    statusDebug: string | null;
    statusClone: unknown;
    statusEquals: boolean | null;
    statusHash: number | null;
    statusSerialize: unknown;
    statusDeserialize: unknown;
    priorityDebug: string | null;
    priorityEquals: boolean | null;
    colorDebug: string | null;
    colorEquals: boolean | null;
}

function collectEnumDerives(): EnumDeriveResults {
    const result: EnumDeriveResults = {
        statusDebug: null,
        statusClone: null,
        statusEquals: null,
        statusHash: null,
        statusSerialize: null,
        statusDeserialize: null,
        priorityDebug: null,
        priorityEquals: null,
        colorDebug: null,
        colorEquals: null
    };

    // Each derive call is isolated so one missing helper doesn't zero
    // out the rest of the result object.
    const safe = <T>(fn: () => T): T | null => {
        try {
            return fn();
        } catch {
            return null;
        }
    };

    result.statusDebug = safe(() => m.statusToString(Status.Active) as string);
    result.statusClone = safe(() => m.statusClone(Status.Pending));
    result.statusEquals = safe(() => m.statusEquals(Status.Active, Status.Active) as boolean);
    result.statusHash = safe(() => m.statusHashCode(Status.Active) as number);
    result.statusSerialize = safe(() => m.statusSerialize(Status.Inactive));
    result.statusDeserialize = safe(() => m.statusDeserialize('pending'));
    result.priorityDebug = safe(() => m.priorityToString(Priority.High) as string);
    result.priorityEquals = safe(() => m.priorityEquals(Priority.Low, Priority.Low) as boolean);
    result.colorDebug = safe(() => m.colorToString(Color.Red) as string);
    result.colorEquals = safe(() => m.colorEquals(Color.Blue, Color.Blue) as boolean);

    return result;
}

// ── Type alias derives ──────────────────────────────────────────

export interface TypeAliasDeriveResults {
    pointDebug: string | null;
    pointClone: unknown;
    pointEquals: boolean | null;
    pointEqualsNe: boolean | null;
    pointHash: number | null;
    pointSerialize: unknown;
    pointDeserialize: unknown;
    userProfileDebug: string | null;
    userProfileEquals: boolean | null;
}

function collectTypeAliasDerives(): TypeAliasDeriveResults {
    const result: TypeAliasDeriveResults = {
        pointDebug: null,
        pointClone: null,
        pointEquals: null,
        pointEqualsNe: null,
        pointHash: null,
        pointSerialize: null,
        pointDeserialize: null,
        userProfileDebug: null,
        userProfileEquals: null
    };

    const safe = <T>(fn: () => T): T | null => {
        try {
            return fn();
        } catch {
            return null;
        }
    };

    const p1: Point = { x: 10, y: 20 };
    const p2: Point = { x: 10, y: 20 };
    const p3: Point = { x: 99, y: 1 };

    result.pointDebug = safe(() => m.pointToString(p1) as string);
    result.pointClone = safe(() => m.pointClone(p1));
    result.pointEquals = safe(() => m.pointEquals(p1, p2) as boolean);
    result.pointEqualsNe = safe(() => m.pointEquals(p1, p3) as boolean);
    result.pointHash = safe(() => m.pointHashCode(p1) as number);
    result.pointSerialize = safe(() => m.pointSerialize(p1));
    result.pointDeserialize = safe(() => m.pointDeserialize({ x: 5, y: 10 }));
    result.userProfileDebug = safe(() => m.userProfileToString(user) as string);
    result.userProfileEquals = safe(() => m.userProfileEquals(user, user) as boolean);

    return result;
}

// ── Inspect macro ──────────────────────────────────────────────

export interface InspectMacroResults {
    fieldMetadata: unknown;
    inspectableFields: unknown;
    clonedArrays: unknown;
    populatedCount: number | null;
}

function collectInspectMacro(): InspectMacroResults {
    const result: InspectMacroResults = {
        fieldMetadata: null,
        inspectableFields: null,
        clonedArrays: null,
        populatedCount: null
    };

    try {
        const model = new FormModel(
            'Test memo',
            'johndoe',
            'A test description',
            ['tag1', 'tag2'],
            null
        );

        if (typeof FormModel.fieldMetadata === 'function') {
            result.fieldMetadata = FormModel.fieldMetadata();
        }
        if (typeof model.getInspectableFields === 'function') {
            result.inspectableFields = model.getInspectableFields();
        }
        if (typeof model.cloneArrayFields === 'function') {
            result.clonedArrays = model.cloneArrayFields();
        }
        if (typeof model.countPopulatedFields === 'function') {
            result.populatedCount = model.countPopulatedFields();
        }
    } catch (e) {
        console.error('Inspect macro collection failed:', e);
    }

    return result;
}

// ── Nested deserialize ──────────────────────────────────────────

export interface NestedDeserResults {
    normal: unknown;
    missingFields: unknown;
    nullElement: unknown;
    mixedElements: unknown;
    recursiveActual: unknown;
}

function collectNestedDeser(): NestedDeserResults {
    return {
        normal: testNormal(),
        missingFields: testMissingFields(),
        nullElement: testNullElement(),
        mixedElements: testMixedElements(),
        recursiveActual: testRecursiveActual()
    };
}

// ── Proc macro derives (class) ──────────────────────────────────

export interface ProcMacroDeriveResults {
    debug: string | null;
    clone: unknown;
    equals: boolean | null;
    equalsSelf: boolean | null;
    hashCode: number | null;
    serialize: string | null;
    deserializeSuccess: unknown;
    deserializeBad: unknown;
    defaultValue: unknown;
}

function collectProcMacroDerives(): ProcMacroDeriveResults {
    const result: ProcMacroDeriveResults = {
        debug: null,
        clone: null,
        equals: null,
        equalsSelf: null,
        hashCode: null,
        serialize: null,
        deserializeSuccess: null,
        deserializeBad: null,
        defaultValue: null
    };

    try {
        if (typeof AllMacrosTestClass.toString === 'function') {
            result.debug = AllMacrosTestClass.toString(testInstance);
        }
        if (typeof AllMacrosTestClass.clone === 'function') {
            result.clone = AllMacrosTestClass.clone(testInstance);
        }
        if (typeof AllMacrosTestClass.equals === 'function') {
            result.equalsSelf = AllMacrosTestClass.equals(testInstance, testInstance);
            const other = new AllMacrosTestClass({
                id: 999,
                name: 'Other',
                email: 'x@y.com',
                secretToken: 'x',
                isActive: false,
                score: 0
            });
            result.equals = AllMacrosTestClass.equals(testInstance, other);
        }
        if (typeof AllMacrosTestClass.hashCode === 'function') {
            result.hashCode = AllMacrosTestClass.hashCode(testInstance);
        }
        if (typeof AllMacrosTestClass.serialize === 'function') {
            result.serialize = AllMacrosTestClass.serialize(testInstance);
        }
        if (typeof AllMacrosTestClass.deserialize === 'function') {
            result.deserializeSuccess = AllMacrosTestClass.deserialize({
                id: 1,
                name: 'OK',
                email: 'ok@ok.com',
                secretToken: 'tok',
                isActive: true,
                score: 50
            });
            result.deserializeBad = AllMacrosTestClass.deserialize(null);
        }
    } catch (e) {
        console.error('Proc macro derive collection failed:', e);
    }

    return result;
}

// ── Attribute macros (@traced) + call macros ($stringify, $concat_names)

export interface AttrMacroResults {
    /** Result of `tracedAdd(2, 3)` — the wrapper must preserve semantics. */
    addResult: number;
    /** Result of `tracedGreet("world")`. */
    greetResult: string;
    /** Call counts captured from `globalThis.__traced` after N invocations. */
    tracedAddCount: number;
    tracedGreetCount: number;
    /** Output of `$stringify(1 + 2 * 3)` — literal text from the source. */
    stringifiedExpr: unknown;
    /** Output of `$stringify(myVariable)`. */
    stringifiedIdent: unknown;
    /** Output of `$concat_names(user, name)`. */
    concatUserName: unknown;
    /** Output of `$concat_names(db, host)`. */
    concatDbHost: unknown;
}

function collectAttrMacros(): AttrMacroResults {
    type TracedWindow = { __traced: Record<string, number> };
    const g = globalThis as unknown as TracedWindow;
    // Reset any prior counters so we're measuring this run's invocations.
    g.__traced = {};

    const addResult = tracedAdd(2, 3);
    tracedAdd(10, 20);
    tracedAdd(100, 200);
    const greetResult = tracedGreet('world');

    const traced = g.__traced ?? {};

    return {
        addResult,
        greetResult,
        tracedAddCount: traced.tracedAdd ?? 0,
        tracedGreetCount: traced.tracedGreet ?? 0,
        stringifiedExpr,
        stringifiedIdent,
        concatUserName,
        concatDbHost
    };
}

// ── Public interface ────────────────────────────────────────────

export interface E2eResults {
    declarative: DeclarativeMacroResults;
    declarativeComplex: DeclarativeComplexResults;
    enums: EnumDeriveResults;
    typeAliases: TypeAliasDeriveResults;
    inspect: InspectMacroResults;
    nestedDeser: NestedDeserResults;
    procDerives: ProcMacroDeriveResults;
    attrMacros: AttrMacroResults;
}

export function runE2eHarness(): E2eResults {
    return {
        declarative: collectDeclarativeMacros(),
        declarativeComplex: collectDeclarativeComplex(),
        enums: collectEnumDerives(),
        typeAliases: collectTypeAliasDerives(),
        inspect: collectInspectMacro(),
        nestedDeser: collectNestedDeser(),
        procDerives: collectProcMacroDerives(),
        attrMacros: collectAttrMacros()
    };
}
