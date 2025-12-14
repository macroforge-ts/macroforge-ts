import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import { Item } from './item.svelte';

export interface BilledItem {
    item: Item;

    quantity: number;

    taxed: boolean;

    upsale: boolean;
}

export function defaultValueBilledItem(): BilledItem {
    return { item: '', quantity: 0, taxed: false, upsale: false } as BilledItem;
}

export function toStringifiedJSONBilledItem(value: BilledItem): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeBilledItem(value, ctx));
}
export function toObjectBilledItem(value: BilledItem): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeBilledItem(value, ctx);
}
export function __serializeBilledItem(
    value: BilledItem,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'BilledItem', __id };
    result['item'] =
        typeof (value.item as any)?.__serialize === 'function'
            ? (value.item as any).__serialize(ctx)
            : value.item;
    result['quantity'] = value.quantity;
    result['taxed'] = value.taxed;
    result['upsale'] = value.upsale;
    return result;
}

export function fromStringifiedJSONBilledItem(
    json: string,
    opts?: DeserializeOptions
): Result<BilledItem, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectBilledItem(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectBilledItem(
    obj: unknown,
    opts?: DeserializeOptions
): Result<BilledItem, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeBilledItem(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'BilledItem.fromObject: root cannot be a forward reference'
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
export function __deserializeBilledItem(
    value: any,
    ctx: DeserializeContext
): BilledItem | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'BilledItem.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('item' in obj)) {
        errors.push({ field: 'item', message: 'missing required field' });
    }
    if (!('quantity' in obj)) {
        errors.push({ field: 'quantity', message: 'missing required field' });
    }
    if (!('taxed' in obj)) {
        errors.push({ field: 'taxed', message: 'missing required field' });
    }
    if (!('upsale' in obj)) {
        errors.push({ field: 'upsale', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_item = obj['item'] as Item;
        {
            const __result = Item.__deserialize(__raw_item, ctx);
            ctx.assignOrDefer(instance, 'item', __result);
        }
    }
    {
        const __raw_quantity = obj['quantity'] as number;
        instance.quantity = __raw_quantity;
    }
    {
        const __raw_taxed = obj['taxed'] as boolean;
        instance.taxed = __raw_taxed;
    }
    {
        const __raw_upsale = obj['upsale'] as boolean;
        instance.upsale = __raw_upsale;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as BilledItem;
}
export function validateFieldBilledItem<K extends keyof BilledItem>(
    field: K,
    value: BilledItem[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsBilledItem(
    partial: Partial<BilledItem>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeBilledItem(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'item' in o && 'quantity' in o && 'taxed' in o && 'upsale' in o;
}
export function isBilledItem(obj: unknown): obj is BilledItem {
    if (!hasShapeBilledItem(obj)) {
        return false;
    }
    const result = fromObjectBilledItem(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsBilledItem = {
    _errors: Option<Array<string>>;
    item: Option<Array<string>>;
    quantity: Option<Array<string>>;
    taxed: Option<Array<string>>;
    upsale: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedBilledItem = {
    item: Option<boolean>;
    quantity: Option<boolean>;
    taxed: Option<boolean>;
    upsale: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersBilledItem {
    readonly item: FieldController<Item>;
    readonly quantity: FieldController<number>;
    readonly taxed: FieldController<boolean>;
    readonly upsale: FieldController<boolean>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformBilledItem {
    readonly data: BilledItem;
    readonly errors: ErrorsBilledItem;
    readonly tainted: TaintedBilledItem;
    readonly fields: FieldControllersBilledItem;
    validate(): Result<BilledItem, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<BilledItem>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormBilledItem(overrides?: Partial<BilledItem>): GigaformBilledItem {
    let data = $state({ ...BilledItem.defaultValue(), ...overrides });
    let errors = $state<ErrorsBilledItem>({
        _errors: Option.none(),
        item: Option.none(),
        quantity: Option.none(),
        taxed: Option.none(),
        upsale: Option.none()
    });
    let tainted = $state<TaintedBilledItem>({
        item: Option.none(),
        quantity: Option.none(),
        taxed: Option.none(),
        upsale: Option.none()
    });
    const fields: FieldControllersBilledItem = {
        item: {
            path: ['item'] as const,
            name: 'item',
            constraints: { required: true },
            label: 'Item',
            get: () => data.item,
            set: (value: Item) => {
                data.item = value;
            },
            transform: (value: Item): Item => value,
            getError: () => errors.item,
            setError: (value: Option<Array<string>>) => {
                errors.item = value;
            },
            getTainted: () => tainted.item,
            setTainted: (value: Option<boolean>) => {
                tainted.item = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = BilledItem.validateField('item', data.item);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        quantity: {
            path: ['quantity'] as const,
            name: 'quantity',
            constraints: { required: true },
            label: 'Quantity',
            get: () => data.quantity,
            set: (value: number) => {
                data.quantity = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.quantity,
            setError: (value: Option<Array<string>>) => {
                errors.quantity = value;
            },
            getTainted: () => tainted.quantity,
            setTainted: (value: Option<boolean>) => {
                tainted.quantity = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = BilledItem.validateField('quantity', data.quantity);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        taxed: {
            path: ['taxed'] as const,
            name: 'taxed',
            constraints: { required: true },
            label: 'Taxed',
            get: () => data.taxed,
            set: (value: boolean) => {
                data.taxed = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.taxed,
            setError: (value: Option<Array<string>>) => {
                errors.taxed = value;
            },
            getTainted: () => tainted.taxed,
            setTainted: (value: Option<boolean>) => {
                tainted.taxed = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = BilledItem.validateField('taxed', data.taxed);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        upsale: {
            path: ['upsale'] as const,
            name: 'upsale',
            constraints: { required: true },
            label: 'Upsale',
            get: () => data.upsale,
            set: (value: boolean) => {
                data.upsale = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.upsale,
            setError: (value: Option<Array<string>>) => {
                errors.upsale = value;
            },
            getTainted: () => tainted.upsale,
            setTainted: (value: Option<boolean>) => {
                tainted.upsale = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = BilledItem.validateField('upsale', data.upsale);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<BilledItem, Array<{ field: string; message: string }>> {
        return BilledItem.fromObject(data);
    }
    function reset(newOverrides?: Partial<BilledItem>): void {
        data = { ...BilledItem.defaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            item: Option.none(),
            quantity: Option.none(),
            taxed: Option.none(),
            upsale: Option.none()
        };
        tainted = {
            item: Option.none(),
            quantity: Option.none(),
            taxed: Option.none(),
            upsale: Option.none()
        };
    }
    return {
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
        fields,
        validate,
        reset
    };
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataBilledItem(
    formData: FormData
): Result<BilledItem, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        // Collect nested object fields with prefix "item."
        const itemObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('item.')) {
                const fieldName = key.slice('item.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = itemObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.item = itemObj;
    }
    {
        const quantityStr = formData.get('quantity');
        obj.quantity = quantityStr ? parseFloat(quantityStr as string) : 0;
        if (obj.quantity !== undefined && isNaN(obj.quantity as number)) obj.quantity = 0;
    }
    {
        const taxedVal = formData.get('taxed');
        obj.taxed = taxedVal === 'true' || taxedVal === 'on' || taxedVal === '1';
    }
    {
        const upsaleVal = formData.get('upsale');
        obj.upsale = upsaleVal === 'true' || upsaleVal === 'on' || upsaleVal === '1';
    }
    return BilledItem.fromStringifiedJSON(JSON.stringify(obj));
}
