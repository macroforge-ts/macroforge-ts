import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type Sector = /** @default */ 'Residential' | 'Commercial';

export function defaultValueSector(): Sector {
    return 'Residential';
}

export function toStringifiedJSONSector(value: Sector): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeSector(value, ctx));
}
export function toObjectSector(value: Sector): unknown {
    const ctx = SerializeContext.create();
    return __serializeSector(value, ctx);
}
export function __serializeSector(value: Sector, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONSector(
    json: string,
    opts?: DeserializeOptions
): Result<Sector, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectSector(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectSector(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Sector, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeSector(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Sector.fromObject: root cannot be a forward reference' }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function __deserializeSector(value: any, ctx: DeserializeContext): Sector | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Sector | PendingRef;
    }
    const allowedValues = ['Residential', 'Commercial'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
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
export function isSector(value: unknown): value is Sector {
    const allowedValues = ['Residential', 'Commercial'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type ResidentialErrorsSector = {
    _errors: Option<Array<string>>;
};
export type CommercialErrorsSector = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type ResidentialTaintedSector = {};
export type CommercialTaintedSector = {}; /** Union error type */
export type ErrorsSector =
    | ({ _value: 'Residential' } & ResidentialErrorsSector)
    | ({ _value: 'Commercial' } & CommercialErrorsSector); /** Union tainted type */
export type TaintedSector =
    | ({ _value: 'Residential' } & ResidentialTaintedSector)
    | ({
          _value: 'Commercial';
      } & CommercialTaintedSector); /** Per-variant field controller types */
export interface ResidentialFieldControllersSector {}
export interface CommercialFieldControllersSector {} /** Union Gigaform interface with variant switching */
export interface GigaformSector {
    readonly currentVariant: 'Residential' | 'Commercial';
    readonly data: Sector;
    readonly errors: ErrorsSector;
    readonly tainted: TaintedSector;
    readonly variants: VariantFieldsSector;
    switchVariant(variant: 'Residential' | 'Commercial'): void;
    validate(): Result<Sector, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Sector>): void;
} /** Variant fields container */
export interface VariantFieldsSector {
    readonly Residential: { readonly fields: ResidentialFieldControllersSector };
    readonly Commercial: { readonly fields: CommercialFieldControllersSector };
} /** Gets default value for a specific variant */
function getDefaultForVariantSector(variant: string): Sector {
    switch (variant) {
        case 'Residential':
            return 'Residential' as Sector;
        case 'Commercial':
            return 'Commercial' as Sector;
        default:
            return 'Residential' as Sector;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormSector(initial?: Sector): GigaformSector {
    const initialVariant: 'Residential' | 'Commercial' =
        (initial as 'Residential' | 'Commercial') ?? 'Residential';
    let currentVariant = $state<'Residential' | 'Commercial'>(initialVariant);
    let data = $state<Sector>(initial ?? getDefaultForVariantSector(initialVariant));
    let errors = $state<ErrorsSector>({} as ErrorsSector);
    let tainted = $state<TaintedSector>({} as TaintedSector);
    const variants: VariantFieldsSector = {
        Residential: {
            fields: {} as ResidentialFieldControllersSector
        },
        Commercial: {
            fields: {} as CommercialFieldControllersSector
        }
    };
    function switchVariant(variant: 'Residential' | 'Commercial'): void {
        currentVariant = variant;
        data = getDefaultForVariantSector(variant);
        errors = {} as ErrorsSector;
        tainted = {} as TaintedSector;
    }
    function validate(): Result<Sector, Array<{ field: string; message: string }>> {
        return Sector.fromObject(data);
    }
    function reset(overrides?: Partial<Sector>): void {
        data = overrides ? (overrides as typeof data) : getDefaultForVariantSector(currentVariant);
        errors = {} as ErrorsSector;
        tainted = {} as TaintedSector;
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
export function fromFormDataSector(
    formData: FormData
): Result<Sector, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as 'Residential' | 'Commercial' | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Residential') {
    } else if (discriminant === 'Commercial') {
    }
    return Sector.fromStringifiedJSON(JSON.stringify(obj));
}
