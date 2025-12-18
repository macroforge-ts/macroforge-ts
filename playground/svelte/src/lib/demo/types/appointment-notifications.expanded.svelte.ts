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

export interface AppointmentNotifications {
    personalScheduleChangeNotifications: string;

    allScheduleChangeNotifications: string;
}

export function appointmentNotificationsDefaultValue(): AppointmentNotifications {
    return {
        personalScheduleChangeNotifications: '',
        allScheduleChangeNotifications: ''
    } as AppointmentNotifications;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function appointmentNotificationsSerialize(
    value: AppointmentNotifications
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(appointmentNotificationsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function appointmentNotificationsSerializeWithContext(
    value: AppointmentNotifications,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'AppointmentNotifications', __id };
    result['personalScheduleChangeNotifications'] = value.personalScheduleChangeNotifications;
    result['allScheduleChangeNotifications'] = value.allScheduleChangeNotifications;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function appointmentNotificationsDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, AppointmentNotifications> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appointmentNotificationsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message:
                        'AppointmentNotifications.deserialize: root cannot be a forward reference'
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
export function appointmentNotificationsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): AppointmentNotifications | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'AppointmentNotifications.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('personalScheduleChangeNotifications' in obj)) {
        errors.push({
            field: 'personalScheduleChangeNotifications',
            message: 'missing required field'
        });
    }
    if (!('allScheduleChangeNotifications' in obj)) {
        errors.push({ field: 'allScheduleChangeNotifications', message: 'missing required field' });
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
        const __raw_personalScheduleChangeNotifications = obj[
            'personalScheduleChangeNotifications'
        ] as string;
        if (__raw_personalScheduleChangeNotifications.length === 0) {
            errors.push({
                field: 'personalScheduleChangeNotifications',
                message: 'must not be empty'
            });
        }
        instance.personalScheduleChangeNotifications = __raw_personalScheduleChangeNotifications;
    }
    {
        const __raw_allScheduleChangeNotifications = obj[
            'allScheduleChangeNotifications'
        ] as string;
        if (__raw_allScheduleChangeNotifications.length === 0) {
            errors.push({ field: 'allScheduleChangeNotifications', message: 'must not be empty' });
        }
        instance.allScheduleChangeNotifications = __raw_allScheduleChangeNotifications;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as AppointmentNotifications;
}
export function appointmentNotificationsValidateField<K extends keyof AppointmentNotifications>(
    field: K,
    value: AppointmentNotifications[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (field) {
        case 'personalScheduleChangeNotifications': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'personalScheduleChangeNotifications',
                    message: 'must not be empty'
                });
            }
            break;
        }
        case 'allScheduleChangeNotifications': {
            const __val = value as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'allScheduleChangeNotifications',
                    message: 'must not be empty'
                });
            }
            break;
        }
    }
    return errors;
}
export function appointmentNotificationsValidateFields(
    partial: Partial<AppointmentNotifications>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if (
        'personalScheduleChangeNotifications' in partial &&
        partial.personalScheduleChangeNotifications !== undefined
    ) {
        const __val = partial.personalScheduleChangeNotifications as string;
        if (__val.length === 0) {
            errors.push({
                field: 'personalScheduleChangeNotifications',
                message: 'must not be empty'
            });
        }
    }
    if (
        'allScheduleChangeNotifications' in partial &&
        partial.allScheduleChangeNotifications !== undefined
    ) {
        const __val = partial.allScheduleChangeNotifications as string;
        if (__val.length === 0) {
            errors.push({ field: 'allScheduleChangeNotifications', message: 'must not be empty' });
        }
    }
    return errors;
}
export function appointmentNotificationsHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'personalScheduleChangeNotifications' in o && 'allScheduleChangeNotifications' in o;
}
export function appointmentNotificationsIs(obj: unknown): obj is AppointmentNotifications {
    if (!appointmentNotificationsHasShape(obj)) {
        return false;
    }
    const result = appointmentNotificationsDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type AppointmentNotificationsErrors = {
    _errors: Option<Array<string>>;
    personalScheduleChangeNotifications: Option<Array<string>>;
    allScheduleChangeNotifications: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type AppointmentNotificationsTainted = {
    personalScheduleChangeNotifications: Option<boolean>;
    allScheduleChangeNotifications: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface AppointmentNotificationsFieldControllers {
    readonly personalScheduleChangeNotifications: FieldController<string>;
    readonly allScheduleChangeNotifications: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface AppointmentNotificationsGigaform {
    readonly data: AppointmentNotifications;
    readonly errors: AppointmentNotificationsErrors;
    readonly tainted: AppointmentNotificationsTainted;
    readonly fields: AppointmentNotificationsFieldControllers;
    validate(): Result<AppointmentNotifications, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<AppointmentNotifications>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function appointmentNotificationsCreateForm(
    overrides?: Partial<AppointmentNotifications>
): AppointmentNotificationsGigaform {
    let data = $state({ ...appointmentNotificationsDefaultValue(), ...overrides });
    let errors = $state<AppointmentNotificationsErrors>({
        _errors: Option.none(),
        personalScheduleChangeNotifications: Option.none(),
        allScheduleChangeNotifications: Option.none()
    });
    let tainted = $state<AppointmentNotificationsTainted>({
        personalScheduleChangeNotifications: Option.none(),
        allScheduleChangeNotifications: Option.none()
    });
    const fields: AppointmentNotificationsFieldControllers = {
        personalScheduleChangeNotifications: {
            path: ['personalScheduleChangeNotifications'] as const,
            name: 'personalScheduleChangeNotifications',
            constraints: { required: true },
            get: () => data.personalScheduleChangeNotifications,
            set: (value: string) => {
                data.personalScheduleChangeNotifications = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.personalScheduleChangeNotifications,
            setError: (value: Option<Array<string>>) => {
                errors.personalScheduleChangeNotifications = value;
            },
            getTainted: () => tainted.personalScheduleChangeNotifications,
            setTainted: (value: Option<boolean>) => {
                tainted.personalScheduleChangeNotifications = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = appointmentNotificationsValidateField(
                    'personalScheduleChangeNotifications',
                    data.personalScheduleChangeNotifications
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        allScheduleChangeNotifications: {
            path: ['allScheduleChangeNotifications'] as const,
            name: 'allScheduleChangeNotifications',
            constraints: { required: true },
            get: () => data.allScheduleChangeNotifications,
            set: (value: string) => {
                data.allScheduleChangeNotifications = value;
            },
            transform: (value: string): string => value,
            getError: () => errors.allScheduleChangeNotifications,
            setError: (value: Option<Array<string>>) => {
                errors.allScheduleChangeNotifications = value;
            },
            getTainted: () => tainted.allScheduleChangeNotifications,
            setTainted: (value: Option<boolean>) => {
                tainted.allScheduleChangeNotifications = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = appointmentNotificationsValidateField(
                    'allScheduleChangeNotifications',
                    data.allScheduleChangeNotifications
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<
        AppointmentNotifications,
        Array<{ field: string; message: string }>
    > {
        return appointmentNotificationsDeserialize(data);
    }
    function reset(newOverrides?: Partial<AppointmentNotifications>): void {
        data = { ...appointmentNotificationsDefaultValue(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            personalScheduleChangeNotifications: Option.none(),
            allScheduleChangeNotifications: Option.none()
        };
        tainted = {
            personalScheduleChangeNotifications: Option.none(),
            allScheduleChangeNotifications: Option.none()
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
export function appointmentNotificationsFromFormData(
    formData: FormData
): Result<AppointmentNotifications, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.personalScheduleChangeNotifications =
        formData.get('personalScheduleChangeNotifications') ?? '';
    obj.allScheduleChangeNotifications = formData.get('allScheduleChangeNotifications') ?? '';
    return appointmentNotificationsDeserialize(obj);
}

export const AppointmentNotifications = {
    defaultValue: appointmentNotificationsDefaultValue,
    serialize: appointmentNotificationsSerialize,
    serializeWithContext: appointmentNotificationsSerializeWithContext,
    deserialize: appointmentNotificationsDeserialize,
    deserializeWithContext: appointmentNotificationsDeserializeWithContext,
    validateFields: appointmentNotificationsValidateFields,
    hasShape: appointmentNotificationsHasShape,
    is: appointmentNotificationsIs,
    createForm: appointmentNotificationsCreateForm,
    fromFormData: appointmentNotificationsFromFormData
} as const;
