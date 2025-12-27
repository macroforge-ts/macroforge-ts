import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { rowHeightSerializeWithContext } from "./row-height.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { rowHeightDeserializeWithContext } from "./row-height.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { RowHeight } from './row-height.svelte';


export interface ScheduleSettings {
    daysPerWeek: number;
    
    rowHeight: RowHeight;
    visibleRoutes: Array<string>;
    detailedCards: boolean;
}

export function scheduleSettingsDefaultValue(): ScheduleSettings {
    return {
        daysPerWeek: 0,
        rowHeight: "Medium",
        visibleRoutes: [],
        detailedCards: false
    } as ScheduleSettings;
}

export function scheduleSettingsSerialize(value: ScheduleSettings): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(scheduleSettingsSerializeWithContext(value, ctx));
}
export function scheduleSettingsSerializeWithContext(value: ScheduleSettings, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"ScheduleSettings"}`,
        __id
    };
    result[`${"daysPerWeek"}`] = value.daysPerWeek;
    result[`${"rowHeight"}`] = rowHeightSerializeWithContext(value.rowHeight, ctx);
    result[`${"visibleRoutes"}`] = value.visibleRoutes;
    result[`${"detailedCards"}`] = value.detailedCards;
    return result;
}

export function scheduleSettingsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ScheduleSettings } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = scheduleSettingsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ScheduleSettings.deserialize: root cannot be a forward reference"
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
export function scheduleSettingsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ScheduleSettings | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ScheduleSettings"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"daysPerWeek"}` in obj)) {
        errors.push({
            field: `${"daysPerWeek"}`,
            message: "missing required field"
        });
    }
    if (!(`${"rowHeight"}` in obj)) {
        errors.push({
            field: `${"rowHeight"}`,
            message: "missing required field"
        });
    }
    if (!(`${"visibleRoutes"}` in obj)) {
        errors.push({
            field: `${"visibleRoutes"}`,
            message: "missing required field"
        });
    }
    if (!(`${"detailedCards"}` in obj)) {
        errors.push({
            field: `${"detailedCards"}`,
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
        const __raw_daysPerWeek = obj[`${"daysPerWeek"}`] as number;
        instance.daysPerWeek = __raw_daysPerWeek;
    }
    {
        const __raw_rowHeight = obj[`${"rowHeight"}`] as RowHeight;
        {
            const __result = rowHeightDeserializeWithContext(__raw_rowHeight, ctx);
            ctx.assignOrDefer(instance, `${"rowHeight"}`, __result);
        }
    }
    {
        const __raw_visibleRoutes = obj[`${"visibleRoutes"}`] as Array<string>;
        if (Array.isArray(__raw_visibleRoutes)) {
            instance.visibleRoutes = __raw_visibleRoutes as string[];
        }
    }
    {
        const __raw_detailedCards = obj[`${"detailedCards"}`] as boolean;
        instance.detailedCards = __raw_detailedCards;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ScheduleSettings;
}
export function scheduleSettingsValidateField<K extends keyof ScheduleSettings>(_field: K, _value: ScheduleSettings[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function scheduleSettingsValidateFields(_partial: Partial<ScheduleSettings>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function scheduleSettingsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"daysPerWeek" in o && "rowHeight" in o && "visibleRoutes" in o && "detailedCards" in o';
}
export function scheduleSettingsIs(obj: unknown): obj is ScheduleSettings {
    if (!scheduleSettingsHasShape(obj)) {
        return false;
    }
    const result = scheduleSettingsDeserialize(obj);
    return result.success;
}

export function scheduleSettingsFromFormData(formData: FormData): Exit<ScheduleSettings, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const daysPerWeekStr = formData.get(`${"daysPerWeek"}`);
        obj.daysPerWeek = daysPerWeekStr ? parseFloat(daysPerWeekStr as string) : $MfPh5;
        if (obj.daysPerWeek !== undefined && isNaN(obj.daysPerWeek as number)) obj.daysPerWeek = "0";
    }
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
    obj.visibleRoutes = formData.getAll(`${"visibleRoutes"}`) as Array<string>;
    {
        const detailedCardsVal = formData.get(`${"detailedCards"}`);
        obj.detailedCards = detailedCardsVal === "true" || detailedCardsVal === "on" || detailedCardsVal === "1";
    }
    return toExit("scheduleSettingsDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: ScheduleSettings;
    readonly errors: ScheduleSettingsErrors;
    readonly tainted: ScheduleSettingsTainted;
    readonly fields: ScheduleSettingsFieldControllers;
    validate(): Exit<ScheduleSettings, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<ScheduleSettings>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function scheduleSettingsCreateForm(overrides: Partial<ScheduleSettings>): ScheduleSettingsGigaform {}
let data = $state({
    ...scheduleSettingsDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as ScheduleSettingsErrors);
let tainted = $state<$MfPh3>({} as ScheduleSettingsTainted);
const fields = {} as ScheduleSettingsFieldControllers;
fields.daysPerWeek = {
    label: `${"daysPerWeek"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.rowHeight = {
    label: `${"rowHeight"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.visibleRoutes = {
    label: `${"visibleRoutes"}`,
    type: `${"text"}`,
    optional: false,
    array: true
};
fields.detailedCards = {
    label: `${"detailedCards"}`,
    type: `${"checkbox"}`,
    optional: false,
    array: false
};
function validate(): Exit<ScheduleSettings, Array<{
    field: string;
    message: string;
}>> {
    return toExit("scheduleSettingsDeserialize(data)");
    data = {
        ...scheduleSettingsDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const ScheduleSettings = {
  defaultValue: scheduleSettingsDefaultValue,
  serialize: scheduleSettingsSerialize,
  serializeWithContext: scheduleSettingsSerializeWithContext,
  deserialize: scheduleSettingsDeserialize,
  deserializeWithContext: scheduleSettingsDeserializeWithContext,
  validateFields: scheduleSettingsValidateFields,
  hasShape: scheduleSettingsHasShape,
  is: scheduleSettingsIs,
  fromFormData: scheduleSettingsFromFormData,
  createForm: scheduleSettingsCreateForm
} as const;