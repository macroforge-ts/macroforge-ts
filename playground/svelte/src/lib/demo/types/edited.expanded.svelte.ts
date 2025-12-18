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

export interface Edited {
    fieldName: string;
    oldValue: string | null;
    newValue: string | null;
}

export function editedDefaultValue(): Edited {
    return { fieldName: '', oldValue: null, newValue: null } as Edited;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function editedSerialize(
    value: Edited
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(editedSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function editedSerializeWithContext(
    value: Edited,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Edited', __id };
    result['fieldName'] = value.fieldName;
    result['oldValue'] = value.oldValue;
    result['newValue'] = value.newValue;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function editedDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Exit.Exit<Array<{ field: string; message: string }>, Edited> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = editedDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Exit.fail([
                {
                    field: '_root',
                    message: 'Edited.deserialize: root cannot be a forward reference'
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
export function editedDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Edited | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Edited.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('fieldName' in obj)) {
        errors.push({ field: 'fieldName', message: 'missing required field' });
    }
    if (!('oldValue' in obj)) {
        errors.push({ field: 'oldValue', message: 'missing required field' });
    }
    if (!('newValue' in obj)) {
        errors.push({ field: 'newValue', message: 'missing required field' });
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
        const __raw_fieldName = obj['fieldName'] as string;
        if (__raw_fieldName.length === 0) {
            errors.push({ field: 'fieldName', message: 'must not be empty' });
        }
        instance.fieldName = __raw_fieldName;
    }
    {
        const __raw_oldValue = obj['oldValue'] as string | null;
        instance.oldValue = __raw_oldValue;
    }
    {
        const __raw_newValue = obj['newValue'] as string | null;
        instance.newValue = __raw_newValue;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Edited;
}
export function editedValidateField<K extends keyof Edited>(
    field: K,
    value: Edited[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'fieldName': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'fieldName', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function editedValidateFields(
    partial: Partial<Edited>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('fieldName' in partial && partial.fieldName !== undefined) {
        const __val = partial.fieldName as string;
        if (__val.length === 0) {
            errors.push({ field: 'fieldName', message: 'must not be empty' });
        }
    }
    return errors;
}
export function editedHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'fieldName' in o && 'oldValue' in o && 'newValue' in o;
}
export function editedIs(obj: unknown): obj is Edited {
    if (!editedHasShape(obj)) {
        return false;
    }
    const result = editedDeserialize(obj);
    return Exit.isSuccess(result);
}

/** Nested error structure matching the data shape */ export type EditedErrors = {
    _errors: Option<Array<string>>;
    fieldName: Option<Array<string>>;
    oldValue: Option<Array<string>>;
    newValue: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type EditedTainted = {
    fieldName: Option<boolean>;
    oldValue: Option<boolean>;
    newValue: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface EditedFieldControllers {
    readonly fieldName: FieldController<string>;
    readonly oldValue: FieldController<string | null>;
    readonly newValue: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface EditedGigaform {
    readonly data: Edited;
    readonly errors: EditedErrors;
    readonly tainted: EditedTainted;
    readonly fields: EditedFieldControllers;
    validate(): Exit<Array<{ field: string; message: string }>, Edited>;
    reset(overrides?: Partial<Edited>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function editedCreateForm(overrides?: Partial<Edited>): EditedGigaform {
    let data = $state({ ...editedDefaultValue(), ...overrides });
    let errors = $state<EditedErrors>({
        _errors: optionNone(),
        fieldName: optionNone(),
        oldValue: optionNone(),
        newValue: optionNone()
    });
    let tainted = $state<EditedTainted>({
        fieldName: optionNone(),
        oldValue: optionNone(),
        newValue: optionNone()
    });
    const fields: EditedFieldControllers = {
        fieldName: {
            path: ['fieldName'] as const,
            name: 'fieldName',
            constraints: { required: true },
            get: () => data.fieldName,
            set: (value: string) => {
                data.fieldName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.fieldName,
            setError: (value: Option<Array<string>>) => {
                errors.fieldName = value;
            },
            getTainted: () => tainted.fieldName,
            setTainted: (value: Option<boolean>) => {
                tainted.fieldName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = editedValidateField('fieldName', data.fieldName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        oldValue: {
            path: ['oldValue'] as const,
            name: 'oldValue',
            constraints: { required: true },
            get: () => data.oldValue,
            set: (value: string | null) => {
                data.oldValue = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.oldValue,
            setError: (value: Option<Array<string>>) => {
                errors.oldValue = value;
            },
            getTainted: () => tainted.oldValue,
            setTainted: (value: Option<boolean>) => {
                tainted.oldValue = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = editedValidateField('oldValue', data.oldValue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        newValue: {
            path: ['newValue'] as const,
            name: 'newValue',
            constraints: { required: true },
            get: () => data.newValue,
            set: (value: string | null) => {
                data.newValue = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.newValue,
            setError: (value: Option<Array<string>>) => {
                errors.newValue = value;
            },
            getTainted: () => tainted.newValue,
            setTainted: (value: Option<boolean>) => {
                tainted.newValue = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = editedValidateField('newValue', data.newValue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<Array<{ field: string; message: string }>, Edited> {
        return toExit(editedDeserialize(data));
    }
    function reset(newOverrides?: Partial<Edited>): void {
        data = { ...editedDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            fieldName: optionNone(),
            oldValue: optionNone(),
            newValue: optionNone()
        };
        tainted = { fieldName: optionNone(), oldValue: optionNone(), newValue: optionNone() };
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
export function editedFromFormData(
    formData: FormData
): Exit<Array<{ field: string; message: string }>, Edited> {
    const obj: Record<string, unknown> = {};
    obj.fieldName = formData.get('fieldName') ?? '';
    obj.oldValue = formData.get('oldValue') ?? '';
    obj.newValue = formData.get('newValue') ?? '';
    return toExit(editedDeserialize(obj));
}

export const Edited = {
    defaultValue: editedDefaultValue,
    serialize: editedSerialize,
    serializeWithContext: editedSerializeWithContext,
    deserialize: editedDeserialize,
    deserializeWithContext: editedDeserializeWithContext,
    validateFields: editedValidateFields,
    hasShape: editedHasShape,
    is: editedIs,
    createForm: editedCreateForm,
    fromFormData: editedFromFormData
} as const;
