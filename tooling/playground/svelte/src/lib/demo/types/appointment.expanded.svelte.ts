import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { colorsSerializeWithContext } from "./colors.svelte";
import { recurrenceRuleSerializeWithContext } from "./recurrence-rule.svelte";
import { statusSerializeWithContext } from "./status.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { colorsDeserializeWithContext } from "./colors.svelte";
import { recurrenceRuleDeserializeWithContext } from "./recurrence-rule.svelte";
import { statusDeserializeWithContext } from "./status.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { DateTime } from 'effect';
import type { Option } from 'effect/Option';
import type { Colors } from './colors.svelte';
import type { Employee } from './employee.svelte';
import type { RecurrenceRule } from './recurrence-rule.svelte';
import type { Site } from './site.svelte';
import type { Status } from './status.svelte';


export interface Appointment {
    
    id: string;
    
    
    title: string;
    
    
    status: Status;
    
    begins: DateTime.DateTime;
    
    duration: number;
    
    timeZone: string;
    
    offsetMs: number;
    
    allDay: boolean;
    
    multiDay: boolean;
    
    employees: Array<string | Employee>;
    
    
    location: string | Site;
    
    description: Option<string>;
    
    
    colors: Colors;
    
    recurrenceRule: RecurrenceRule | null;
}

export function appointmentDefaultValue(): Appointment {
    return {
        id: "",
        title: "",
        status: "Scheduled",
        begins: dateTime.dateTimeDefaultValue(),
        duration: 0,
        timeZone: "",
        offsetMs: 0,
        allDay: false,
        multiDay: false,
        employees: [],
        location: "",
        description: optionDefaultValue<string>(),
        colors: {
            main: "#000000",
            hover: "#333333",
            active: "#666666"
        },
        recurrenceRule: null
    } as Appointment;
}

export function appointmentSerialize(value: Appointment): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(appointmentSerializeWithContext(value, ctx));
}
export function appointmentSerializeWithContext(value: Appointment, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Appointment"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"title"}`] = value.title;
    result[`${"status"}`] = statusSerializeWithContext(value.status, ctx);
    result[`${"begins"}`] = dateTime.dateTimeSerializeWithContext(value.begins, ctx);
    result[`${"duration"}`] = value.duration;
    result[`${"timeZone"}`] = value.timeZone;
    result[`${"offsetMs"}`] = value.offsetMs;
    result[`${"allDay"}`] = value.allDay;
    result[`${"multiDay"}`] = value.multiDay;
    result[`${"employees"}`] = value.employees;
    result[`${"location"}`] = value.location;
    result[`${"description"}`] = option<string>SerializeWithContext(value.description, ctx);
    result[`${"colors"}`] = colorsSerializeWithContext(value.colors, ctx);
    if (value.recurrenceRule !== null) {
        result[`${"recurrenceRule"}`] = recurrenceRuleSerializeWithContext(value.recurrenceRule, ctx);
    }
    return result;
}

export function appointmentDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Appointment } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appointmentDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Appointment.deserialize: root cannot be a forward reference"
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
export function appointmentDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Appointment | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Appointment"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"id"}` in obj)) {
        errors.push({
            field: `${"id"}`,
            message: "missing required field"
        });
    }
    if (!(`${"title"}` in obj)) {
        errors.push({
            field: `${"title"}`,
            message: "missing required field"
        });
    }
    if (!(`${"status"}` in obj)) {
        errors.push({
            field: `${"status"}`,
            message: "missing required field"
        });
    }
    if (!(`${"begins"}` in obj)) {
        errors.push({
            field: `${"begins"}`,
            message: "missing required field"
        });
    }
    if (!(`${"duration"}` in obj)) {
        errors.push({
            field: `${"duration"}`,
            message: "missing required field"
        });
    }
    if (!(`${"timeZone"}` in obj)) {
        errors.push({
            field: `${"timeZone"}`,
            message: "missing required field"
        });
    }
    if (!(`${"offsetMs"}` in obj)) {
        errors.push({
            field: `${"offsetMs"}`,
            message: "missing required field"
        });
    }
    if (!(`${"allDay"}` in obj)) {
        errors.push({
            field: `${"allDay"}`,
            message: "missing required field"
        });
    }
    if (!(`${"multiDay"}` in obj)) {
        errors.push({
            field: `${"multiDay"}`,
            message: "missing required field"
        });
    }
    if (!(`${"employees"}` in obj)) {
        errors.push({
            field: `${"employees"}`,
            message: "missing required field"
        });
    }
    if (!(`${"location"}` in obj)) {
        errors.push({
            field: `${"location"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
            message: "missing required field"
        });
    }
    if (!(`${"colors"}` in obj)) {
        errors.push({
            field: `${"colors"}`,
            message: "missing required field"
        });
    }
    if (!(`${"recurrenceRule"}` in obj)) {
        errors.push({
            field: `${"recurrenceRule"}`,
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
        const __raw_id = obj[`${"id"}`] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_title = obj[`${"title"}`] as string;
        if (__raw_title.trim().length === 0) {
            errors.push({
                field: "title",
                message: "Appointment.title must not be empty"
            });
        }
        instance.title = __raw_title;
    }
    {
        const __raw_status = obj[`${"status"}`] as Status;
        {
            const __result = statusDeserializeWithContext(__raw_status, ctx);
            ctx.assignOrDefer(instance, `${"status"}`, __result);
        }
    }
    {
        const __raw_begins = obj[`${"begins"}`] as DateTime.DateTime;
        {
            const __result = dateTime.dateTimeDeserializeWithContext(__raw_begins, ctx);
            ctx.assignOrDefer(instance, `${"begins"}`, __result);
        }
    }
    {
        const __raw_duration = obj[`${"duration"}`] as number;
        instance.duration = __raw_duration;
    }
    {
        const __raw_timeZone = obj[`${"timeZone"}`] as string;
        instance.timeZone = __raw_timeZone;
    }
    {
        const __raw_offsetMs = obj[`${"offsetMs"}`] as number;
        instance.offsetMs = __raw_offsetMs;
    }
    {
        const __raw_allDay = obj[`${"allDay"}`] as boolean;
        instance.allDay = __raw_allDay;
    }
    {
        const __raw_multiDay = obj[`${"multiDay"}`] as boolean;
        instance.multiDay = __raw_multiDay;
    }
    {
        const __raw_employees = obj[`${"employees"}`] as Array<string | Employee>;
        if (Array.isArray(__raw_employees)) {
            instance.employees = __raw_employees as string | Employee[];
        }
    }
    {
        const __raw_location = obj[`${"location"}`] as string | Site;
        instance.location = __raw_location;
    }
    {
        const __raw_description = obj[`${"description"}`] as Option<string>;
        {
            const __result = option<string>DeserializeWithContext(__raw_description, ctx);
            ctx.assignOrDefer(instance, `${"description"}`, __result);
        }
    }
    {
        const __raw_colors = obj[`${"colors"}`] as Colors;
        {
            const __result = colorsDeserializeWithContext(__raw_colors, ctx);
            ctx.assignOrDefer(instance, `${"colors"}`, __result);
        }
    }
    {
        const __raw_recurrenceRule = obj[`${"recurrenceRule"}`] as RecurrenceRule | null;
        if (__raw_recurrenceRule === null) {
            instance.recurrenceRule = null;
        } else {
            const __result = recurrenceRuleDeserializeWithContext(__raw_recurrenceRule, ctx);
            ctx.assignOrDefer(instance, `${"recurrenceRule"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Appointment;
}
export function appointmentValidateField<K extends keyof Appointment>(_field: K, _value: Appointment[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"title"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "title",
                message: "Appointment.title must not be empty"
            });
        }
    }
    return errors;
}
export function appointmentValidateFields(_partial: Partial<Appointment>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"title"}` in _partial && _partial.title !== undefined) {
        const __val = _partial.title as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "title",
                message: "Appointment.title must not be empty"
            });
        }
    }
    return errors;
}
export function appointmentHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "title" in o && "status" in o && "begins" in o && "duration" in o && "timeZone" in o && "offsetMs" in o && "allDay" in o && "multiDay" in o && "employees" in o && "location" in o && "description" in o && "colors" in o && "recurrenceRule" in o';
}
export function appointmentIs(obj: unknown): obj is Appointment {
    if (!appointmentHasShape(obj)) {
        return false;
    }
    const result = appointmentDeserialize(obj);
    return result.success;
}

export function appointmentFromFormData(formData: FormData): Exit<Appointment, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.title = formData.get(`${"title"}`) ?? "";
    {
        const statusObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"status"}.`)) {
                const fieldName = key.slice(`${"status"}.`.length);
                const parts = fieldName.split(".");
                let current = statusObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.status = statusObj;
    }
    {
        const beginsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"begins"}.`)) {
                const fieldName = key.slice(`${"begins"}.`.length);
                const parts = fieldName.split(".");
                let current = beginsObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.begins = beginsObj;
    }
    {
        const durationStr = formData.get(`${"duration"}`);
        obj.duration = durationStr ? parseFloat(durationStr as string) : $MfPh5;
        if (obj.duration !== undefined && isNaN(obj.duration as number)) obj.duration = "0";
    }
    obj.timeZone = formData.get(`${"timeZone"}`) ?? "";
    {
        const offsetMsStr = formData.get(`${"offsetMs"}`);
        obj.offsetMs = offsetMsStr ? parseFloat(offsetMsStr as string) : $MfPh5;
        if (obj.offsetMs !== undefined && isNaN(obj.offsetMs as number)) obj.offsetMs = "0";
    }
    {
        const allDayVal = formData.get(`${"allDay"}`);
        obj.allDay = allDayVal === "true" || allDayVal === "on" || allDayVal === "1";
    }
    {
        const multiDayVal = formData.get(`${"multiDay"}`);
        obj.multiDay = multiDayVal === "true" || multiDayVal === "on" || multiDayVal === "1";
    }
    obj.employees = formData.getAll(`${"employees"}`) as Array<string>;
    obj.location = formData.get(`${"location"}`) ?? "";
    {
        const descriptionObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"description"}.`)) {
                const fieldName = key.slice(`${"description"}.`.length);
                const parts = fieldName.split(".");
                let current = descriptionObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.description = descriptionObj;
    }
    {
        const colorsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"colors"}.`)) {
                const fieldName = key.slice(`${"colors"}.`.length);
                const parts = fieldName.split(".");
                let current = colorsObj;
                for(let i = 0; i < parts.length - 1; i++){
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.colors = colorsObj;
    }
    obj.recurrenceRule = formData.get(`${"recurrenceRule"}`) ?? "";
    return toExit("appointmentDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Appointment;
    readonly errors: AppointmentErrors;
    readonly tainted: AppointmentTainted;
    readonly fields: AppointmentFieldControllers;
    validate(): Exit<Appointment, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Appointment>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function appointmentCreateForm(overrides: Partial<Appointment>): AppointmentGigaform {}
let data = $state({
    ...appointmentDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as AppointmentErrors);
let tainted = $state<$MfPh3>({} as AppointmentTainted);
const fields = {} as AppointmentFieldControllers;
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.title = {
    label: `${"title"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.status = {
    label: `${"status"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.begins = {
    label: `${"begins"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.duration = {
    label: `${"duration"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.timeZone = {
    label: `${"timeZone"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.offsetMs = {
    label: `${"offsetMs"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.allDay = {
    label: `${"allDay"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.multiDay = {
    label: `${"multiDay"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.employees = {
    label: `${"employees"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.location = {
    label: `${"location"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.description = {
    label: `${"description"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.colors = {
    label: `${"colors"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.recurrenceRule = {
    label: `${"recurrenceRule"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Appointment, Array<{
    field: string;
    message: string;
}>> {
    return toExit("appointmentDeserialize(data)");
    data = {
        ...appointmentDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Appointment = {
  defaultValue: appointmentDefaultValue,
  serialize: appointmentSerialize,
  serializeWithContext: appointmentSerializeWithContext,
  deserialize: appointmentDeserialize,
  deserializeWithContext: appointmentDeserializeWithContext,
  validateFields: appointmentValidateFields,
  hasShape: appointmentHasShape,
  is: appointmentIs,
  fromFormData: appointmentFromFormData,
  createForm: appointmentCreateForm
} as const;