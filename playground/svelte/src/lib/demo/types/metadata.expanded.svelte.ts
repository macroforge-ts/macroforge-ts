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
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface Metadata {
    createdAt: string;
    lastLogin: string | null;
    isActive: boolean;
    roles: string[];
}

export function metadataDefaultValue(): Metadata {
    return { createdAt: '', lastLogin: null, isActive: false, roles: [] } as Metadata;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function metadataSerialize(
    value: Metadata
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(metadataSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function metadataSerializeWithContext(
    value: Metadata,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Metadata', __id };
    result['createdAt'] = value.createdAt;
    result['lastLogin'] = value.lastLogin;
    result['isActive'] = value.isActive;
    result['roles'] = value.roles;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function metadataDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, Metadata> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = metadataDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'Metadata.deserialize: root cannot be a forward reference'
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
export function metadataDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): Metadata | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            { field: '_root', message: 'Metadata.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('createdAt' in obj)) {
        errors.push({ field: 'createdAt', message: 'missing required field' });
    }
    if (!('lastLogin' in obj)) {
        errors.push({ field: 'lastLogin', message: 'missing required field' });
    }
    if (!('isActive' in obj)) {
        errors.push({ field: 'isActive', message: 'missing required field' });
    }
    if (!('roles' in obj)) {
        errors.push({ field: 'roles', message: 'missing required field' });
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
        const __raw_createdAt = obj['createdAt'] as string;
        instance.createdAt = __raw_createdAt;
    }
    {
        const __raw_lastLogin = obj['lastLogin'] as string | null;
        instance.lastLogin = __raw_lastLogin;
    }
    {
        const __raw_isActive = obj['isActive'] as boolean;
        instance.isActive = __raw_isActive;
    }
    {
        const __raw_roles = obj['roles'] as string[];
        if (Array.isArray(__raw_roles)) {
            instance.roles = __raw_roles as string[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Metadata;
}
export function metadataValidateField<K extends keyof Metadata>(
    field: K,
    value: Metadata[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function metadataValidateFields(
    partial: Partial<Metadata>
): Array<{ field: string; message: string }> {
    return [];
}
export function metadataHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'createdAt' in o && 'lastLogin' in o && 'isActive' in o && 'roles' in o;
}
export function metadataIs(obj: unknown): obj is Metadata {
    if (!metadataHasShape(obj)) {
        return false;
    }
    const result = metadataDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type MetadataErrors = {
    _errors: Option<Array<string>>;
    createdAt: Option<Array<string>>;
    lastLogin: Option<Array<string>>;
    isActive: Option<Array<string>>;
    roles: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type MetadataTainted = {
    createdAt: Option<boolean>;
    lastLogin: Option<boolean>;
    isActive: Option<boolean>;
    roles: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface MetadataFieldControllers {
    readonly createdAt: FieldController<string>;
    readonly lastLogin: FieldController<string | null>;
    readonly isActive: FieldController<boolean>;
    readonly roles: ArrayFieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface MetadataGigaform {
    readonly data: Metadata;
    readonly errors: MetadataErrors;
    readonly tainted: MetadataTainted;
    readonly fields: MetadataFieldControllers;
    validate(): Result<Metadata, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Metadata>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function metadataCreateForm(overrides?: Partial<Metadata>): MetadataGigaform {
    let data = $state({ ...metadataDefaultValue(), ...overrides });
    let errors = $state<MetadataErrors>({
        _errors: Option.none(),
        createdAt: Option.none(),
        lastLogin: Option.none(),
        isActive: Option.none(),
        roles: Option.none()
    });
    let tainted = $state<MetadataTainted>({
        createdAt: Option.none(),
        lastLogin: Option.none(),
        isActive: Option.none(),
        roles: Option.none()
    });
    const fields: MetadataFieldControllers = {
        createdAt: {
            path: ['createdAt'] as const,
            name: 'createdAt',
            constraints: { required: true },
            get: () => data.createdAt,
            set: (value: string) => {
                data.createdAt = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.createdAt,
            setError: (value: Option<Array<string>>) => {
                errors.createdAt = value;
            },
            getTainted: () => tainted.createdAt,
            setTainted: (value: Option<boolean>) => {
                tainted.createdAt = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = metadataValidateField('createdAt', data.createdAt);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        lastLogin: {
            path: ['lastLogin'] as const,
            name: 'lastLogin',
            constraints: { required: true },
            get: () => data.lastLogin,
            set: (value: string | null) => {
                data.lastLogin = value;
            },
            transform: (value: string | null): string | null => value,
            getError: () => errors.lastLogin,
            setError: (value: Option<Array<string>>) => {
                errors.lastLogin = value;
            },
            getTainted: () => tainted.lastLogin,
            setTainted: (value: Option<boolean>) => {
                tainted.lastLogin = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = metadataValidateField('lastLogin', data.lastLogin);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        isActive: {
            path: ['isActive'] as const,
            name: 'isActive',
            constraints: { required: true },
            get: () => data.isActive,
            set: (value: boolean) => {
                data.isActive = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.isActive,
            setError: (value: Option<Array<string>>) => {
                errors.isActive = value;
            },
            getTainted: () => tainted.isActive,
            setTainted: (value: Option<boolean>) => {
                tainted.isActive = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = metadataValidateField('isActive', data.isActive);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        roles: {
            path: ['roles'] as const,
            name: 'roles',
            constraints: { required: true },
            get: () => data.roles,
            set: (value: string[]) => {
                data.roles = value;
            },
            transform: (value: string[]): string[] => value,
            getError: () => errors.roles,
            setError: (value: Option<Array<string>>) => {
                errors.roles = value;
            },
            getTainted: () => tainted.roles,
            setTainted: (value: Option<boolean>) => {
                tainted.roles = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = metadataValidateField('roles', data.roles);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['roles', index] as const,
                name: `roles.${index}`,
                constraints: { required: true },
                get: () => data.roles[index]!,
                set: (value: string) => {
                    data.roles[index] = value;
                },
                transform: (value: string): string => value,
                getError: () => errors.roles,
                setError: (value: Option<Array<string>>) => {
                    errors.roles = value;
                },
                getTainted: () => tainted.roles,
                setTainted: (value: Option<boolean>) => {
                    tainted.roles = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string) => {
                data.roles.push(item);
            },
            remove: (index: number) => {
                data.roles.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.roles[a]!;
                data.roles[a] = data.roles[b]!;
                data.roles[b] = tmp;
            }
        }
    };
    function validate(): Result<Metadata, Array<{ field: string; message: string }>> {
        return metadataDeserialize(data);
    }
    function reset(newOverrides?: Partial<Metadata>): void {
        data = { ...metadataDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            createdAt: Option.none(),
            lastLogin: Option.none(),
            isActive: Option.none(),
            roles: Option.none()
        };
        tainted = {
            createdAt: Option.none(),
            lastLogin: Option.none(),
            isActive: Option.none(),
            roles: Option.none()
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function metadataFromFormData(
    formData: FormData
): Result<Metadata, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.createdAt = formData.get('createdAt') ?? '';
    obj.lastLogin = formData.get('lastLogin') ?? '';
    {
        const isActiveVal = formData.get('isActive');
        obj.isActive = isActiveVal === 'true' || isActiveVal === 'on' || isActiveVal === '1';
    }
    obj.roles = formData.getAll('roles') as Array<string>;
    return metadataDeserialize(obj);
}

export const Metadata = {
    defaultValue: metadataDefaultValue,
    serialize: metadataSerialize,
    serializeWithContext: metadataSerializeWithContext,
    deserialize: metadataDeserialize,
    deserializeWithContext: metadataDeserializeWithContext,
    validateFields: metadataValidateFields,
    hasShape: metadataHasShape,
    is: metadataIs,
    createForm: metadataCreateForm,
    fromFormData: metadataFromFormData
} as const;
