import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Gradient } from './gradient.svelte';
import { Custom } from './custom.svelte';
import { Ordinal } from './ordinal.svelte';
import { Cardinal } from './cardinal.svelte';

export type ColorsConfig = Cardinal | Ordinal | Custom | /** @default */ Gradient;

export function defaultValueColorsConfig(): ColorsConfig {
    return Gradient.defaultValue();
}

export function toStringifiedJSONColorsConfig(value: ColorsConfig): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeColorsConfig(value, ctx));
}
export function toObjectColorsConfig(value: ColorsConfig): unknown {
    const ctx = SerializeContext.create();
    return __serializeColorsConfig(value, ctx);
}
export function __serializeColorsConfig(value: ColorsConfig, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONColorsConfig(
    json: string,
    opts?: DeserializeOptions
): Result<ColorsConfig, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectColorsConfig(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectColorsConfig(
    obj: unknown,
    opts?: DeserializeOptions
): Result<ColorsConfig, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeColorsConfig(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'ColorsConfig.fromObject: root cannot be a forward reference'
                }
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
export function __deserializeColorsConfig(
    value: any,
    ctx: DeserializeContext
): ColorsConfig | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as ColorsConfig | PendingRef;
    }
    if (typeof value !== 'object' || value === null) {
        throw new DeserializeError([
            { field: '_root', message: 'ColorsConfig.__deserialize: expected an object' }
        ]);
    }
    const __typeName = (value as any).__type;
    if (typeof __typeName !== 'string') {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'ColorsConfig.__deserialize: missing __type field for union dispatch'
            }
        ]);
    }
    if (__typeName === 'Cardinal') {
        if (typeof (Cardinal as any)?.__deserialize === 'function') {
            return (Cardinal as any).__deserialize(value, ctx) as ColorsConfig;
        }
        return value as ColorsConfig;
    }
    if (__typeName === 'Ordinal') {
        if (typeof (Ordinal as any)?.__deserialize === 'function') {
            return (Ordinal as any).__deserialize(value, ctx) as ColorsConfig;
        }
        return value as ColorsConfig;
    }
    if (__typeName === 'Custom') {
        if (typeof (Custom as any)?.__deserialize === 'function') {
            return (Custom as any).__deserialize(value, ctx) as ColorsConfig;
        }
        return value as ColorsConfig;
    }
    if (__typeName === 'Gradient') {
        if (typeof (Gradient as any)?.__deserialize === 'function') {
            return (Gradient as any).__deserialize(value, ctx) as ColorsConfig;
        }
        return value as ColorsConfig;
    }
    throw new DeserializeError([
        {
            field: '_root',
            message:
                'ColorsConfig.__deserialize: unknown type "' +
                __typeName +
                '". Expected one of: Cardinal, Ordinal, Custom, Gradient'
        }
    ]);
}
export function isColorsConfig(value: unknown): value is ColorsConfig {
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

/** Per-variant error types */ export type CardinalErrorsColorsConfig = {
    _errors: Option<Array<string>>;
};
export type OrdinalErrorsColorsConfig = { _errors: Option<Array<string>> };
export type CustomErrorsColorsConfig = { _errors: Option<Array<string>> };
export type GradientErrorsColorsConfig = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type CardinalTaintedColorsConfig = {};
export type OrdinalTaintedColorsConfig = {};
export type CustomTaintedColorsConfig = {};
export type GradientTaintedColorsConfig = {}; /** Union error type */
export type ErrorsColorsConfig =
    | ({ _type: 'Cardinal' } & CardinalErrorsColorsConfig)
    | ({ _type: 'Ordinal' } & OrdinalErrorsColorsConfig)
    | ({ _type: 'Custom' } & CustomErrorsColorsConfig)
    | ({ _type: 'Gradient' } & GradientErrorsColorsConfig); /** Union tainted type */
export type TaintedColorsConfig =
    | ({ _type: 'Cardinal' } & CardinalTaintedColorsConfig)
    | ({ _type: 'Ordinal' } & OrdinalTaintedColorsConfig)
    | ({ _type: 'Custom' } & CustomTaintedColorsConfig)
    | ({
          _type: 'Gradient';
      } & GradientTaintedColorsConfig); /** Per-variant field controller types */
export interface CardinalFieldControllersColorsConfig {}
export interface OrdinalFieldControllersColorsConfig {}
export interface CustomFieldControllersColorsConfig {}
export interface GradientFieldControllersColorsConfig {} /** Union Gigaform interface with variant switching */
export interface GigaformColorsConfig {
    readonly currentVariant: 'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient';
    readonly data: ColorsConfig;
    readonly errors: ErrorsColorsConfig;
    readonly tainted: TaintedColorsConfig;
    readonly variants: VariantFieldsColorsConfig;
    switchVariant(variant: 'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient'): void;
    validate(): Result<ColorsConfig, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ColorsConfig>): void;
} /** Variant fields container */
export interface VariantFieldsColorsConfig {
    readonly Cardinal: { readonly fields: CardinalFieldControllersColorsConfig };
    readonly Ordinal: { readonly fields: OrdinalFieldControllersColorsConfig };
    readonly Custom: { readonly fields: CustomFieldControllersColorsConfig };
    readonly Gradient: { readonly fields: GradientFieldControllersColorsConfig };
} /** Gets default value for a specific variant */
function getDefaultForVariantColorsConfig(variant: string): ColorsConfig {
    switch (variant) {
        case 'Cardinal':
            return Cardinal.defaultValue() as ColorsConfig;
        case 'Ordinal':
            return Ordinal.defaultValue() as ColorsConfig;
        case 'Custom':
            return Custom.defaultValue() as ColorsConfig;
        case 'Gradient':
            return Gradient.defaultValue() as ColorsConfig;
        default:
            return Cardinal.defaultValue() as ColorsConfig;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormColorsConfig(initial?: ColorsConfig): GigaformColorsConfig {
    const initialVariant: 'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient' = 'Cardinal';
    let currentVariant = $state<'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient'>(initialVariant);
    let data = $state<ColorsConfig>(initial ?? getDefaultForVariantColorsConfig(initialVariant));
    let errors = $state<ErrorsColorsConfig>({} as ErrorsColorsConfig);
    let tainted = $state<TaintedColorsConfig>({} as TaintedColorsConfig);
    const variants: VariantFieldsColorsConfig = {
        Cardinal: {
            fields: {} as CardinalFieldControllersColorsConfig
        },
        Ordinal: {
            fields: {} as OrdinalFieldControllersColorsConfig
        },
        Custom: {
            fields: {} as CustomFieldControllersColorsConfig
        },
        Gradient: {
            fields: {} as GradientFieldControllersColorsConfig
        }
    };
    function switchVariant(variant: 'Cardinal' | 'Ordinal' | 'Custom' | 'Gradient'): void {
        currentVariant = variant;
        data = getDefaultForVariantColorsConfig(variant);
        errors = {} as ErrorsColorsConfig;
        tainted = {} as TaintedColorsConfig;
    }
    function validate(): Result<ColorsConfig, Array<{ field: string; message: string }>> {
        return ColorsConfig.fromObject(data);
    }
    function reset(overrides?: Partial<ColorsConfig>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantColorsConfig(currentVariant);
        errors = {} as ErrorsColorsConfig;
        tainted = {} as TaintedColorsConfig;
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
export function fromFormDataColorsConfig(
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
    return ColorsConfig.fromStringifiedJSON(JSON.stringify(obj));
}
