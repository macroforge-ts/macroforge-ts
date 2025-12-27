import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { applicationsSerializeWithContext } from "./applications.svelte";
import { pageSerializeWithContext } from "./page.svelte";
import { tableSerializeWithContext } from "./table.svelte";
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

import type { Applications } from './applications.svelte';
import type { Page } from './page.svelte';
import type { Table } from './table.svelte';


export interface AppPermissions {
    applications: Array<Applications>;
    pages: Array<Page>;
    data: Array<Table>;
}

export function appPermissionsDefaultValue(): AppPermissions {
    return {
        applications: [],
        pages: [],
        data: []
    } as AppPermissions;
}

export function appPermissionsSerialize(value: AppPermissions): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(appPermissionsSerializeWithContext(value, ctx));
}
export function appPermissionsSerializeWithContext(value: AppPermissions, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"AppPermissions"}`,
        __id
    };
    result[`${"applications"}`] = value.applications.map((item)=>applicationsSerializeWithContext(item, ctx));
    result[`${"pages"}`] = value.pages.map((item)=>pageSerializeWithContext(item, ctx));
    result[`${"data"}`] = value.data.map((item)=>tableSerializeWithContext(item, ctx));
    return result;
}

export function appPermissionsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: AppPermissions } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = appPermissionsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "AppPermissions.deserialize: root cannot be a forward reference"
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
export function appPermissionsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): AppPermissions | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"AppPermissions"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"applications"}` in obj)) {
        errors.push({
            field: `${"applications"}`,
            message: "missing required field"
        });
    }
    if (!(`${"pages"}` in obj)) {
        errors.push({
            field: `${"pages"}`,
            message: "missing required field"
        });
    }
    if (!(`${"data"}` in obj)) {
        errors.push({
            field: `${"data"}`,
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
        const __raw_applications = obj[`${"applications"}`] as Array<Applications>;
        if (Array.isArray(__raw_applications)) {
            instance.applications = __raw_applications as Applications[];
        }
    }
    {
        const __raw_pages = obj[`${"pages"}`] as Array<Page>;
        if (Array.isArray(__raw_pages)) {
            instance.pages = __raw_pages as Page[];
        }
    }
    {
        const __raw_data = obj[`${"data"}`] as Array<Table>;
        if (Array.isArray(__raw_data)) {
            instance.data = __raw_data as Table[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as AppPermissions;
}
export function appPermissionsValidateField<K extends keyof AppPermissions>(_field: K, _value: AppPermissions[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function appPermissionsValidateFields(_partial: Partial<AppPermissions>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function appPermissionsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"applications" in o && "pages" in o && "data" in o';
}
export function appPermissionsIs(obj: unknown): obj is AppPermissions {
    if (!appPermissionsHasShape(obj)) {
        return false;
    }
    const result = appPermissionsDeserialize(obj);
    return result.success;
}

export function appPermissionsFromFormData(formData: FormData): Exit<AppPermissions, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const applicationsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"applications"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"applications"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"applications"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"applications"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                applicationsItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.applications = applicationsItems;
    }
    {
        const pagesItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"pages"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"pages"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"pages"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"pages"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                pagesItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.pages = pagesItems;
    }
    {
        const dataItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"data"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"data"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"data"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"data"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                dataItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.data = dataItems;
    }
    return toExit("appPermissionsDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: AppPermissions;
    readonly errors: AppPermissionsErrors;
    readonly tainted: AppPermissionsTainted;
    readonly fields: AppPermissionsFieldControllers;
    validate(): Exit<AppPermissions, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<AppPermissions>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function appPermissionsCreateForm(overrides: Partial<AppPermissions>): AppPermissionsGigaform {}
let data = $state({
    ...appPermissionsDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as AppPermissionsErrors);
let tainted = $state<$MfPh3>({} as AppPermissionsTainted);
const fields = {} as AppPermissionsFieldControllers;
fields.applications = {
    label: `${"applications"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.pages = {
    label: `${"pages"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.data = {
    label: `${"data"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
function validate(): Exit<AppPermissions, Array<{
    field: string;
    message: string;
}>> {
    return toExit("appPermissionsDeserialize(data)");
    data = {
        ...appPermissionsDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const AppPermissions = {
  defaultValue: appPermissionsDefaultValue,
  serialize: appPermissionsSerialize,
  serializeWithContext: appPermissionsSerializeWithContext,
  deserialize: appPermissionsDeserialize,
  deserializeWithContext: appPermissionsDeserializeWithContext,
  validateFields: appPermissionsValidateFields,
  hasShape: appPermissionsHasShape,
  is: appPermissionsIs,
  fromFormData: appPermissionsFromFormData,
  createForm: appPermissionsCreateForm
} as const;