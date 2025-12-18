import { SerializeContext } from 'macroforge/serde';
import { Exit } from 'macroforge/utils/effect';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Package {
    id: string;

    date: string;
}

export function packageDefaultValue(): Package {
    return { id: '', date: '' } as Package;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function packageSerialize(
    value: Package
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(packageSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function packageSerializeWithContext(
    value: Package,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Package', __id };
    result['id'] = value.id;
    result['date'] = value.date;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function packageDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, Package> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = packageDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                {
                    field: '_root',
                    message: 'Package.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Exit.succeed(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Exit.fail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Exit.fail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function packageDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Package | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Package.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('date' in obj)) {
        errors.push({ field: 'date', message: 'missing required field' });
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
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_date = obj['date'] as string;
        instance.date = __raw_date;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Package;
}
export function packageValidateField<K extends keyof Package>(
    field: K,
    value: Package[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function packageValidateFields(
    partial: Partial<Package>
): Array<{ field: string; message: string }> {
    return [];
}
export function packageHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'date' in o;
}
export function packageIs(obj: unknown): obj is Package {
    if (!packageHasShape(obj)) {
        return false;
    }
    const result = packageDeserialize(obj);
    return Exit.isSuccess(result);
}

/** Nested error structure matching the data shape */ export type PackageErrors = {
    _errors: Option<Array<string>>;
    id: Option<Array<string>>;
    date: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type PackageTainted = {
    id: Option<boolean>;
    date: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface PackageFieldControllers {
    readonly id: FieldController<string>;
    readonly date: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface PackageGigaform {
    readonly data: Package;
    readonly errors: PackageErrors;
    readonly tainted: PackageTainted;
    readonly fields: PackageFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Package>;
    reset(overrides?: Partial<Package>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function packageCreateForm(overrides?: Partial<Package>): PackageGigaform {
    let data = $state({ ...packageDefaultValue(), ...overrides });
    let errors = $state<PackageErrors>({
        _errors: optionNone(),
        id: optionNone(),
        date: optionNone()
    });
    let tainted = $state<PackageTainted>({ id: optionNone(), date: optionNone() });
    const fields: PackageFieldControllers = {
        id: {
            path: ['id'] as const,
            name: 'id',
            constraints: { required: true },
            get: () => data.id,
            set: (value: string) => {
                data.id = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.id,
            setError: (value: Option<Array<string>>) => {
                errors.id = value;
            },
            getTainted: () => tainted.id,
            setTainted: (value: Option<boolean>) => {
                tainted.id = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = packageValidateField('id', data.id);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        date: {
            path: ['date'] as const,
            name: 'date',
            constraints: { required: true },
            label: 'Date',
            get: () => data.date,
            set: (value: string) => {
                data.date = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.date,
            setError: (value: Option<Array<string>>) => {
                errors.date = value;
            },
            getTainted: () => tainted.date,
            setTainted: (value: Option<boolean>) => {
                tainted.date = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = packageValidateField('date', data.date);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Package> {
        return toExit(packageDeserialize(data));
    }
    function reset(newOverrides?: Partial<Package>): void {
        data = { ...packageDefaultValue(), ...newOverrides };
        errors = { _errors: optionNone(), id: optionNone(), date: optionNone() };
        tainted = { id: optionNone(), date: optionNone() };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function packageFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Package> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get('id') ?? '';
    obj.date = formData.get('date') ?? '';
    return toExit(packageDeserialize(obj));
}

export const Package = {
    defaultValue: packageDefaultValue,
    serialize: packageSerialize,
    serializeWithContext: packageSerializeWithContext,
    deserialize: packageDeserialize,
    deserializeWithContext: packageDeserializeWithContext,
    validateFields: packageValidateFields,
    hasShape: packageHasShape,
    is: packageIs,
    createForm: packageCreateForm,
    fromFormData: packageFromFormData
} as const;
