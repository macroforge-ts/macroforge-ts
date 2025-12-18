import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Commented {
    comment: string;
    replyTo: string | null;
}

export function commentedDefaultValue(): Commented {
    return { comment: '', replyTo: null } as Commented;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function commentedSerialize(
    value: Commented
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(commentedSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function commentedSerializeWithContext(
    value: Commented,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Commented', __id };
    result['comment'] = value.comment;
    result['replyTo'] = value.replyTo;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function commentedDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Commented> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = commentedDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Commented.deserialize: root cannot be a forward reference'
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
export function commentedDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Commented | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Commented.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('comment' in obj)) {
        errors.push({ field: 'comment', message: 'missing required field' });
    }
    if (!('replyTo' in obj)) {
        errors.push({ field: 'replyTo', message: 'missing required field' });
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
        const __raw_comment = obj['comment'] as string;
        if (__raw_comment.length === 0) {
            errors.push({ field: 'comment', message: 'must not be empty' });
        }
        instance.comment = __raw_comment;
    }
    {
        const __raw_replyTo = obj['replyTo'] as string | null;
        instance.replyTo = __raw_replyTo;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Commented;
}
export function commentedValidateField<K extends keyof Commented>(
    field: K,
    value: Commented[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'comment': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'comment', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function commentedValidateFields(
    partial: Partial<Commented>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('comment' in partial && partial.comment !== undefined) {
        const __val = partial.comment as string;
        if (__val.length === 0) {
            errors.push({ field: 'comment', message: 'must not be empty' });
        }
    }
    return errors;
}
export function commentedHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'comment' in o && 'replyTo' in o;
}
export function commentedIs(obj: unknown): obj is Commented {
    if (!commentedHasShape(obj)) {
        return false;
    }
    const result = commentedDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type CommentedErrors = {
    _errors: Option<Array<string>>;
    comment: Option<Array<string>>;
    replyTo: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type CommentedTainted = {
    comment: Option<boolean>;
    replyTo: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface CommentedFieldControllers {
    readonly comment: FieldController<string>;
    readonly replyTo: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface CommentedGigaform {
    readonly data: Commented;
    readonly errors: CommentedErrors;
    readonly tainted: CommentedTainted;
    readonly fields: CommentedFieldControllers;
    validate(): Result<Commented, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Commented>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function commentedCreateForm(overrides?: Partial<Commented>): CommentedGigaform {
    let data = $state({ ...commentedDefaultValue(), ...overrides });
    let errors = $state<CommentedErrors>({
        _errors: Option.none(),
        comment: Option.none(),
        replyTo: Option.none()
    });
    let tainted = $state<CommentedTainted>({ comment: Option.none(), replyTo: Option.none() });
    const fields: CommentedFieldControllers = {
        comment: {
            path: ['comment'] as const,
            name: 'comment',
            constraints: { required: true },
            get: () => data.comment,
            set: (value: string) => {
                data.comment = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.comment,
            setError: (value: Option<Array<string>>) => {
                errors.comment = value;
            },
            getTainted: () => tainted.comment,
            setTainted: (value: Option<boolean>) => {
                tainted.comment = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = commentedValidateField('comment', data.comment);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        replyTo: {
            path: ['replyTo'] as const,
            name: 'replyTo',
            constraints: { required: true },
            get: () => data.replyTo,
            set: (value: string | null) => {
                data.replyTo = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.replyTo,
            setError: (value: Option<Array<string>>) => {
                errors.replyTo = value;
            },
            getTainted: () => tainted.replyTo,
            setTainted: (value: Option<boolean>) => {
                tainted.replyTo = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = commentedValidateField('replyTo', data.replyTo);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Commented, Array<{ field: string; message: string }>> {
        return commentedDeserialize(data);
    }
    function reset(newOverrides?: Partial<Commented>): void {
        data = { ...commentedDefaultValue(), ...newOverrides };
        errors = { _errors: Option.none(), comment: Option.none(), replyTo: Option.none() };
        tainted = { comment: Option.none(), replyTo: Option.none() };
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
export function commentedFromFormData(
    formData: FormData
): Result<Commented, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.comment = formData.get('comment') ?? '';
    obj.replyTo = formData.get('replyTo') ?? '';
    return commentedDeserialize(obj);
}

export const Commented = {
    defaultValue: commentedDefaultValue,
    serialize: commentedSerialize,
    serializeWithContext: commentedSerializeWithContext,
    deserialize: commentedDeserialize,
    deserializeWithContext: commentedDeserializeWithContext,
    validateFields: commentedValidateFields,
    hasShape: commentedHasShape,
    is: commentedIs,
    createForm: commentedCreateForm,
    fromFormData: commentedFromFormData
} as const;
