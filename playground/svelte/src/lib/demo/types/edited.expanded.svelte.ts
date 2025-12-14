import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Edited {
    fieldName: string;
    oldValue: string | null;
    newValue: string | null;
}

export function defaultValueEdited(): Edited {
    return { fieldName: '', oldValue: null, newValue: null } as Edited;
}

export function toStringifiedJSONEdited(value: Edited): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeEdited(value, ctx));
}
export function toObjectEdited(value: Edited): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeEdited(value, ctx);
}
export function __serializeEdited(value: Edited, ctx: SerializeContext): Record<string, unknown> {
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

export function fromStringifiedJSONEdited(
    json: string,
    opts?: DeserializeOptions
): Result<Edited, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectEdited(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectEdited(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Edited, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeEdited(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Edited.fromObject: root cannot be a forward reference' }
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
export function __deserializeEdited(value: any, ctx: DeserializeContext): Edited | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Edited.__deserialize: expected an object' }
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
export function validateFieldEdited<K extends keyof Edited>(
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
export function validateFieldsEdited(
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
export function hasShapeEdited(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'fieldName' in o && 'oldValue' in o && 'newValue' in o;
}
export function isEdited(obj: unknown): obj is Edited {
    if (!hasShapeEdited(obj)) {
        return false;
    }
    const result = fromObjectEdited(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsEdited = {
    _errors: Option<Array<string>>;
    fieldName: Option<Array<string>>;
    oldValue: Option<Array<string>>;
    newValue: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedEdited = {
    fieldName: Option<boolean>;
    oldValue: Option<boolean>;
    newValue: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersEdited {
    readonly fieldName: FieldController<string>;
    readonly oldValue: FieldController<string | null>;
    readonly newValue: FieldController<string | null>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformEdited {
    readonly data: Edited;
    readonly errors: ErrorsEdited;
    readonly tainted: TaintedEdited;
    readonly fields: FieldControllersEdited;
    validate(): Result<Edited, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Edited>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormEdited(overrides?: Partial<Edited>): GigaformEdited {
    let data = $state({ ...Edited.defaultValue(), ...overrides });
    let errors = $state<ErrorsEdited>({
        _errors: Option.none(),
        fieldName: Option.none(),
        oldValue: Option.none(),
        newValue: Option.none()
    });
    let tainted = $state<TaintedEdited>({
        fieldName: Option.none(),
        oldValue: Option.none(),
        newValue: Option.none()
    });
    const fields: FieldControllersEdited = {
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
                const fieldErrors = Edited.validateField('fieldName', data.fieldName);
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
                const fieldErrors = Edited.validateField('oldValue', data.oldValue);
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
                const fieldErrors = Edited.validateField('newValue', data.newValue);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<Edited, Array<{ field: string; message: string }>> {
        return Edited.fromObject(data);
    }
    function reset(newOverrides?: Partial<Edited>): void {
        data = { ...Edited.defaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            fieldName: Option.none(),
            oldValue: Option.none(),
            newValue: Option.none()
        };
        tainted = { fieldName: Option.none(), oldValue: Option.none(), newValue: Option.none() };
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
export function fromFormDataEdited(
    formData: FormData
): Result<Edited, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.fieldName = formData.get('fieldName') ?? '';
    obj.oldValue = formData.get('oldValue') ?? '';
    obj.newValue = formData.get('newValue') ?? '';
    return Edited.fromStringifiedJSON(JSON.stringify(obj));
}
