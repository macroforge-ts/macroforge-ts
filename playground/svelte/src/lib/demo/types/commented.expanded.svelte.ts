import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Commented {
    comment: string;
    replyTo: string | null;
}

export function defaultValueCommented(): Commented {
    return { comment: '', replyTo: null } as Commented;
}

export function toStringifiedJSONCommented(value: Commented): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeCommented(value, ctx));
}
export function toObjectCommented(value: Commented): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeCommented(value, ctx);
}
export function __serializeCommented(
    value: Commented,
    ctx: SerializeContext
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

export function fromStringifiedJSONCommented(
    json: string,
    opts?: DeserializeOptions
): Result<Commented, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectCommented(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectCommented(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Commented, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeCommented(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'Commented.fromObject: root cannot be a forward reference'
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
export function __deserializeCommented(
    value: any,
    ctx: DeserializeContext
): Commented | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Commented.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
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
        throw new DeserializeError(errors);
    }
    return instance as Commented;
}
export function validateFieldCommented<K extends keyof Commented>(
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
export function validateFieldsCommented(
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
export function hasShapeCommented(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'comment' in o && 'replyTo' in o;
}
export function isCommented(obj: unknown): obj is Commented {
    if (!hasShapeCommented(obj)) {
        return false;
    }
    const result = fromObjectCommented(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsCommented = {
    _errors: Option<Array<string>>;
    comment: Option<Array<string>>;
    replyTo: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedCommented = {
    comment: Option<boolean>;
    replyTo: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersCommented {
    readonly comment: FieldController<string>;
    readonly replyTo: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformCommented {
    readonly data: Commented;
    readonly errors: ErrorsCommented;
    readonly tainted: TaintedCommented;
    readonly fields: FieldControllersCommented;
    validate(): Result<Commented, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Commented>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormCommented(overrides?: Partial<Commented>): GigaformCommented {
    let data = $state({ ...defaultValueCommented(), ...overrides });
    let errors = $state<ErrorsCommented>({
        _errors: Option.none(),
        comment: Option.none(),
        replyTo: Option.none()
    });
    let tainted = $state<TaintedCommented>({ comment: Option.none(), replyTo: Option.none() });
    const fields: FieldControllersCommented = {
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
                const fieldErrors = validateFieldCommented('comment', data.comment);
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
                const fieldErrors = validateFieldCommented('replyTo', data.replyTo);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Commented, Array<{ field: string; message: string }>> {
        return fromObjectCommented(data);
    }
    function reset(newOverrides?: Partial<Commented>): void {
        data = { ...defaultValueCommented(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataCommented(
    formData: FormData
): Result<Commented, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.comment = formData.get('comment') ?? '';
    obj.replyTo = formData.get('replyTo') ?? '';
    return fromStringifiedJSONCommented(JSON.stringify(obj));
}
