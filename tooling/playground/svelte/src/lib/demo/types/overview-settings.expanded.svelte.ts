import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { columnConfigSerializeWithContext } from "./column-config.svelte";
import { overviewDisplaySerializeWithContext } from "./overview-display.svelte";
import { rowHeightSerializeWithContext } from "./row-height.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { overviewDisplayDeserializeWithContext } from "./overview-display.svelte";
import { rowHeightDeserializeWithContext } from "./row-height.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { ColumnConfig } from './column-config.svelte';
import type { OverviewDisplay } from './overview-display.svelte';
import type { RowHeight } from './row-height.svelte';


export interface OverviewSettings {
    
    rowHeight: RowHeight;
    
    cardOrRow: OverviewDisplay;
    perPage: number;
    columnConfigs: Array<ColumnConfig>;
}

export function overviewSettingsDefaultValue(): OverviewSettings {
    return {
        rowHeight: "Medium",
        cardOrRow: "Table",
        perPage: 0,
        columnConfigs: []
    } as OverviewSettings;
}

export function overviewSettingsSerialize(value: OverviewSettings): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(overviewSettingsSerializeWithContext(value, ctx));
}
export function overviewSettingsSerializeWithContext(value: OverviewSettings, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"OverviewSettings"}`,
        __id
    };
    result[`${"rowHeight"}`] = rowHeightSerializeWithContext(value.rowHeight, ctx);
    result[`${"cardOrRow"}`] = overviewDisplaySerializeWithContext(value.cardOrRow, ctx);
    result[`${"perPage"}`] = value.perPage;
    result[`${"columnConfigs"}`] = value.columnConfigs.map((item)=>columnConfigSerializeWithContext(item, ctx));
    return result;
}

export function overviewSettingsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: OverviewSettings } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = overviewSettingsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "OverviewSettings.deserialize: root cannot be a forward reference"
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
export function overviewSettingsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): OverviewSettings | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"OverviewSettings"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"rowHeight"}` in obj)) {
        errors.push({
            field: `${"rowHeight"}`,
            message: "missing required field"
        });
    }
    if (!(`${"cardOrRow"}` in obj)) {
        errors.push({
            field: `${"cardOrRow"}`,
            message: "missing required field"
        });
    }
    if (!(`${"perPage"}` in obj)) {
        errors.push({
            field: `${"perPage"}`,
            message: "missing required field"
        });
    }
    if (!(`${"columnConfigs"}` in obj)) {
        errors.push({
            field: `${"columnConfigs"}`,
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
        const __raw_rowHeight = obj[`${"rowHeight"}`] as RowHeight;
        {
            const __result = rowHeightDeserializeWithContext(__raw_rowHeight, ctx);
            ctx.assignOrDefer(instance, `${"rowHeight"}`, __result);
        }
    }
    {
        const __raw_cardOrRow = obj[`${"cardOrRow"}`] as OverviewDisplay;
        {
            const __result = overviewDisplayDeserializeWithContext(__raw_cardOrRow, ctx);
            ctx.assignOrDefer(instance, `${"cardOrRow"}`, __result);
        }
    }
    {
        const __raw_perPage = obj[`${"perPage"}`] as number;
        instance.perPage = __raw_perPage;
    }
    {
        const __raw_columnConfigs = obj[`${"columnConfigs"}`] as Array<ColumnConfig>;
        if (Array.isArray(__raw_columnConfigs)) {
            instance.columnConfigs = __raw_columnConfigs as ColumnConfig[];
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as OverviewSettings;
}
export function overviewSettingsValidateField<K extends keyof OverviewSettings>(_field: K, _value: OverviewSettings[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function overviewSettingsValidateFields(_partial: Partial<OverviewSettings>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function overviewSettingsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"rowHeight" in o && "cardOrRow" in o && "perPage" in o && "columnConfigs" in o';
}
export function overviewSettingsIs(obj: unknown): obj is OverviewSettings {
    if (!overviewSettingsHasShape(obj)) {
        return false;
    }
    const result = overviewSettingsDeserialize(obj);
    return result.success;
}

export function overviewSettingsFromFormData(formData: FormData): Exit<OverviewSettings, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const rowHeightObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"rowHeight"}.`)) {
                const fieldName = key.slice(`${"rowHeight"}.`.length);
                const parts = fieldName.split(".");
                let current = rowHeightObj;
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
        obj.rowHeight = rowHeightObj;
    }
    {
        const cardOrRowObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"cardOrRow"}.`)) {
                const fieldName = key.slice(`${"cardOrRow"}.`.length);
                const parts = fieldName.split(".");
                let current = cardOrRowObj;
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
        obj.cardOrRow = cardOrRowObj;
    }
    {
        const perPageStr = formData.get(`${"perPage"}`);
        obj.perPage = perPageStr ? parseFloat(perPageStr as string) : $MfPh5;
        if (obj.perPage !== undefined && isNaN(obj.perPage as number)) obj.perPage = "0";
    }
    {
        const columnConfigsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while(formData.has(`${"columnConfigs"}.` + idx + ".") || idx === 0){
            const hasAny = Array.from(formData.keys()).some((k)=>k.startsWith(`${"columnConfigs"}.` + idx + "."));
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())){
                    if (key.startsWith(`${"columnConfigs"}.` + idx + ".")) {
                        const fieldName = key.slice(`${"columnConfigs"}.`.length + String(idx).length + 1);
                        item[fieldName] = value;
                    }
                }
                columnConfigsItems.push(item);
            }
            idx++;
            if (idx > 1000) break;
        }
        obj.columnConfigs = columnConfigsItems;
    }
    return toExit("overviewSettingsDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: OverviewSettings;
    readonly errors: OverviewSettingsErrors;
    readonly tainted: OverviewSettingsTainted;
    readonly fields: OverviewSettingsFieldControllers;
    validate(): Exit<OverviewSettings, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<OverviewSettings>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function overviewSettingsCreateForm(overrides: Partial<OverviewSettings>): OverviewSettingsGigaform {}
let data = $state({
    ...overviewSettingsDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as OverviewSettingsErrors);
let tainted = $state<$MfPh3>({} as OverviewSettingsTainted);
const fields = {} as OverviewSettingsFieldControllers;
fields.rowHeight = {
    label: `${"rowHeight"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.cardOrRow = {
    label: `${"cardOrRow"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.perPage = {
    label: `${"perPage"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.columnConfigs = {
    label: `${"columnConfigs"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
function validate(): Exit<OverviewSettings, Array<{
    field: string;
    message: string;
}>> {
    return toExit("overviewSettingsDeserialize(data)");
    data = {
        ...overviewSettingsDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const OverviewSettings = {
  defaultValue: overviewSettingsDefaultValue,
  serialize: overviewSettingsSerialize,
  serializeWithContext: overviewSettingsSerializeWithContext,
  deserialize: overviewSettingsDeserialize,
  deserializeWithContext: overviewSettingsDeserializeWithContext,
  validateFields: overviewSettingsValidateFields,
  hasShape: overviewSettingsHasShape,
  is: overviewSettingsIs,
  fromFormData: overviewSettingsFromFormData,
  createForm: overviewSettingsCreateForm
} as const;