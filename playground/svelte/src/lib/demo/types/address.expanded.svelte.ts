import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Address {
    street: string;

    city: string;

    state: string;

    zipcode: string;
}

export function defaultValueAddress(): Address {
    return { street: '', city: '', state: '', zipcode: '' } as Address;
}

export function toStringifiedJSONAddress(value: Address): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeAddress(value, ctx));
}
export function toObjectAddress(value: Address): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeAddress(value, ctx);
}
export function __serializeAddress(value: Address, ctx: SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Address', __id };
    result['street'] = value.street;
    result['city'] = value.city;
    result['state'] = value.state;
    result['zipcode'] = value.zipcode;
    return result;
}

export function fromStringifiedJSONAddress(
    json: string,
    opts?: DeserializeOptions
): Result<Address, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectAddress(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectAddress(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Address, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeAddress(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Address.fromObject: root cannot be a forward reference'
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
export function __deserializeAddress(value: any, ctx: DeserializeContext): Address | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Address.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('street' in obj)) {
        errors.push({ field: 'street', message: 'missing required field' });
    }
    if (!('city' in obj)) {
        errors.push({ field: 'city', message: 'missing required field' });
    }
    if (!('state' in obj)) {
        errors.push({ field: 'state', message: 'missing required field' });
    }
    if (!('zipcode' in obj)) {
        errors.push({ field: 'zipcode', message: 'missing required field' });
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
        const __raw_street = obj['street'] as string;
        if (__raw_street.length === 0) {
            errors.push({ field: 'street', message: 'must not be empty' });
        }
        instance.street = __raw_street;
    }
    {
        const __raw_city = obj['city'] as string;
        if (__raw_city.length === 0) {
            errors.push({ field: 'city', message: 'must not be empty' });
        }
        instance.city = __raw_city;
    }
    {
        const __raw_state = obj['state'] as string;
        if (__raw_state.length === 0) {
            errors.push({ field: 'state', message: 'must not be empty' });
        }
        instance.state = __raw_state;
    }
    {
        const __raw_zipcode = obj['zipcode'] as string;
        if (__raw_zipcode.length === 0) {
            errors.push({ field: 'zipcode', message: 'must not be empty' });
        }
        instance.zipcode = __raw_zipcode;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Address;
}
export function validateFieldAddress<K extends keyof Address>(
    field: K,
    value: Address[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'street': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'street', message: 'must not be empty' });
            }
            break;
        }
        case 'city': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'city', message: 'must not be empty' });
            }
            break;
        }
        case 'state': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'state', message: 'must not be empty' });
            }
            break;
        }
        case 'zipcode': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'zipcode', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsAddress(
    partial: Partial<Address>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('street' in partial && partial.street !== undefined) {
        const __val = partial.street as string;
        if (__val.length === 0) {
            errors.push({ field: 'street', message: 'must not be empty' });
        }
    }
    if ('city' in partial && partial.city !== undefined) {
        const __val = partial.city as string;
        if (__val.length === 0) {
            errors.push({ field: 'city', message: 'must not be empty' });
        }
    }
    if ('state' in partial && partial.state !== undefined) {
        const __val = partial.state as string;
        if (__val.length === 0) {
            errors.push({ field: 'state', message: 'must not be empty' });
        }
    }
    if ('zipcode' in partial && partial.zipcode !== undefined) {
        const __val = partial.zipcode as string;
        if (__val.length === 0) {
            errors.push({ field: 'zipcode', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapeAddress(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'street' in o && 'city' in o && 'state' in o && 'zipcode' in o;
}
export function isAddress(obj: unknown): obj is Address {
    if (!hasShapeAddress(obj)) {
        return false;
    }
    const result = fromObjectAddress(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsAddress = {
    _errors: Option<Array<string>>;
    street: Option<Array<string>>;
    city: Option<Array<string>>;
    state: Option<Array<string>>;
    zipcode: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedAddress = {
    street: Option<boolean>;
    city: Option<boolean>;
    state: Option<boolean>;
    zipcode: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersAddress {
    readonly street: FieldController<string>;
    readonly city: FieldController<string>;
    readonly state: FieldController<string>;
    readonly zipcode: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformAddress {
    readonly data: Address;
    readonly errors: ErrorsAddress;
    readonly tainted: TaintedAddress;
    readonly fields: FieldControllersAddress;
    validate(): Result<Address, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Address>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormAddress(overrides?: Partial<Address>): GigaformAddress {
    let data = $state({ ...defaultValueAddress(), ...overrides });
    let errors = $state<ErrorsAddress>({
        _errors: Option.none(),
        street: Option.none(),
        city: Option.none(),
        state: Option.none(),
        zipcode: Option.none()
    });
    let tainted = $state<TaintedAddress>({
        street: Option.none(),
        city: Option.none(),
        state: Option.none(),
        zipcode: Option.none()
    });
    const fields: FieldControllersAddress = {
        street: {
            path: ['street'] as const,
            name: 'street',
            constraints: { required: true },

            get: () => data.street,
            set: (value: string) => {
                data.street = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.street,
            setError: (value: Option<Array<string>>) => {
                errors.street = value;
            },
            getTainted: () => tainted.street,
            setTainted: (value: Option<boolean>) => {
                tainted.street = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAddress('street', data.street);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        city: {
            path: ['city'] as const,
            name: 'city',
            constraints: { required: true },

            get: () => data.city,
            set: (value: string) => {
                data.city = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.city,
            setError: (value: Option<Array<string>>) => {
                errors.city = value;
            },
            getTainted: () => tainted.city,
            setTainted: (value: Option<boolean>) => {
                tainted.city = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAddress('city', data.city);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        state: {
            path: ['state'] as const,
            name: 'state',
            constraints: { required: true },

            get: () => data.state,
            set: (value: string) => {
                data.state = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.state,
            setError: (value: Option<Array<string>>) => {
                errors.state = value;
            },
            getTainted: () => tainted.state,
            setTainted: (value: Option<boolean>) => {
                tainted.state = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAddress('state', data.state);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        zipcode: {
            path: ['zipcode'] as const,
            name: 'zipcode',
            constraints: { required: true },

            get: () => data.zipcode,
            set: (value: string) => {
                data.zipcode = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.zipcode,
            setError: (value: Option<Array<string>>) => {
                errors.zipcode = value;
            },
            getTainted: () => tainted.zipcode,
            setTainted: (value: Option<boolean>) => {
                tainted.zipcode = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldAddress('zipcode', data.zipcode);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Address, Array<{ field: string; message: string }>> {
        return fromObjectAddress(data);
    }
    function reset(newOverrides?: Partial<Address>): void {
        data = { ...defaultValueAddress(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            street: Option.none(),
            city: Option.none(),
            state: Option.none(),
            zipcode: Option.none()
        };
        tainted = {
            street: Option.none(),
            city: Option.none(),
            state: Option.none(),
            zipcode: Option.none()
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
export function fromFormDataAddress(
    formData: FormData
): Result<Address, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.street = formData.get('street') ?? '';
    obj.city = formData.get('city') ?? '';
    obj.state = formData.get('state') ?? '';
    obj.zipcode = formData.get('zipcode') ?? '';
    return fromStringifiedJSONAddress(JSON.stringify(obj));
}
