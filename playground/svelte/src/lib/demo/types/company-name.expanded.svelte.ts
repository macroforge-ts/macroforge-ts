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

import type { Company } from './company.svelte';

export interface CompanyName {
    companyName: string;
}

export function companyNameDefaultValue(): CompanyName {
    return { companyName: '' } as CompanyName;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function companyNameSerialize(
    value: CompanyName
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(companyNameSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function companyNameSerializeWithContext(
    value: CompanyName,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'CompanyName', __id };
    result['companyName'] = value.companyName;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function companyNameDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, CompanyName> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = companyNameDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'CompanyName.deserialize: root cannot be a forward reference'
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
export function companyNameDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): CompanyName | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'CompanyName.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('companyName' in obj)) {
        errors.push({ field: 'companyName', message: 'missing required field' });
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
        const __raw_companyName = obj['companyName'] as string;
        if (__raw_companyName.length === 0) {
            errors.push({ field: 'companyName', message: 'must not be empty' });
        }
        instance.companyName = __raw_companyName;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as CompanyName;
}
export function companyNameValidateField<K extends keyof CompanyName>(
    field: K,
    value: CompanyName[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'companyName': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'companyName', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function companyNameValidateFields(
    partial: Partial<CompanyName>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('companyName' in partial && partial.companyName !== undefined) {
        const __val = partial.companyName as string;
        if (__val.length === 0) {
            errors.push({ field: 'companyName', message: 'must not be empty' });
        }
    }
    return errors;
}
export function companyNameHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'companyName' in o;
}
export function companyNameIs(obj: unknown): obj is CompanyName {
    if (!companyNameHasShape(obj)) {
        return false;
    }
    const result = companyNameDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type CompanyNameErrors = {
    _errors: Option<Array<string>>;
    companyName: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type CompanyNameTainted = {
    companyName: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface CompanyNameFieldControllers {
    readonly companyName: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface CompanyNameGigaform {
    readonly data: CompanyName;
    readonly errors: CompanyNameErrors;
    readonly tainted: CompanyNameTainted;
    readonly fields: CompanyNameFieldControllers;
    validate(): Result<CompanyName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<CompanyName>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function companyNameCreateForm(overrides?: Partial<CompanyName>): CompanyNameGigaform {
    let data = $state({ ...companyNameDefaultValue(), ...overrides });
    let errors = $state<CompanyNameErrors>({ _errors: Option.none(), companyName: Option.none() });
    let tainted = $state<CompanyNameTainted>({ companyName: Option.none() });
    const fields: CompanyNameFieldControllers = {
        companyName: {
            path: ['companyName'] as const,
            name: 'companyName',
            constraints: { required: true },
            label: 'Company Name',
            get: () => data.companyName,
            set: (value: string) => {
                data.companyName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.companyName,
            setError: (value: Option<Array<string>>) => {
                errors.companyName = value;
            },
            getTainted: () => tainted.companyName,
            setTainted: (value: Option<boolean>) => {
                tainted.companyName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = companyNameValidateField('companyName', data.companyName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<CompanyName, Array<{ field: string; message: string }>> {
        return companyNameDeserialize(data);
    }
    function reset(newOverrides?: Partial<CompanyName>): void {
        data = { ...companyNameDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), companyName: Option.none() };
        tainted = { companyName: Option.none() };
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
export function companyNameFromFormData(
    formData: FormData
): Result<CompanyName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.companyName = formData.get('companyName') ?? '';
    return companyNameDeserialize(obj);
}

export const CompanyName = {
    defaultValue: companyNameDefaultValue,
    serialize: companyNameSerialize,
    serializeWithContext: companyNameSerializeWithContext,
    deserialize: companyNameDeserialize,
    deserializeWithContext: companyNameDeserializeWithContext,
    validateFields: companyNameValidateFields,
    hasShape: companyNameHasShape,
    is: companyNameIs,
    createForm: companyNameCreateForm,
    fromFormData: companyNameFromFormData
} as const;
