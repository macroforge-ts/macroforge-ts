import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Employee } from './employee.svelte';


export interface Route {
    id: string;
    techs: Array<string | Employee> | null;
    active: boolean;
    
    name: string;
    
    phone: string;
    
    position: string;
    serviceRoute: boolean;
    defaultDurationHours: number;
    tags: Array<string>;
    icon: string | null;
    color: string | null;
}

export function routeDefaultValue(): Route {
    return {
        id: "",
        techs: null,
        active: false,
        name: "",
        phone: "",
        position: "",
        serviceRoute: false,
        defaultDurationHours: 0,
        tags: [],
        icon: null,
        color: null
    } as Route;
}

export function routeSerialize(value: Route): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(routeSerializeWithContext(value, ctx));
}
export function routeSerializeWithContext(value: Route, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Route"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    if (value.techs !== null) {
        result[`${"techs"}`] = value.techs;
    }
    result[`${"active"}`] = value.active;
    result[`${"name"}`] = value.name;
    result[`${"phone"}`] = value.phone;
    result[`${"position"}`] = value.position;
    result[`${"serviceRoute"}`] = value.serviceRoute;
    result[`${"defaultDurationHours"}`] = value.defaultDurationHours;
    result[`${"tags"}`] = value.tags;
    result[`${"icon"}`] = value.icon;
    result[`${"color"}`] = value.color;
    return result;
}

export function routeDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Route } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = routeDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Route.deserialize: root cannot be a forward reference"
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
export function routeDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Route | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Route"}.deserializeWithContext: expected an object`
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
    if (!(`${"techs"}` in obj)) {
        errors.push({
            field: `${"techs"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
            message: "missing required field"
        });
    }
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"phone"}` in obj)) {
        errors.push({
            field: `${"phone"}`,
            message: "missing required field"
        });
    }
    if (!(`${"position"}` in obj)) {
        errors.push({
            field: `${"position"}`,
            message: "missing required field"
        });
    }
    if (!(`${"serviceRoute"}` in obj)) {
        errors.push({
            field: `${"serviceRoute"}`,
            message: "missing required field"
        });
    }
    if (!(`${"defaultDurationHours"}` in obj)) {
        errors.push({
            field: `${"defaultDurationHours"}`,
            message: "missing required field"
        });
    }
    if (!(`${"tags"}` in obj)) {
        errors.push({
            field: `${"tags"}`,
            message: "missing required field"
        });
    }
    if (!(`${"icon"}` in obj)) {
        errors.push({
            field: `${"icon"}`,
            message: "missing required field"
        });
    }
    if (!(`${"color"}` in obj)) {
        errors.push({
            field: `${"color"}`,
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
        const __raw_techs = obj[`${"techs"}`] as Array<string | Employee> | null;
        if (__raw_techs === null) {
            instance.techs = null;
        } else {
            instance.techs = __raw_techs;
        }
    }
    {
        const __raw_active = obj[`${"active"}`] as boolean;
        instance.active = __raw_active;
    }
    {
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "Route.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_phone = obj[`${"phone"}`] as string;
        if (__raw_phone.trim().length === 0) {
            errors.push({
                field: "phone",
                message: "Route.phone must not be empty"
            });
        }
        instance.phone = __raw_phone;
    }
    {
        const __raw_position = obj[`${"position"}`] as string;
        if (__raw_position.trim().length === 0) {
            errors.push({
                field: "position",
                message: "Route.position must not be empty"
            });
        }
        instance.position = __raw_position;
    }
    {
        const __raw_serviceRoute = obj[`${"serviceRoute"}`] as boolean;
        instance.serviceRoute = __raw_serviceRoute;
    }
    {
        const __raw_defaultDurationHours = obj[`${"defaultDurationHours"}`] as number;
        instance.defaultDurationHours = __raw_defaultDurationHours;
    }
    {
        const __raw_tags = obj[`${"tags"}`] as Array<string>;
        if (Array.isArray(__raw_tags)) {
            instance.tags = __raw_tags as string[];
        }
    }
    {
        const __raw_icon = obj[`${"icon"}`] as string | null;
        instance.icon = __raw_icon;
    }
    {
        const __raw_color = obj[`${"color"}`] as string | null;
        instance.color = __raw_color;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Route;
}
export function routeValidateField<K extends keyof Route>(_field: K, _value: Route[K]): Array<{
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
                message: "Route.name must not be empty"
            });
        }
    }
    if (_field === `${"phone"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "phone",
                message: "Route.phone must not be empty"
            });
        }
    }
    if (_field === `${"position"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "position",
                message: "Route.position must not be empty"
            });
        }
    }
    return errors;
}
export function routeValidateFields(_partial: Partial<Route>): Array<{
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
                message: "Route.name must not be empty"
            });
        }
    }
    if (`${"phone"}` in _partial && _partial.phone !== undefined) {
        const __val = _partial.phone as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "phone",
                message: "Route.phone must not be empty"
            });
        }
    }
    if (`${"position"}` in _partial && _partial.position !== undefined) {
        const __val = _partial.position as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "position",
                message: "Route.position must not be empty"
            });
        }
    }
    return errors;
}
export function routeHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "techs" in o && "active" in o && "name" in o && "phone" in o && "position" in o && "serviceRoute" in o && "defaultDurationHours" in o && "tags" in o && "icon" in o && "color" in o';
}
export function routeIs(obj: unknown): obj is Route {
    if (!routeHasShape(obj)) {
        return false;
    }
    const result = routeDeserialize(obj);
    return result.success;
}

export function routeFromFormData(formData: FormData): Exit<Route, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.techs = formData.get(`${"techs"}`) ?? "";
    {
        const activeVal = formData.get(`${"active"}`);
        obj.active = activeVal === "true" || activeVal === "on" || activeVal === "1";
    }
    obj.name = formData.get(`${"name"}`) ?? "";
    obj.phone = formData.get(`${"phone"}`) ?? "";
    obj.position = formData.get(`${"position"}`) ?? "";
    {
        const serviceRouteVal = formData.get(`${"serviceRoute"}`);
        obj.serviceRoute = serviceRouteVal === "true" || serviceRouteVal === "on" || serviceRouteVal === "1";
    }
    {
        const defaultDurationHoursStr = formData.get(`${"defaultDurationHours"}`);
        obj.defaultDurationHours = defaultDurationHoursStr ? parseFloat(defaultDurationHoursStr as string) : $MfPh5;
        if (obj.defaultDurationHours !== undefined && isNaN(obj.defaultDurationHours as number)) obj.defaultDurationHours = "0";
    }
    obj.tags = formData.getAll(`${"tags"}`) as Array<string>;
    obj.icon = formData.get(`${"icon"}`) ?? "";
    obj.color = formData.get(`${"color"}`) ?? "";
    return toExit("routeDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Route;
    readonly errors: RouteErrors;
    readonly tainted: RouteTainted;
    readonly fields: RouteFieldControllers;
    validate(): Exit<Route, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Route>): void;
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
 }; export function routeCreateForm(overrides: Partial<Route>): RouteGigaform {}
let data = $state({
    ...routeDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as RouteErrors);
let tainted = $state<$MfPh3>({} as RouteTainted);
const fields = {} as RouteFieldControllers;
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.techs = {
    label: `${"techs"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.active = {
    label: `${"active"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.name = {
    label: `${"name"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.phone = {
    label: `${"phone"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.position = {
    label: `${"position"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.serviceRoute = {
    label: `${"serviceRoute"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
fields.defaultDurationHours = {
    label: `${"defaultDurationHours"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.tags = {
    label: `${"tags"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.icon = {
    label: `${"icon"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.color = {
    label: `${"color"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Route, Array<{
    field: string;
    message: string;
}>> {
    return toExit("routeDeserialize(data)");
    data = {
        ...routeDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Route = {
  defaultValue: routeDefaultValue,
  serialize: routeSerialize,
  serializeWithContext: routeSerializeWithContext,
  deserialize: routeDeserialize,
  deserializeWithContext: routeDeserializeWithContext,
  validateFields: routeValidateFields,
  hasShape: routeHasShape,
  is: routeIs,
  fromFormData: routeFromFormData,
  createForm: routeCreateForm
} as const;