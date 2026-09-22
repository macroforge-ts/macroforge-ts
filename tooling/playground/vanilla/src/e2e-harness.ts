// Comprehensive e2e test harness.
//
// Imports all macro types and collects their results, which main.ts
// publishes as `vanillaPlayground.e2e` so Playwright specs can assert
// correctness without touching the DOM.

import type {
    AttrMacroResults,
    DeclarativeComplexResults,
    DeclarativeMacroResults,
    E2eResults,
    EnumDeriveResults,
    InspectMacroResults,
    NestedDeserResults,
    ProcMacroDeriveResults,
    TypeAliasDeriveResults
} from './e2e-results.ts';
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

// The `status*`, `priority*`, `color*`, `point*` and `userProfile*` helpers
// are emitted by the derives, so they exist only in the expanded module.
import {
    Color,
    colorEquals,
    colorToString,
    type Point,
    pointClone,
    pointDeserialize,
    pointEquals,
    pointHashCode,
    pointSerialize,
    pointToString,
    Priority,
    priorityEquals,
    priorityToString,
    Status,
    statusClone,
    statusDeserialize,
    statusEquals,
    statusHashCode,
    statusSerialize,
    statusToString,
    user,
    userProfileEquals,
    userProfileToString
} from './enum-type-examples.ts';

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

/**
 * Runs one derive helper in isolation, so a helper that throws is reported
 * and leaves `null` instead of losing the rest of the results.
 */
function probe<Value>(label: string, run: () => Value): Value | null {
    try {
        return run();
    } catch (error) {
        console.error(`${label} failed:`, error);
        return null;
    }
}

// ── Declarative macros ──────────────────────────────────────────

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

function collectEnumDerives(): EnumDeriveResults {
    return {
        statusDebug: probe('statusToString', () => statusToString(Status.Active)),
        statusClone: probe('statusClone', () => statusClone(Status.Pending)),
        statusEquals: probe('statusEquals', () => statusEquals(Status.Active, Status.Active)),
        statusHash: probe('statusHashCode', () => statusHashCode(Status.Active)),
        statusSerialize: probe('statusSerialize', () => statusSerialize(Status.Inactive)),
        statusDeserialize: probe('statusDeserialize', () => statusDeserialize('pending')),
        priorityDebug: probe('priorityToString', () => priorityToString(Priority.High)),
        priorityEquals: probe('priorityEquals', () => priorityEquals(Priority.Low, Priority.Low)),
        colorDebug: probe('colorToString', () => colorToString(Color.Red)),
        colorEquals: probe('colorEquals', () => colorEquals(Color.Blue, Color.Blue))
    };
}

// ── Type alias derives ──────────────────────────────────────────

function collectTypeAliasDerives(): TypeAliasDeriveResults {
    const origin: Point = { x: 10, y: 20 };
    const sameAsOrigin: Point = { x: 10, y: 20 };
    const elsewhere: Point = { x: 99, y: 1 };

    return {
        pointDebug: probe('pointToString', () => pointToString(origin)),
        pointClone: probe('pointClone', () => pointClone(origin)),
        pointEquals: probe('pointEquals', () => pointEquals(origin, sameAsOrigin)),
        pointEqualsNe: probe('pointEquals', () => pointEquals(origin, elsewhere)),
        pointHash: probe('pointHashCode', () => pointHashCode(origin)),
        pointSerialize: probe('pointSerialize', () => pointSerialize(origin)),
        pointDeserialize: probe('pointDeserialize', () => pointDeserialize({ x: 5, y: 10 })),
        userProfileDebug: probe('userProfileToString', () => userProfileToString(user)),
        userProfileEquals: probe('userProfileEquals', () => userProfileEquals(user, user))
    };
}

// ── Inspect macro ──────────────────────────────────────────────

function collectInspectMacro(): InspectMacroResults {
    const model = new FormModel(
        'Test memo',
        'johndoe',
        'A test description',
        ['tag1', 'tag2'],
        null
    );

    return {
        fieldMetadata: probe('fieldMetadata', () => FormModel.fieldMetadata()),
        inspectableFields: probe('getInspectableFields', () => model.getInspectableFields()),
        clonedArrays: probe('cloneArrayFields', () => model.cloneArrayFields()),
        populatedCount: probe('countPopulatedFields', () => model.countPopulatedFields())
    };
}

// ── Nested deserialize ──────────────────────────────────────────

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

function collectProcMacroDerives(): ProcMacroDeriveResults {
    const other = new AllMacrosTestClass({
        id: 999,
        name: 'Other',
        email: 'x@y.com',
        secretToken: 'x',
        isActive: false,
        score: 0
    });

    return {
        debug: probe('toString', () => AllMacrosTestClass.toString(testInstance)),
        clone: probe('clone', () => AllMacrosTestClass.clone(testInstance)),
        equals: probe('equals', () => AllMacrosTestClass.equals(testInstance, other)),
        equalsSelf: probe('equals', () => AllMacrosTestClass.equals(testInstance, testInstance)),
        hashCode: probe('hashCode', () => AllMacrosTestClass.hashCode(testInstance)),
        serialize: probe('serialize', () => AllMacrosTestClass.serialize(testInstance)),
        deserializeSuccess: probe('deserialize', () =>
            AllMacrosTestClass.deserialize({
                id: 1,
                name: 'OK',
                email: 'ok@ok.com',
                secretToken: 'tok',
                isActive: true,
                score: 50
            })),
        deserializeBad: probe('deserialize', () => AllMacrosTestClass.deserialize(null))
    };
}

// ── Attribute macros (@traced) + call macros ($stringify, $concat_names)

declare global {
    /** Per-function call counts the `@traced` wrapper records. */
    var __traced: Record<string, number> | undefined;
}

function collectAttrMacros(): AttrMacroResults {
    // Reset any prior counters so we're measuring this run's invocations.
    globalThis.__traced = {};

    const addResult = tracedAdd(2, 3);
    tracedAdd(10, 20);
    tracedAdd(100, 200);
    const greetResult = tracedGreet('world');

    return {
        addResult,
        greetResult,
        tracedAddCount: globalThis.__traced?.tracedAdd ?? 0,
        tracedGreetCount: globalThis.__traced?.tracedGreet ?? 0,
        stringifiedExpr,
        stringifiedIdent,
        concatUserName,
        concatDbHost
    };
}

// ── Public interface ────────────────────────────────────────────

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
