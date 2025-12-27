import { emailDefaultValue } from "./email.svelte";
import { settingsDefaultValue } from "./settings.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { emailSerializeWithContext } from "./email.svelte";
import { jobTitleSerializeWithContext } from "./job-title.svelte";
import { phoneNumberSerializeWithContext } from "./phone-number.svelte";
import { settingsSerializeWithContext } from "./settings.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { emailDeserializeWithContext } from "./email.svelte";
import { jobTitleDeserializeWithContext } from "./job-title.svelte";
import { settingsDeserializeWithContext } from "./settings.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Email } from './email.svelte';
import type { JobTitle } from './job-title.svelte';
import type { PhoneNumber } from './phone-number.svelte';
import type { Route } from './route.svelte';
import type { Settings } from './settings.svelte';


export interface Employee {
    id: string;
    imageUrl: string | null;
    
    name: string;
    phones: Array<PhoneNumber>;
    
    role: string;
    
    title: JobTitle;
    email: Email;
    
    address: string;
    
    username: string;
    
    route: string | Route;
    ratePerHour: number;
    active: boolean;
    isTechnician: boolean;
    isSalesRep: boolean;
    description: string | null;
    linkedinUrl: string | null;
    attendance: Array<string>;
    settings: Settings;
}

export function employeeDefaultValue(): Employee {
    return {
        id: "",
        imageUrl: null,
        name: "",
        phones: [],
        role: "",
        title: "Technician",
        email: emailDefaultValue(),
        address: "",
        username: "",
        route: "",
        ratePerHour: 0,
        active: false,
        isTechnician: false,
        isSalesRep: false,
        description: null,
        linkedinUrl: null,
        attendance: [],
        settings: settingsDefaultValue()
    } as Employee;
}

export function employeeSerialize(value: Employee): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(employeeSerializeWithContext(value, ctx));
}
export function employeeSerializeWithContext(value: Employee, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Employee"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"imageUrl"}`] = value.imageUrl;
    result[`${"name"}`] = value.name;
    result[`${"phones"}`] = value.phones.map((item)=>phoneNumberSerializeWithContext(item, ctx));
    result[`${"role"}`] = value.role;
    result[`${"title"}`] = jobTitleSerializeWithContext(value.title, ctx);
    result[`${"email"}`] = emailSerializeWithContext(value.email, ctx);
    result[`${"address"}`] = value.address;
    result[`${"username"}`] = value.username;
    result[`${"route"}`] = value.route;
    result[`${"ratePerHour"}`] = value.ratePerHour;
    result[`${"active"}`] = value.active;
    result[`${"isTechnician"}`] = value.isTechnician;
    result[`${"isSalesRep"}`] = value.isSalesRep;
    result[`${"description"}`] = value.description;
    result[`${"linkedinUrl"}`] = value.linkedinUrl;
    result[`${"attendance"}`] = value.attendance;
    result[`${"settings"}`] = settingsSerializeWithContext(value.settings, ctx);
    return result;
}

export function employeeDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Employee } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = employeeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Employee.deserialize: root cannot be a forward reference"
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
export function employeeDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Employee | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Employee"}.deserializeWithContext: expected an object`
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
    if (!(`${"imageUrl"}` in obj)) {
        errors.push({
            field: `${"imageUrl"}`,
            message: "missing required field"
        });
    }
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phones"}` in obj)) {
        errors.push({
            field: `${"phones"}`,
            message: "missing required field"
        });
    }
    if (!(`${"role"}` in obj)) {
        errors.push({
            field: `${"role"}`,
            message: "missing required field"
        });
    }
    if (!(`${"title"}` in obj)) {
        errors.push({
            field: `${"title"}`,
            message: "missing required field"
        });
    }
    if (!(`${"email"}` in obj)) {
        errors.push({
            field: `${"email"}`,
            message: "missing required field"
        });
    }
    if (!(`${"address"}` in obj)) {
        errors.push({
            field: `${"address"}`,
            message: "missing required field"
        });
    }
    if (!(`${"username"}` in obj)) {
        errors.push({
            field: `${"username"}`,
            message: "missing required field"
        });
    }
    if (!(`${"route"}` in obj)) {
        errors.push({
            field: `${"route"}`,
            message: "missing required field"
        });
    }
    if (!(`${"ratePerHour"}` in obj)) {
        errors.push({
            field: `${"ratePerHour"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isTechnician"}` in obj)) {
        errors.push({
            field: `${"isTechnician"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isSalesRep"}` in obj)) {
        errors.push({
            field: `${"isSalesRep"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
            message: "missing required field"
        });
    }
    if (!(`${"linkedinUrl"}` in obj)) {
        errors.push({
            field: `${"linkedinUrl"}`,
            message: "missing required field"
        });
    }
    if (!(`${"attendance"}` in obj)) {
        errors.push({
            field: `${"attendance"}`,
            message: "missing required field"
        });
    }
    if (!(`${"settings"}` in obj)) {
        errors.push({
            field: `${"settings"}`,
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
        const __raw_imageUrl = obj[`${"imageUrl"}`] as string | null;
        instance.imageUrl = __raw_imageUrl;
    }
    {
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Employee.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_phones = obj[`${"phones"}`] as Array<PhoneNumber>;
        if (Array.isArray(__raw_phones)) {
            instance.phones = __raw_phones as PhoneNumber[];
        }
    }
    {
        const __raw_role = obj[`${"role"}`] as string;
        if (__raw_role.trim().length === 0) {
            errors.push({
                field: "role",
                message: "Employee.role must not be empty"
            });
        }
        instance.role = __raw_role;
    }
    {
        const __raw_title = obj[`${"title"}`] as JobTitle;
        {
            const __result = jobTitleDeserializeWithContext(__raw_title, ctx);
            ctx.assignOrDefer(instance, `${"title"}`, __result);
        }
    }
    {
        const __raw_email = obj[`${"email"}`] as Email;
        {
            const __result = emailDeserializeWithContext(__raw_email, ctx);
            ctx.assignOrDefer(instance, `${"email"}`, __result);
        }
    }
    {
        const __raw_address = obj[`${"address"}`] as string;
        if (__raw_address.trim().length === 0) {
            errors.push({
                field: "address",
                message: "Employee.address must not be empty"
            });
        }
        instance.address = __raw_address;
    }
    {
        const __raw_username = obj[`${"username"}`] as string;
        if (__raw_username.trim().length === 0) {
            errors.push({
                field: "username",
                message: "Employee.username must not be empty"
            });
        }
        instance.username = __raw_username;
    }
    {
        const __raw_route = obj[`${"route"}`] as string | Route;
        instance.route = __raw_route;
    }
    {
        const __raw_ratePerHour = obj[`${"ratePerHour"}`] as number;
        instance.ratePerHour = __raw_ratePerHour;
    }
    {
        const __raw_active = obj[`${"active"}`] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_isTechnician = obj[`${"isTechnician"}`] as boolean;
        instance.isTechnician = __raw_isTechnician;
    }
    {
        const __raw_isSalesRep = obj[`${"isSalesRep"}`] as boolean;
        instance.isSalesRep = __raw_isSalesRep;
    }
    {
        const __raw_description = obj[`${"description"}`] as string | null;
        instance.description = __raw_description;
    }
    {
        const __raw_linkedinUrl = obj[`${"linkedinUrl"}`] as string | null;
        instance.linkedinUrl = __raw_linkedinUrl;
    }
    {
        const __raw_attendance = obj[`${"attendance"}`] as Array<string>;
        if (Array.isArray(__raw_attendance)) {
            instance.attendance = __raw_attendance as string[];
        }
    }
    {
        const __raw_settings = obj[`${"settings"}`] as Settings;
        {
            const __result = settingsDeserializeWithContext(__raw_settings, ctx);
            ctx.assignOrDefer(instance, `${"settings"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Employee;
}
export function employeeValidateField<K extends keyof Employee>(_field: K, _value: Employee[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"name"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Employee.name must not be empty"
            });
        }
    }
    if (_field === `${"role"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "role",
                message: "Employee.role must not be empty"
            });
        }
    }
    if (_field === `${"address"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "address",
                message: "Employee.address must not be empty"
            });
        }
    }
    if (_field === `${"username"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "username",
                message: "Employee.username must not be empty"
            });
        }
    }
    return errors;
}
export function employeeValidateFields(_partial: Partial<Employee>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"name"}` in _partial && _partial.name !== undefined) {
        const __val = _partial.name as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Employee.name must not be empty"
            });
        }
    }
    if (`${"role"}` in _partial && _partial.role !== undefined) {
        const __val = _partial.role as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "role",
                message: "Employee.role must not be empty"
            });
        }
    }
    if (`${"address"}` in _partial && _partial.address !== undefined) {
        const __val = _partial.address as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "address",
                message: "Employee.address must not be empty"
            });
        }
    }
    if (`${"username"}` in _partial && _partial.username !== undefined) {
        const __val = _partial.username as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "username",
                message: "Employee.username must not be empty"
            });
        }
    }
    return errors;
}
export function employeeHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "imageUrl" in o && "name" in o && "phones" in o && "role" in o && "title" in o && "email" in o && "address" in o && "username" in o && "route" in o && "ratePerHour" in o && "active" in o && "isTechnician" in o && "isSalesRep" in o && "description" in o && "linkedinUrl" in o && "attendance" in o && "settings" in o';
}
export function employeeIs(obj: unknown): obj is Employee {
    if (!employeeHasShape(obj)) {
        return false;
    }
    const result = employeeDeserialize(obj);
    return result.success;
}

export function employeeFromFormData(formData: FormData): Exit<Employee, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.imageUrl = formData.get(`${"imageUrl"}`) ?? "";
    obj.name = formData.get(`${"name"}`) ?? "";
    {
        const phonesItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"phones"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"phones"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"phones"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"phones"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                phonesItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.phones = phonesItems;
    }
    obj.role = formData.get(`${"role"}`) ?? "";
    {
        const titleObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"title"}.`)) {
                const fieldName = key.slice(`${"title"}.`.length);
                const parts = fieldName.split(".");
                let current = titleObj;
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
        obj.title = titleObj;
    }
    {
        const emailObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"email"}.`)) {
                const fieldName = key.slice(`${"email"}.`.length);
                const parts = fieldName.split(".");
                let current = emailObj;
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
        obj.email = emailObj;
    }
    obj.address = formData.get(`${"address"}`) ?? "";
    obj.username = formData.get(`${"username"}`) ?? "";
    obj.route = formData.get(`${"route"}`) ?? "";
    {
        const ratePerHourStr = formData.get(`${"ratePerHour"}`);
        obj.ratePerHour = ratePerHourStr ? parseFloat(ratePerHourStr as string) : $MfPh5;
        if (obj.ratePerHour !== undefined && isNaN(obj.ratePerHour as number)) obj.ratePerHour = "0";
    }
    {
        const activeVal = formData.get(`${"active"}`);
        obj.active = activeVal === "true" || activeVal === "on" || activeVal === "1";
    }
    {
        const isTechnicianVal = formData.get(`${"isTechnician"}`);
        obj.isTechnician = isTechnicianVal === "true" || isTechnicianVal === "on" || isTechnicianVal === "1";
    }
    {
        const isSalesRepVal = formData.get(`${"isSalesRep"}`);
        obj.isSalesRep = isSalesRepVal === "true" || isSalesRepVal === "on" || isSalesRepVal === "1";
    }
    obj.description = formData.get(`${"description"}`) ?? "";
    obj.linkedinUrl = formData.get(`${"linkedinUrl"}`) ?? "";
    obj.attendance = formData.getAll(`${"attendance"}`) as Array<string>;
    {
        const settingsObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"settings"}.`)) {
                const fieldName = key.slice(`${"settings"}.`.length);
                const parts = fieldName.split(".");
                let current = settingsObj;
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
        obj.settings = settingsObj;
    }
    return toExit("employeeDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Employee;
    readonly errors: EmployeeErrors;
    readonly tainted: EmployeeTainted;
    readonly fields: EmployeeFieldControllers;
    validate(): Exit<Employee, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Employee>): void;
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
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function employeeCreateForm(overrides: Partial<Employee>): EmployeeGigaform {}
let data = $state({
    ...employeeDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as EmployeeErrors);
let tainted = $state<$MfPh3>({} as EmployeeTainted);
const fields = {} as EmployeeFieldControllers;
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.imageUrl = {
    label: `${"imageUrl"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.name = {
    label: `${"name"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.phones = {
    label: `${"phones"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.role = {
    label: `${"role"}`,
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
fields.email = {
    label: `${"email"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.address = {
    label: `${"address"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.username = {
    label: `${"username"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.route = {
    label: `${"route"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.ratePerHour = {
    label: `${"ratePerHour"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.active = {
    label: `${"active"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.isTechnician = {
    label: `${"isTechnician"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.isSalesRep = {
    label: `${"isSalesRep"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.description = {
    label: `${"description"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.linkedinUrl = {
    label: `${"linkedinUrl"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.attendance = {
    label: `${"attendance"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.settings = {
    label: `${"settings"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Employee, Array<{
    field: string;
    message: string;
}>> {
    return toExit("employeeDeserialize(data)");
    data = {
        ...employeeDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Employee = {
  defaultValue: employeeDefaultValue,
  serialize: employeeSerialize,
  serializeWithContext: employeeSerializeWithContext,
  deserialize: employeeDeserialize,
  deserializeWithContext: employeeDeserializeWithContext,
  validateFields: employeeValidateFields,
  hasShape: employeeHasShape,
  is: employeeIs,
  fromFormData: employeeFromFormData,
  createForm: employeeCreateForm
} as const;