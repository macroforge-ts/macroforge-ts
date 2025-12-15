import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Table } from './table.svelte';

export type OverviewDisplay = /** @default */ 'Card' | 'Table';

export function defaultValueOverviewDisplay(): OverviewDisplay {
    return 'Card';
}

export function toStringifiedJSONOverviewDisplay(value: OverviewDisplay): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeOverviewDisplay(value, ctx));
}
export function toObjectOverviewDisplay(value: OverviewDisplay): unknown {
    const ctx = SerializeContext.create();
    return __serializeOverviewDisplay(value, ctx);
}
export function __serializeOverviewDisplay(value: OverviewDisplay, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONOverviewDisplay(
    json: string,
    opts?: DeserializeOptions
): Result<OverviewDisplay, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectOverviewDisplay(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectOverviewDisplay(
    obj: unknown,
    opts?: DeserializeOptions
): Result<OverviewDisplay, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeOverviewDisplay(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'OverviewDisplay.fromObject: root cannot be a forward reference'
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
export function __deserializeOverviewDisplay(
    value: any,
    ctx: DeserializeContext
): OverviewDisplay | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as OverviewDisplay | PendingRef;
    }
    const allowedValues = ['Card', 'Table'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for OverviewDisplay: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as OverviewDisplay;
}
export function isOverviewDisplay(value: unknown): value is OverviewDisplay {
    const allowedValues = ['Card', 'Table'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type CardErrorsOverviewDisplay = {
    _errors: Option<Array<string>>;
};
export type TableErrorsOverviewDisplay = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type CardTaintedOverviewDisplay = {};
export type TableTaintedOverviewDisplay = {}; /** Union error type */
export type ErrorsOverviewDisplay =
    | ({ _value: 'Card' } & CardErrorsOverviewDisplay)
    | ({ _value: 'Table' } & TableErrorsOverviewDisplay); /** Union tainted type */
export type TaintedOverviewDisplay =
    | ({ _value: 'Card' } & CardTaintedOverviewDisplay)
    | ({ _value: 'Table' } & TableTaintedOverviewDisplay); /** Per-variant field controller types */
export interface CardFieldControllersOverviewDisplay {}
export interface TableFieldControllersOverviewDisplay {} /** Union Gigaform interface with variant switching */
export interface GigaformOverviewDisplay {
    readonly currentVariant: 'Card' | 'Table';
    readonly data: OverviewDisplay;
    readonly errors: ErrorsOverviewDisplay;
    readonly tainted: TaintedOverviewDisplay;
    readonly variants: VariantFieldsOverviewDisplay;
    switchVariant(variant: 'Card' | 'Table'): void;
    validate(): Result<OverviewDisplay, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<OverviewDisplay>): void;
} /** Variant fields container */
export interface VariantFieldsOverviewDisplay {
    readonly Card: { readonly fields: CardFieldControllersOverviewDisplay };
    readonly Table: { readonly fields: TableFieldControllersOverviewDisplay };
} /** Gets default value for a specific variant */
function getDefaultForVariantOverviewDisplay(variant: string): OverviewDisplay {
    switch (variant) {
        case 'Card':
            return 'Card' as OverviewDisplay;
        case 'Table':
            return 'Table' as OverviewDisplay;
        default:
            return 'Card' as OverviewDisplay;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormOverviewDisplay(initial?: OverviewDisplay): GigaformOverviewDisplay {
    const initialVariant: 'Card' | 'Table' = (initial as 'Card' | 'Table') ?? 'Card';
    let currentVariant = $state<'Card' | 'Table'>(initialVariant);
    let data = $state<OverviewDisplay>(
        initial ?? getDefaultForVariantOverviewDisplay(initialVariant)
    );
    let errors = $state<ErrorsOverviewDisplay>({} as ErrorsOverviewDisplay);
    let tainted = $state<TaintedOverviewDisplay>({} as TaintedOverviewDisplay);
    const variants: VariantFieldsOverviewDisplay = {
        Card: {
            fields: {} as CardFieldControllersOverviewDisplay
        },
        Table: {
            fields: {} as TableFieldControllersOverviewDisplay
        }
    };
    function switchVariant(variant: 'Card' | 'Table'): void {
        currentVariant = variant;
        data = getDefaultForVariantOverviewDisplay(variant);
        errors = {} as ErrorsOverviewDisplay;
        tainted = {} as TaintedOverviewDisplay;
    }
    function validate(): Result<OverviewDisplay, Array<{ field: string; message: string }>> {
        return fromObjectOverviewDisplay(data);
    }
    function reset(overrides?: Partial<OverviewDisplay>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantOverviewDisplay(currentVariant);
        errors = {} as ErrorsOverviewDisplay;
        tainted = {} as TaintedOverviewDisplay;
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
export function fromFormDataOverviewDisplay(
    formData: FormData
): Result<OverviewDisplay, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as 'Card' | 'Table' | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'Card') {
    } else if (discriminant === 'Table') {
    }
    return fromStringifiedJSONOverviewDisplay(JSON.stringify(obj));
}
