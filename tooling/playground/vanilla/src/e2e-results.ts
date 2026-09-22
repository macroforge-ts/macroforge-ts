// Results the e2e harness publishes for the Playwright specs. The specs
// compile this contract but not the harness, so it takes types only from
// fixtures that type-check on their own. Values of declarative and call
// macros stay `unknown`: the specs compare them by value, and typing them
// would pull in fixtures whose macros resolve across files.
import type { AllMacrosTestClass } from './all-macros-test.ts';
import type * as enumTypes from './enum-type-examples.ts';
import type { FormModel } from './form-model.ts';
import type * as runtimeDeser from './runtime-deser-test.ts';

/** `null` when the derive's helper was missing or threw. */
type Probed<Value> = Value | null;

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
    hygieneCheck: { callerTemp: number; macroResult: number };
    sqPlus1: unknown;
    samplePatch: unknown;
    definitelyHello: unknown;
}

export interface EnumDeriveResults {
    statusDebug: Probed<string>;
    statusClone: Probed<ReturnType<typeof enumTypes.statusClone>>;
    statusEquals: Probed<boolean>;
    statusHash: Probed<number>;
    statusSerialize: Probed<ReturnType<typeof enumTypes.statusSerialize>>;
    statusDeserialize: Probed<ReturnType<typeof enumTypes.statusDeserialize>>;
    priorityDebug: Probed<string>;
    priorityEquals: Probed<boolean>;
    colorDebug: Probed<string>;
    colorEquals: Probed<boolean>;
}

export interface TypeAliasDeriveResults {
    pointDebug: Probed<string>;
    pointClone: Probed<ReturnType<typeof enumTypes.pointClone>>;
    pointEquals: Probed<boolean>;
    pointEqualsNe: Probed<boolean>;
    pointHash: Probed<number>;
    pointSerialize: Probed<ReturnType<typeof enumTypes.pointSerialize>>;
    pointDeserialize: Probed<ReturnType<typeof enumTypes.pointDeserialize>>;
    userProfileDebug: Probed<string>;
    userProfileEquals: Probed<boolean>;
}

export interface InspectMacroResults {
    fieldMetadata: Probed<ReturnType<typeof FormModel.fieldMetadata>>;
    inspectableFields: Probed<ReturnType<FormModel['getInspectableFields']>>;
    clonedArrays: Probed<ReturnType<FormModel['cloneArrayFields']>>;
    populatedCount: Probed<number>;
}

export interface NestedDeserResults {
    normal: ReturnType<typeof runtimeDeser.testNormal>;
    missingFields: ReturnType<typeof runtimeDeser.testMissingFields>;
    nullElement: ReturnType<typeof runtimeDeser.testNullElement>;
    mixedElements: ReturnType<typeof runtimeDeser.testMixedElements>;
    recursiveActual: runtimeDeser.RecursiveDeserResult;
}

export interface ProcMacroDeriveResults {
    debug: Probed<string>;
    clone: Probed<AllMacrosTestClass>;
    equals: Probed<boolean>;
    equalsSelf: Probed<boolean>;
    hashCode: Probed<number>;
    serialize: Probed<string>;
    deserializeSuccess: Probed<ReturnType<typeof AllMacrosTestClass.deserialize>>;
    deserializeBad: Probed<ReturnType<typeof AllMacrosTestClass.deserialize>>;
}

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
