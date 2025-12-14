import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface ServiceDefaults {
    price: number;

    description: string;
}

export function defaultValueServiceDefaults(): ServiceDefaults {
    return { price: 0, description: '' } as ServiceDefaults;
}

export function toStringifiedJSONServiceDefaults(value: ServiceDefaults): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeServiceDefaults(value, ctx));
}
export function toObjectServiceDefaults(value: ServiceDefaults): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeServiceDefaults(value, ctx);
}
export function __serializeServiceDefaults(
    value: ServiceDefaults,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'ServiceDefaults', __id };
    result['price'] = value.price;
    result['description'] = value.description;
    return result;
}

export function fromStringifiedJSONServiceDefaults(
    json: string,
    opts?: DeserializeOptions
): Result<ServiceDefaults, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectServiceDefaults(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectServiceDefaults(
    obj: unknown,
    opts?: DeserializeOptions
): Result<ServiceDefaults, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeServiceDefaults(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'ServiceDefaults.fromObject: root cannot be a forward reference'
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
export function __deserializeServiceDefaults(
    value: any,
    ctx: DeserializeContext
): ServiceDefaults | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'ServiceDefaults.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('price' in obj)) {
        errors.push({ field: 'price', message: 'missing required field' });
    }
    if (!('description' in obj)) {
        errors.push({ field: 'description', message: 'missing required field' });
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
        const __raw_price = obj['price'] as number;
        instance.price = __raw_price;
    }
    {
        const __raw_description = obj['description'] as string;
        if (__raw_description.length === 0) {
            errors.push({ field: 'description', message: 'must not be empty' });
        }
        instance.description = __raw_description;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as ServiceDefaults;
}
export function validateFieldServiceDefaults<K extends keyof ServiceDefaults>(
    field: K,
    value: ServiceDefaults[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'description': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'description', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsServiceDefaults(
    partial: Partial<ServiceDefaults>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('description' in partial && partial.description !== undefined) {
        const __val = partial.description as string;
        if (__val.length === 0) {
            errors.push({ field: 'description', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeServiceDefaults(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'price' in o && 'description' in o;
}
export function isServiceDefaults(obj: unknown): obj is ServiceDefaults {
    if (!hasShapeServiceDefaults(obj)) {
        return false;
    }
    const result = fromObjectServiceDefaults(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsServiceDefaults = {
    _errors: Option<Array<string>>;
    price: Option<Array<string>>;
    description: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedServiceDefaults = {
    price: Option<boolean>;
    description: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersServiceDefaults {
    readonly price: FieldController<number>;
    readonly description: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformServiceDefaults {
    readonly data: ServiceDefaults;
    readonly errors: ErrorsServiceDefaults;
    readonly tainted: TaintedServiceDefaults;
    readonly fields: FieldControllersServiceDefaults;
    validate(): Result<ServiceDefaults, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ServiceDefaults>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormServiceDefaults(
    overrides?: Partial<ServiceDefaults>
): GigaformServiceDefaults {
    let data = $state({ ...ServiceDefaults.defaultValue(), ...overrides });
    let errors = $state<ErrorsServiceDefaults>({
        _errors: Option.none(),
        price: Option.none(),
        description: Option.none()
    });
    let tainted = $state<TaintedServiceDefaults>({
        price: Option.none(),
        description: Option.none()
    });
    const fields: FieldControllersServiceDefaults = {
        price: {
            path: ['price'] as const,
            name: 'price',
            constraints: { required: true },
            label: 'Price',
            get: () => data.price,
            set: (value: number) => {
                data.price = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.price,
            setError: (value: Option<Array<string>>) => {
                errors.price = value;
            },
            getTainted: () => tainted.price,
            setTainted: (value: Option<boolean>) => {
                tainted.price = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ServiceDefaults.validateField('price', data.price);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        description: {
            path: ['description'] as const,
            name: 'description',
            constraints: { required: true },
            label: 'Description',
            get: () => data.description,
            set: (value: string) => {
                data.description = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.description,
            setError: (value: Option<Array<string>>) => {
                errors.description = value;
            },
            getTainted: () => tainted.description,
            setTainted: (value: Option<boolean>) => {
                tainted.description = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = ServiceDefaults.validateField('description', data.description);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<ServiceDefaults, Array<{ field: string; message: string }>> {
        return ServiceDefaults.fromObject(data);
    }
    function reset(newOverrides?: Partial<ServiceDefaults>): void {
        data = { ...ServiceDefaults.defaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), price: Option.none(), description: Option.none() };
        tainted = { price: Option.none(), description: Option.none() };
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
export function fromFormDataServiceDefaults(
    formData: FormData
): Result<ServiceDefaults, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const priceStr = formData.get('price');
        obj.price = priceStr ? parseFloat(priceStr as string) : 0;
        if (obj.price !== undefined && isNaN(obj.price as number)) obj.price = 0;
    }
    obj.description = formData.get('description') ?? '';
    return ServiceDefaults.fromStringifiedJSON(JSON.stringify(obj));
}
