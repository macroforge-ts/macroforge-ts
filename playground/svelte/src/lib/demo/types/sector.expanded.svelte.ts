import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { exitFail } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';

export type Sector = /** @default */ 'Residential' | 'Commercial';

export function sectorDefaultValue(): Sector {
    return 'Residential';
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function sectorSerialize(
    value: Sector
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(sectorSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function sectorSerializeWithContext(value: Sector, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === 'function') {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function sectorDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Sector> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = sectorDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Sector.deserialize: root cannot be a forward reference'
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
export function sectorDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Sector | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Sector | __mf_PendingRef;
    }
    const allowedValues = ['Residential', 'Commercial'] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Sector: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Sector;
}
export function sectorIs(value: unknown): value is Sector {
    const allowedValues = ['Residential', 'Commercial'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type SectorResidentialErrors = {
    _errors: Option<Array<string>>;
};
export type SectorCommercialErrors = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type SectorResidentialTainted = {};
export type SectorCommercialTainted = {}; /** Union error type */
export type SectorErrors =
    | ({ _value: 'Residential' } & SectorResidentialErrors)
    | ({ _value: 'Commercial' } & SectorCommercialErrors); /** Union tainted type */
export type SectorTainted =
    | ({ _value: 'Residential' } & SectorResidentialTainted)
    | ({
          _value: 'Commercial';
      } & SectorCommercialTainted); /** Per-variant field controller types */
export interface SectorResidentialFieldControllers {}
export interface SectorCommercialFieldControllers {} /** Union Gigaform interface with variant switching */
export interface SectorGigaform {
    readonly currentVariant: 'Residential' | 'Commercial';
    readonly data: Sector;
    readonly errors: SectorErrors;
    readonly tainted: SectorTainted;
    readonly variants: SectorVariantFields;
    switchVariant(variant: 'Residential' | 'Commercial'): void;
    validate(): Exit<Array<{ field: string; message: string }>, Sector>;
    reset(overrides?: Partial<Sector>): void;
} /** Variant fields container */
export interface SectorVariantFields {
    readonly Residential: { readonly fields: SectorResidentialFieldControllers };
    readonly Commercial: { readonly fields: SectorCommercialFieldControllers };
} /** Gets default value for a specific variant */
function sectorGetDefaultForVariant(variant: string): Sector {
    switch (variant) {
        case 'Residential':
            return 'Residential' as Sector;
        case 'Commercial':
            return 'Commercial' as Sector;
        default:
            return 'Residential' as Sector;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function sectorCreateForm(initial?: Sector): SectorGigaform {
    const initialVariant: 'Residential' | 'Commercial' =
        (initial as 'Residential' | 'Commercial') ?? 'Residential';
    let currentVariant = $state<'Residential' | 'Commercial'>(initialVariant);
    let data = $state<Sector>(initial ?? sectorGetDefaultForVariant(initialVariant));
    let errors = $state<SectorErrors>({} as SectorErrors);
    let tainted = $state<SectorTainted>({} as SectorTainted);
    const variants: SectorVariantFields = {
        Residential: { fields: {} as SectorResidentialFieldControllers },
        Commercial: { fields: {} as SectorCommercialFieldControllers }
    };
    function switchVariant(variant: 'Residential' | 'Commercial'): void {
        currentVariant = variant;
        data = sectorGetDefaultForVariant(variant);
        errors = {} as SectorErrors;
        tainted = {} as SectorTainted;
    }
    function validate(): Exit<Array<{ field: string; message: string }>, Sector> {
        return sectorDeserialize(data);
    }
    function reset(overrides?: Partial<Sector>): void {
        data = overrides ? (overrides as typeof data) : sectorGetDefaultForVariant(currentVariant);
        errors = {} as SectorErrors;
        tainted = {} as SectorTainted;
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
export function sectorFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Sector> {
    const discriminant = formData.get('_value') as 'Residential' | 'Commercial' | null;
    if (!discriminant) {
        return exitFail([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Residential') {
    } else if (discriminant === 'Commercial') {
    }
    return sectorDeserialize(obj);
}

export const Sector = {
    defaultValue: sectorDefaultValue,
    serialize: sectorSerialize,
    serializeWithContext: sectorSerializeWithContext,
    deserialize: sectorDeserialize,
    deserializeWithContext: sectorDeserializeWithContext,
    is: sectorIs,
    createForm: sectorCreateForm,
    fromFormData: sectorFromFormData
} as const;
