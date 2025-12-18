import { gradientDefaultValue } from './gradient.svelte';
import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { cardinalDeserializeWithContext } from './cardinal.svelte';
import { customDeserializeWithContext } from './custom.svelte';
import { gradientDeserializeWithContext } from './gradient.svelte';
import { ordinalDeserializeWithContext } from './ordinal.svelte';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
import { cardinalDefaultValue } from './cardinal.svelte';
import { customDefaultValue } from './custom.svelte';
import { ordinalDefaultValue } from './ordinal.svelte';
/** import macro {Gigaform} from "@playground/macro"; */

import type { Gradient } from './gradient.svelte';
import type { Custom } from './custom.svelte';
import type { Ordinal } from './ordinal.svelte';
import type { Cardinal } from './cardinal.svelte';

export type ColorsConfig = Cardinal | Ordinal | Custom | /** @default */ Gradient;

export function colorsConfigDefaultValue(): ColorsConfig {
    return gradientDefaultValue();
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function colorsConfigSerialize(
    value: ColorsConfig
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(colorsConfigSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function colorsConfigSerializeWithContext(
    value: ColorsConfig,
    ctx: __mf_SerializeContext
): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function colorsConfigDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, ColorsConfig> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = colorsConfigDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'ColorsConfig.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function colorsConfigDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): ColorsConfig | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as ColorsConfig | __mf_PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'ColorsConfig.deserializeWithContext: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'ColorsConfig.deserializeWithContext: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'Cardinal') {
        return cardinalDeserializeWithContext(value, ctx) as ColorsConfig;
    }
    if (__typeName === 'Ordinal') {
        return ordinalDeserializeWithContext(value, ctx) as ColorsConfig;
    }
    if (__typeName === 'Custom') {
        return customDeserializeWithContext(value, ctx) as ColorsConfig;
    }
    if (__typeName === 'Gradient') {
        return gradientDeserializeWithContext(value, ctx) as ColorsConfig;
    }
    throw new __mf_DeserializeError([
        {
            field: '_root',
            message:
                'ColorsConfig.deserializeWithContext: unknown type "' +
                __typeName +
                '". Expected one of: Cardinal, Ordinal, Custom, Gradient'
        }
    ]);
}
export function colorsConfigIs(value: unknown): value is ColorsConfig {
    if (typeof value !== 'object' || value === null) {
        return false;
    }
    const __typeName = (value as any).__type;
    return (
        __typeName === 'Cardinal' ||
        __typeName === 'Ordinal' ||
        __typeName === 'Custom' ||
        __typeName === 'Gradient'
    );
}

/** Per-variant error types */ export type ColorsConfigCardinalErrors = {
    _errors: Option<Array<string>>;
};
export type ColorsConfigOrdinalErrors = { _errors: Option<Array<string>> };
export type ColorsConfigCustomErrors = { _errors: Option<Array<string>> };
export type ColorsConfigGradientErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type ColorsConfigCardinalTainted = {};
export type ColorsConfigOrdinalTainted = {};
export type ColorsConfigCustomTainted = {};
export type ColorsConfigGradientTainted = {}; /** Union error type */
export type ColorsConfigErrors =
    | ({ _type: 'Cardinal' } & ColorsConfigCardinalErrors)
    | ({ _type: 'Ordinal' } & ColorsConfigOrdinalErrors)
    | ({ _type: 'Custom' } & ColorsConfigCustomErrors)
    | ({ _type: 'Gradient' } & ColorsConfigGradientErrors); /** Union tainted type */
export type ColorsConfigTainted =
    | ({ _type: 'Cardinal' } & ColorsConfigCardinalTainted)
    | ({ _type: 'Ordinal' } & ColorsConfigOrdinalTainted)
    | ({ _type: 'Custom' } & ColorsConfigCustomTainted)
    | ({
          _type: 'Gradient';
      } & ColorsConfigGradientTainted); /** Per-variant field controller types */
export interface ColorsConfigCardinalFieldControllers {}
export interface ColorsConfigOrdinalFieldControllers {}
export interface ColorsConfigCustomFieldControllers {}
export interface ColorsConfigGradientFieldControllers {} /** Union Gigaform interface with variant switching */
export interface ColorsConfigGigaform {
    readonly currentVariant: 'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient';
    readonly data: ColorsConfig;
    readonly errors: ColorsConfigErrors;
    readonly tainted: ColorsConfigTainted;
    readonly variants: ColorsConfigVariantFields;
    switchVariant(variant: 'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient'): void;
    validate(): Result<ColorsConfig, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ColorsConfig>): void;
} /** Variant fields container */
export interface ColorsConfigVariantFields {
    readonly Cardinal: { readonly fields: ColorsConfigCardinalFieldControllers };
    readonly Ordinal: { readonly fields: ColorsConfigOrdinalFieldControllers };
    readonly Custom: { readonly fields: ColorsConfigCustomFieldControllers };
    readonly Gradient: { readonly fields: ColorsConfigGradientFieldControllers };
} /** Gets default value for a specific variant */
function colorsConfigGetDefaultForVariant(variant: string): ColorsConfig {
    switch (variant) {
        case 'Cardinal':
            return cardinalDefaultValue() as ColorsConfig;
        case 'Ordinal':
            return ordinalDefaultValue() as ColorsConfig;
        case 'Custom':
            return customDefaultValue() as ColorsConfig;
        case 'Gradient':
            return gradientDefaultValue() as ColorsConfig;
        default:
            return cardinalDefaultValue() as ColorsConfig;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function colorsConfigCreateForm(initial?: ColorsConfig): ColorsConfigGigaform {
    const initialVariant: 'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient' = 'Cardinal';
    let currentVariant = $state<'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient'>(initialVariant);
    let data = $state<ColorsConfig>(initial ?? colorsConfigGetDefaultForVariant(initialVariant));
    let errors = $state<ColorsConfigErrors>({} as ColorsConfigErrors);
    let tainted = $state<ColorsConfigTainted>({} as ColorsConfigTainted);
    const variants: ColorsConfigVariantFields = {
        Cardinal: { fields: {} as ColorsConfigCardinalFieldControllers },
        Ordinal: { fields: {} as ColorsConfigOrdinalFieldControllers },
        Custom: { fields: {} as ColorsConfigCustomFieldControllers },
        Gradient: { fields: {} as ColorsConfigGradientFieldControllers }
    };
    function switchVariant(variant: 'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient'): void {
        currentVariant = variant;
        data = colorsConfigGetDefaultForVariant(variant);
        errors = {} as ColorsConfigErrors;
        tainted = {} as ColorsConfigTainted;
    }
    function validate(): Result<ColorsConfig, Array<{ field: string; message: string }>> {
        return colorsConfigDeserialize(data);
    }
    function reset(overrides?: Partial<ColorsConfig>): void {
        data = overrides
            ? (overrides as typeof data)
            : colorsConfigGetDefaultForVariant(currentVariant);
        errors = {} as ColorsConfigErrors;
        tainted = {} as ColorsConfigTainted;
    }
    return {
        get currentVariant() {
            return currentVariant;
        },
        get data() {
            return data;
        },
        set data(v) {
            data = v;
        },
        get errors() {
            return errors;
        },
        set errors(v) {
            errors = v;
        },
        get tainted() {
            return tainted;
        },
        set tainted(v) {
            tainted = v;
        },
        variants,
        switchVariant,
        validate,
        reset
    };
} /** Parses FormData for union type, determining variant from discriminant field */
export function colorsConfigFromFormData(
    formData: FormData
): Result<ColorsConfig, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_type') as
        | 'Cardinal'
        | 'Ordinal'
        | 'Custom'
        | 'Gradient'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_type', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._type = discriminant;
    if (discriminant === 'Cardinal') {
    } else if (discriminant === 'Ordinal') {
    } else if (discriminant === 'Custom') {
    } else if (discriminant === 'Gradient') {
    }
    return colorsConfigDeserialize(obj);
}

export const ColorsConfig = {
    defaultValue: colorsConfigDefaultValue,
    serialize: colorsConfigSerialize,
    serializeWithContext: colorsConfigSerializeWithContext,
    deserialize: colorsConfigDeserialize,
    deserializeWithContext: colorsConfigDeserializeWithContext,
    is: colorsConfigIs,
    createForm: colorsConfigCreateForm,
    fromFormData: colorsConfigFromFormData
} as const;
