import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
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
):
    | { success: true; value: AppointmentNotifications }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appointmentNotificationsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message:
                            'AppointmentNotifications.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
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
    _field: K,
    _value: AppointmentNotifications[K]
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    switch (_field) {
        case 'personalScheduleChangeNotifications': {
            const __val = _value as string;
            if (__val.length === 0) {
                errors.push({
                    field: 'personalScheduleChangeNotifications',
                    message: 'must not be empty'
                });
            }
            break;
        }
        case 'allScheduleChangeNotifications': {
            const __val = _value as string;
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
    _partial: Partial<AppointmentNotifications>
): Array<{ field: string; message: string }> {
    const errors: Array<{ field: string; message: string }> = [];
    if (
        'personalScheduleChangeNotifications' in _partial &&
        _partial.personalScheduleChangeNotifications !== undefined
    ) {
        const __val = _partial.personalScheduleChangeNotifications as string;
        if (__val.length === 0) {
            errors.push({
                field: 'personalScheduleChangeNotifications',
                message: 'must not be empty'
            });
        }
    }
    if (
        'allScheduleChangeNotifications' in _partial &&
        _partial.allScheduleChangeNotifications !== undefined
    ) {
        const __val = _partial.allScheduleChangeNotifications as string;
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
    return result.success;
}

/** Nested error structure matching the data shape */ export type AppointmentNotificationsErrors = {
    _errors: __gf_Option<Array<string>>;
    personalScheduleChangeNotifications: __gf_Option<Array<string>>;
    allScheduleChangeNotifications: __gf_Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type AppointmentNotificationsTainted = {
    personalScheduleChangeNotifications: __gf_Option<boolean>;
    allScheduleChangeNotifications: __gf_Option<boolean>;
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
    validate(): Exit<AppointmentNotifications, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<AppointmentNotifications>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function appointmentNotificationsCreateForm(
    overrides?: Partial<AppointmentNotifications>
): AppointmentNotificationsGigaform {
    let data = $state({ ...appointmentNotificationsDefaultValue(), ...overrides });
    let errors = $state<AppointmentNotificationsErrors>({
        _errors: optionNone(),
        personalScheduleChangeNotifications: optionNone(),
        allScheduleChangeNotifications: optionNone()
    });
    let tainted = $state<AppointmentNotificationsTainted>({
        personalScheduleChangeNotifications: optionNone(),
        allScheduleChangeNotifications: optionNone()
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
            setError: (value: __gf_Option<Array<string>>) => {
                errors.personalScheduleChangeNotifications = value;
            },
            getTainted: () => tainted.personalScheduleChangeNotifications,
            setTainted: (value: __gf_Option<boolean>) => {
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
            setError: (value: __gf_Option<Array<string>>) => {
                errors.allScheduleChangeNotifications = value;
            },
            getTainted: () => tainted.allScheduleChangeNotifications,
            setTainted: (value: __gf_Option<boolean>) => {
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
    function validate(): Exit<AppointmentNotifications, Array<{ field: string; message: string }>> {
        return toExit(appointmentNotificationsDeserialize(data));
    }
    function reset(newOverrides?: Partial<AppointmentNotifications>): void {
        data = { ...appointmentNotificationsDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            personalScheduleChangeNotifications: optionNone(),
            allScheduleChangeNotifications: optionNone()
        };
        tainted = {
            personalScheduleChangeNotifications: optionNone(),
            allScheduleChangeNotifications: optionNone()
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
): Exit<AppointmentNotifications, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.personalScheduleChangeNotifications =
        formData.get('personalScheduleChangeNotifications') ?? '';
    obj.allScheduleChangeNotifications = formData.get('allScheduleChangeNotifications') ?? '';
    return toExit(appointmentNotificationsDeserialize(obj));
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
