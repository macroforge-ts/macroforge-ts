import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from '@playground/macro/gigaform';
import { Option } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Commissions {
    technician: string;

    salesRep: string;
}

export function commissionsDefaultValue(): Commissions {
    return { technician: '', salesRep: '' } as Commissions;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function commissionsSerialize(
    value: Commissions
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(commissionsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function commissionsSerializeWithContext(
    value: Commissions,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Commissions', __id };
    result['technician'] = value.technician;
    result['salesRep'] = value.salesRep;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function commissionsDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Commissions> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = commissionsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Commissions.deserialize: root cannot be a forward reference'
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
export function commissionsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Commissions | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Commissions.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('technician' in obj)) {
        errors.push({ field: 'technician', message: 'missing required field' });
    }
    if (!('salesRep' in obj)) {
        errors.push({ field: 'salesRep', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_technician = obj['technician'] as string;
        if (__raw_technician.length === 0) {
            errors.push({ field: 'technician', message: 'must not be empty' });
        }
        instance.technician = __raw_technician;
    }
    {
        const __raw_salesRep = obj['salesRep'] as string;
        if (__raw_salesRep.length === 0) {
            errors.push({ field: 'salesRep', message: 'must not be empty' });
        }
        instance.salesRep = __raw_salesRep;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Commissions;
}
export function commissionsValidateField<K extends keyof Commissions>(
    field: K,
    value: Commissions[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'technician': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'technician', message: 'must not be empty' });
            }
            break;
        }
        case 'salesRep': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'salesRep', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function commissionsValidateFields(
    partial: Partial<Commissions>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('technician' in partial && partial.technician !== undefined) {
        const __val = partial.technician as string;
        if (__val.length === 0) {
            errors.push({ field: 'technician', message: 'must not be empty' });
        }
    }
    if ('salesRep' in partial && partial.salesRep !== undefined) {
        const __val = partial.salesRep as string;
        if (__val.length === 0) {
            errors.push({ field: 'salesRep', message: 'must not be empty' });
        }
    }
    return errors;
}
export function commissionsHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'technician' in o && 'salesRep' in o;
}
export function commissionsIs(obj: unknown): obj is Commissions {
    if (!commissionsHasShape(obj)) {
        return false;
    }
    const result = commissionsDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type CommissionsErrors = {
    _errors: Option<Array<string>>;
    technician: Option<Array<string>>;
    salesRep: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type CommissionsTainted = {
    technician: Option<boolean>;
    salesRep: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface CommissionsFieldControllers {
    readonly technician: FieldController<string>;
    readonly salesRep: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface CommissionsGigaform {
    readonly data: Commissions;
    readonly errors: CommissionsErrors;
    readonly tainted: CommissionsTainted;
    readonly fields: CommissionsFieldControllers;
    validate(): Result<Commissions, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Commissions>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function commissionsCreateForm(overrides?: Partial<Commissions>): CommissionsGigaform {
    let data = $state({ ...commissionsDefaultValue(), ...overrides });
    let errors = $state<CommissionsErrors>({
        _errors: Option.none(),
        technician: Option.none(),
        salesRep: Option.none()
    });
    let tainted = $state<CommissionsTainted>({
        technician: Option.none(),
        salesRep: Option.none()
    });
    const fields: CommissionsFieldControllers = {
        technician: {
            path: ['technician'] as const,
            name: 'technician',
            constraints: { required: true },
            get: () => data.technician,
            set: (value: string) => {
                data.technician = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.technician,
            setError: (value: Option<Array<string>>) => {
                errors.technician = value;
            },
            getTainted: () => tainted.technician,
            setTainted: (value: Option<boolean>) => {
                tainted.technician = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = commissionsValidateField('technician', data.technician);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        salesRep: {
            path: ['salesRep'] as const,
            name: 'salesRep',
            constraints: { required: true },
            get: () => data.salesRep,
            set: (value: string) => {
                data.salesRep = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.salesRep,
            setError: (value: Option<Array<string>>) => {
                errors.salesRep = value;
            },
            getTainted: () => tainted.salesRep,
            setTainted: (value: Option<boolean>) => {
                tainted.salesRep = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = commissionsValidateField('salesRep', data.salesRep);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Commissions, Array<{ field: string; message: string }>> {
        return commissionsDeserialize(data);
    }
    function reset(newOverrides?: Partial<Commissions>): void {
        data = { ...commissionsDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), technician: Option.none(), salesRep: Option.none() };
        tainted = { technician: Option.none(), salesRep: Option.none() };
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
export function commissionsFromFormData(
    formData: FormData
): Result<Commissions, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.technician = formData.get('technician') ?? '';
    obj.salesRep = formData.get('salesRep') ?? '';
    return commissionsDeserialize(obj);
}

export const Commissions = {
    defaultValue: commissionsDefaultValue,
    serialize: commissionsSerialize,
    serializeWithContext: commissionsSerializeWithContext,
    deserialize: commissionsDeserialize,
    deserializeWithContext: commissionsDeserializeWithContext,
    validateFields: commissionsValidateFields,
    hasShape: commissionsHasShape,
    is: commissionsIs,
    createForm: commissionsCreateForm,
    fromFormData: commissionsFromFormData
} as const;
