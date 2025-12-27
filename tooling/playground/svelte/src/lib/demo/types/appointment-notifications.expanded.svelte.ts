import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
/** import macro {Gigaform} from "@playground/macro"; */


export interface AppointmentNotifications {
    
    personalScheduleChangeNotifications: string;
    
    allScheduleChangeNotifications: string;
}

export function appointmentNotificationsDefaultValue(): AppointmentNotifications {
    return {
        personalScheduleChangeNotifications: "",
        allScheduleChangeNotifications: ""
    } as AppointmentNotifications;
}

export function appointmentNotificationsSerialize(value: AppointmentNotifications): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(appointmentNotificationsSerializeWithContext(value, ctx));
}
export function appointmentNotificationsSerializeWithContext(value: AppointmentNotifications, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"AppointmentNotifications"}`,
        __id
    };
    result[`${"personalScheduleChangeNotifications"}`] = value.personalScheduleChangeNotifications;
    result[`${"allScheduleChangeNotifications"}`] = value.allScheduleChangeNotifications;
    return result;
}

export function appointmentNotificationsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: AppointmentNotifications } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appointmentNotificationsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "AppointmentNotifications.deserialize: root cannot be a forward reference"
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return {
            success: true,
            value: resultOrRef
        };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return {
                success: false,
                errors: e.errors
            };
        }
        const message = e instanceof Error ? e.message : String(e);
        return {
            success: false,
            errors: [
                {
                    field: "_root",
                    message
                }
            ]
        };
    }
}
export function appointmentNotificationsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): AppointmentNotifications | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"AppointmentNotifications"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"personalScheduleChangeNotifications"}` in obj)) {
        errors.push({
            field: `${"personalScheduleChangeNotifications"}`,
            message: "missing required field"
        });
    }
    if (!(`${"allScheduleChangeNotifications"}` in obj)) {
        errors.push({
            field: `${"allScheduleChangeNotifications"}`,
            message: "missing required field"
        });
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
        const __raw_personalScheduleChangeNotifications = obj[`${"personalScheduleChangeNotifications"}`] as string;
        if (__raw_personalScheduleChangeNotifications.trim().length === 0) {
            errors.push({
                field: "personalScheduleChangeNotifications",
                message: "AppointmentNotifications.personalScheduleChangeNotifications must not be empty"
            });
        }
        instance.personalScheduleChangeNotifications = __raw_personalScheduleChangeNotifications;
    }
    {
        const __raw_allScheduleChangeNotifications = obj[`${"allScheduleChangeNotifications"}`] as string;
        if (__raw_allScheduleChangeNotifications.trim().length === 0) {
            errors.push({
                field: "allScheduleChangeNotifications",
                message: "AppointmentNotifications.allScheduleChangeNotifications must not be empty"
            });
        }
        instance.allScheduleChangeNotifications = __raw_allScheduleChangeNotifications;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as AppointmentNotifications;
}
export function appointmentNotificationsValidateField<K extends keyof AppointmentNotifications>(_field: K, _value: AppointmentNotifications[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"personalScheduleChangeNotifications"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "personalScheduleChangeNotifications",
                message: "AppointmentNotifications.personalScheduleChangeNotifications must not be empty"
            });
        }
    }
    if (_field === `${"allScheduleChangeNotifications"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "allScheduleChangeNotifications",
                message: "AppointmentNotifications.allScheduleChangeNotifications must not be empty"
            });
        }
    }
    return errors;
}
export function appointmentNotificationsValidateFields(_partial: Partial<AppointmentNotifications>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"personalScheduleChangeNotifications"}` in _partial && _partial.personalScheduleChangeNotifications !== undefined) {
        const __val = _partial.personalScheduleChangeNotifications as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "personalScheduleChangeNotifications",
                message: "AppointmentNotifications.personalScheduleChangeNotifications must not be empty"
            });
        }
    }
    if (`${"allScheduleChangeNotifications"}` in _partial && _partial.allScheduleChangeNotifications !== undefined) {
        const __val = _partial.allScheduleChangeNotifications as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "allScheduleChangeNotifications",
                message: "AppointmentNotifications.allScheduleChangeNotifications must not be empty"
            });
        }
    }
    return errors;
}
export function appointmentNotificationsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"personalScheduleChangeNotifications" in o && "allScheduleChangeNotifications" in o';
}
export function appointmentNotificationsIs(obj: unknown): obj is AppointmentNotifications {
    if (!appointmentNotificationsHasShape(obj)) {
        return false;
    }
    const result = appointmentNotificationsDeserialize(obj);
    return result.success;
}

export const AppointmentNotifications = {
  defaultValue: appointmentNotificationsDefaultValue,
  serialize: appointmentNotificationsSerialize,
  serializeWithContext: appointmentNotificationsSerializeWithContext,
  deserialize: appointmentNotificationsDeserialize,
  deserializeWithContext: appointmentNotificationsDeserializeWithContext,
  validateFields: appointmentNotificationsValidateFields,
  hasShape: appointmentNotificationsHasShape,
  is: appointmentNotificationsIs
} as const;