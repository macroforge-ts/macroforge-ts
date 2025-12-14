import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

export type RowHeight = 'ExtraSmall' | 'Small' | /** @default */ 'Medium' | 'Large';

export function defaultValueRowHeight(): RowHeight {
    return 'Medium';
}

export function toStringifiedJSONRowHeight(value: RowHeight): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeRowHeight(value, ctx));
}
export function toObjectRowHeight(value: RowHeight): unknown {
    const ctx = SerializeContext.create();
    return __serializeRowHeight(value, ctx);
}
export function __serializeRowHeight(value: RowHeight, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONRowHeight(
    json: string,
    opts?: DeserializeOptions
): Result<RowHeight, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectRowHeight(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectRowHeight(
    obj: unknown,
    opts?: DeserializeOptions
): Result<RowHeight, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeRowHeight(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'RowHeight.fromObject: root cannot be a forward reference'
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
export function __deserializeRowHeight(
    value: any,
    ctx: DeserializeContext
): RowHeight | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as RowHeight | PendingRef;
    }
    const allowedValues = ['ExtraSmall', 'Small', 'Medium', 'Large'] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for RowHeight: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as RowHeight;
}
export function isRowHeight(value: unknown): value is RowHeight {
    const allowedValues = ['ExtraSmall', 'Small', 'Medium', 'Large'] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type ExtraSmallErrorsRowHeight = {
    _errors: Option<Array<string>>;
};
export type SmallErrorsRowHeight = { _errors: Option<Array<string>> };
export type MediumErrorsRowHeight = { _errors: Option<Array<string>> };
export type LargeErrorsRowHeight = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type ExtraSmallTaintedRowHeight = {};
export type SmallTaintedRowHeight = {};
export type MediumTaintedRowHeight = {};
export type LargeTaintedRowHeight = {}; /** Union error type */
export type ErrorsRowHeight =
    | ({ _value: 'ExtraSmall' } & ExtraSmallErrorsRowHeight)
    | ({ _value: 'Small' } & SmallErrorsRowHeight)
    | ({ _value: 'Medium' } & MediumErrorsRowHeight)
    | ({ _value: 'Large' } & LargeErrorsRowHeight); /** Union tainted type */
export type TaintedRowHeight =
    | ({ _value: 'ExtraSmall' } & ExtraSmallTaintedRowHeight)
    | ({ _value: 'Small' } & SmallTaintedRowHeight)
    | ({ _value: 'Medium' } & MediumTaintedRowHeight)
    | ({ _value: 'Large' } & LargeTaintedRowHeight); /** Per-variant field controller types */
export interface ExtraSmallFieldControllersRowHeight {}
export interface SmallFieldControllersRowHeight {}
export interface MediumFieldControllersRowHeight {}
export interface LargeFieldControllersRowHeight {} /** Union Gigaform interface with variant switching */
export interface GigaformRowHeight {
    readonly currentVariant: 'ExtraSmall' | 'Small' | 'Medium' | 'Large';
    readonly data: RowHeight;
    readonly errors: ErrorsRowHeight;
    readonly tainted: TaintedRowHeight;
    readonly variants: VariantFieldsRowHeight;
    switchVariant(variant: 'ExtraSmall' | 'Small' | 'Medium' | 'Large'): void;
    validate(): Result<RowHeight, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<RowHeight>): void;
} /** Variant fields container */
export interface VariantFieldsRowHeight {
    readonly ExtraSmall: { readonly fields: ExtraSmallFieldControllersRowHeight };
    readonly Small: { readonly fields: SmallFieldControllersRowHeight };
    readonly Medium: { readonly fields: MediumFieldControllersRowHeight };
    readonly Large: { readonly fields: LargeFieldControllersRowHeight };
} /** Gets default value for a specific variant */
function getDefaultForVariantRowHeight(variant: string): RowHeight {
    switch (variant) {
        case 'ExtraSmall':
            return 'ExtraSmall' as RowHeight;
        case 'Small':
            return 'Small' as RowHeight;
        case 'Medium':
            return 'Medium' as RowHeight;
        case 'Large':
            return 'Large' as RowHeight;
        default:
            return 'ExtraSmall' as RowHeight;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormRowHeight(initial?: RowHeight): GigaformRowHeight {
    const initialVariant: 'ExtraSmall' | 'Small' | 'Medium' | 'Large' =
        (initial as 'ExtraSmall' | 'Small' | 'Medium' | 'Large') ?? 'ExtraSmall';
    let currentVariant = $state<'ExtraSmall' | 'Small' | 'Medium' | 'Large'>(initialVariant);
    let data = $state<RowHeight>(initial ?? getDefaultForVariantRowHeight(initialVariant));
    let errors = $state<ErrorsRowHeight>({} as ErrorsRowHeight);
    let tainted = $state<TaintedRowHeight>({} as TaintedRowHeight);
    const variants: VariantFieldsRowHeight = {
        ExtraSmall: {
            fields: {} as ExtraSmallFieldControllersRowHeight
        },
        Small: {
            fields: {} as SmallFieldControllersRowHeight
        },
        Medium: {
            fields: {} as MediumFieldControllersRowHeight
        },
        Large: {
            fields: {} as LargeFieldControllersRowHeight
        }
    };
    function switchVariant(variant: 'ExtraSmall' | 'Small' | 'Medium' | 'Large'): void {
        currentVariant = variant;
        data = getDefaultForVariantRowHeight(variant);
        errors = {} as ErrorsRowHeight;
        tainted = {} as TaintedRowHeight;
    }
    function validate(): Result<RowHeight, Array<{ field: string; message: string }>> {
        return RowHeight.fromObject(data);
    }
    function reset(overrides?: Partial<RowHeight>): void {
        data = overrides
            ? (overrides as typeof data)
            : getDefaultForVariantRowHeight(currentVariant);
        errors = {} as ErrorsRowHeight;
        tainted = {} as TaintedRowHeight;
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
export function fromFormDataRowHeight(
    formData: FormData
): Result<RowHeight, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'ExtraSmall'
        | 'Small'
        | 'Medium'
        | 'Large'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'ExtraSmall') {
    } else if (discriminant === 'Small') {
    } else if (discriminant === 'Medium') {
    } else if (discriminant === 'Large') {
    }
    return RowHeight.fromStringifiedJSON(JSON.stringify(obj));
}
