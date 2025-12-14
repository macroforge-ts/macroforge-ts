import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

export interface AppointmentNotifications {
    personalScheduleChangeNotifications: string;

    allScheduleChangeNotifications: string;
}

export function defaultValueAppointmentNotifications(): AppointmentNotifications {
    return {
        personalScheduleChangeNotifications: '',
        allScheduleChangeNotifications: ''
    } as AppointmentNotifications;
}

export function toStringifiedJSONAppointmentNotifications(value: AppointmentNotifications): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeAppointmentNotifications(value, ctx));
}
export function toObjectAppointmentNotifications(
    value: AppointmentNotifications
): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeAppointmentNotifications(value, ctx);
}
export function __serializeAppointmentNotifications(
    value: AppointmentNotifications,
    ctx: SerializeContext
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

export function fromStringifiedJSONAppointmentNotifications(
    json: string,
    opts?: DeserializeOptions
): Result<AppointmentNotifications, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectAppointmentNotifications(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectAppointmentNotifications(
    obj: unknown,
    opts?: DeserializeOptions
): Result<AppointmentNotifications, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeAppointmentNotifications(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message:
                        'AppointmentNotifications.fromObject: root cannot be a forward reference'
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
export function __deserializeAppointmentNotifications(
    value: any,
    ctx: DeserializeContext
): AppointmentNotifications | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message: 'AppointmentNotifications.__deserialize: expected an object'
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
        throw new DeserializeError(errors);
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
        throw new DeserializeError(errors);
    }
    return instance as AppointmentNotifications;
}
export function validateFieldAppointmentNotifications<K extends keyof AppointmentNotifications>(
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
export function validateFieldsAppointmentNotifications(
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
export function hasShapeAppointmentNotifications(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'personalScheduleChangeNotifications' in o && 'allScheduleChangeNotifications' in o;
}
export function isAppointmentNotifications(obj: unknown): obj is AppointmentNotifications {
    if (!hasShapeAppointmentNotifications(obj)) {
        return false;
    }
    const result = fromObjectAppointmentNotifications(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsAppointmentNotifications = {
    _errors: Option<Array<string>>;
    personalScheduleChangeNotifications: Option<Array<string>>;
    allScheduleChangeNotifications: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedAppointmentNotifications = {
    personalScheduleChangeNotifications: Option<boolean>;
    allScheduleChangeNotifications: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersAppointmentNotifications {
    readonly personalScheduleChangeNotifications: FieldController<string>;
    readonly allScheduleChangeNotifications: FieldController<string>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformAppointmentNotifications {
    readonly data: AppointmentNotifications;
    readonly errors: ErrorsAppointmentNotifications;
    readonly tainted: TaintedAppointmentNotifications;
    readonly fields: FieldControllersAppointmentNotifications;
    validate(): Result<AppointmentNotifications, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<AppointmentNotifications>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormAppointmentNotifications(
    overrides?: Partial<AppointmentNotifications>
): GigaformAppointmentNotifications {
    let data = $state({ ...AppointmentNotifications.defaultValue(), ...overrides });
    let errors = $state<ErrorsAppointmentNotifications>({
        _errors: Option.none(),
        personalScheduleChangeNotifications: Option.none(),
        allScheduleChangeNotifications: Option.none()
    });
    let tainted = $state<TaintedAppointmentNotifications>({
        personalScheduleChangeNotifications: Option.none(),
        allScheduleChangeNotifications: Option.none()
    });
    const fields: FieldControllersAppointmentNotifications = {
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
                const fieldErrors = AppointmentNotifications.validateField(
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
                const fieldErrors = AppointmentNotifications.validateField(
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
        return AppointmentNotifications.fromObject(data);
    }
    function reset(newOverrides?: Partial<AppointmentNotifications>): void {
        data = { ...AppointmentNotifications.defaultValue(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataAppointmentNotifications(
    formData: FormData
): Result<AppointmentNotifications, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    obj.personalScheduleChangeNotifications =
        formData.get('personalScheduleChangeNotifications') ?? '';
    obj.allScheduleChangeNotifications = formData.get('allScheduleChangeNotifications') ?? '';
    return AppointmentNotifications.fromStringifiedJSON(JSON.stringify(obj));
}
