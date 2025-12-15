import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface PersonName {
    firstName: string;

    lastName: string;
}

export function defaultValuePersonName(): PersonName {
    return { firstName: '', lastName: '' } as PersonName;
}

export function toStringifiedJSONPersonName(value: PersonName): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializePersonName(value, ctx));
}
export function toObjectPersonName(value: PersonName): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializePersonName(value, ctx);
}
export function __serializePersonName(
    value: PersonName,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'PersonName', __id };
    result['firstName'] = value.firstName;
    result['lastName'] = value.lastName;
    return result;
}

export function fromStringifiedJSONPersonName(
    json: string,
    opts?: DeserializeOptions
): Result<PersonName, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectPersonName(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectPersonName(
    obj: unknown,
    opts?: DeserializeOptions
): Result<PersonName, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializePersonName(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'PersonName.fromObject: root cannot be a forward reference'
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
export function __deserializePersonName(
    value: any,
    ctx: DeserializeContext
): PersonName | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'PersonName.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('firstName' in obj)) {
        errors.push({ field: 'firstName', message: 'missing required field' });
    }
    if (!('lastName' in obj)) {
        errors.push({ field: 'lastName', message: 'missing required field' });
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
        const __raw_firstName = obj['firstName'] as string;
        if (__raw_firstName.length === 0) {
            errors.push({ field: 'firstName', message: 'must not be empty' });
        }
        instance.firstName = __raw_firstName;
    }
    {
        const __raw_lastName = obj['lastName'] as string;
        if (__raw_lastName.length === 0) {
            errors.push({ field: 'lastName', message: 'must not be empty' });
        }
        instance.lastName = __raw_lastName;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as PersonName;
}
export function validateFieldPersonName<K extends keyof PersonName>(
    field: K,
    value: PersonName[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'firstName': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'firstName', message: 'must not be empty' });
            }
            break;
        }
        case 'lastName': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({ field: 'lastName', message: 'must not be empty' });
            }
            break;
        }
    }
    return errors;
}
export function validateFieldsPersonName(
    partial: Partial<PersonName>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if ('firstName' in partial && partial.firstName !== undefined) {
        const __val = partial.firstName as string;
        if (__val.length === 0) {
            errors.push({ field: 'firstName', message: 'must not be empty' });
        }
    }
    if ('lastName' in partial && partial.lastName !== undefined) {
        const __val = partial.lastName as string;
        if (__val.length === 0) {
            errors.push({ field: 'lastName', message: 'must not be empty' });
        }
    }
    return errors;
}
export function hasShapePersonName(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'firstName' in o && 'lastName' in o;
}
export function isPersonName(obj: unknown): obj is PersonName {
    if (!hasShapePersonName(obj)) {
        return false;
    }
    const result = fromObjectPersonName(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsPersonName = {
    _errors: Option<Array<string>>;
    firstName: Option<Array<string>>;
    lastName: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedPersonName = {
    firstName: Option<boolean>;
    lastName: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersPersonName {
    readonly firstName: FieldController<string>;
    readonly lastName: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformPersonName {
    readonly data: PersonName;
    readonly errors: ErrorsPersonName;
    readonly tainted: TaintedPersonName;
    readonly fields: FieldControllersPersonName;
    validate(): Result<PersonName, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<PersonName>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormPersonName(overrides?: Partial<PersonName>): GigaformPersonName {
    let data = $state({ ...defaultValuePersonName(), ...overrides });
    let errors = $state<ErrorsPersonName>({
        _errors: Option.none(),
        firstName: Option.none(),
        lastName: Option.none()
    });
    let tainted = $state<TaintedPersonName>({ firstName: Option.none(), lastName: Option.none() });
    const fields: FieldControllersPersonName = {
        firstName: {
            path: ['firstName'] as const,
            name: 'firstName',
            constraints: { required: true },
            label: 'First Name',
            get: () => data.firstName,
            set: (value: string) => {
                data.firstName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.firstName,
            setError: (value: Option<Array<string>>) => {
                errors.firstName = value;
            },
            getTainted: () => tainted.firstName,
            setTainted: (value: Option<boolean>) => {
                tainted.firstName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldPersonName('firstName', data.firstName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        lastName: {
            path: ['lastName'] as const,
            name: 'lastName',
            constraints: { required: true },
            label: 'Last Name',
            get: () => data.lastName,
            set: (value: string) => {
                data.lastName = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.lastName,
            setError: (value: Option<Array<string>>) => {
                errors.lastName = value;
            },
            getTainted: () => tainted.lastName,
            setTainted: (value: Option<boolean>) => {
                tainted.lastName = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldPersonName('lastName', data.lastName);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<PersonName, Array<{ field: string; message: string }>> {
        return fromObjectPersonName(data);
    }
    function reset(newOverrides?: Partial<PersonName>): void {
        data = { ...defaultValuePersonName(), ...newOverrides };
        errors = { _errors: Option.none(), firstName: Option.none(), lastName: Option.none() };
        tainted = { firstName: Option.none(), lastName: Option.none() };
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
export function fromFormDataPersonName(
    formData: FormData
): Result<PersonName, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.firstName = formData.get('firstName') ?? '';
    obj.lastName = formData.get('lastName') ?? '';
    return fromStringifiedJSONPersonName(JSON.stringify(obj));
}
