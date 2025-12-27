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
/** import macro {Gigaform} from "@playground/macro"; */


export interface Ordinal {
    north: number;
    northeast: number;
    east: number;
    southeast: number;
    south: number;
    southwest: number;
    west: number;
    northwest: number;
}

export function ordinalDefaultValue(): Ordinal {
    return {
        north: 0,
        northeast: 0,
        east: 0,
        southeast: 0,
        south: 0,
        southwest: 0,
        west: 0,
        northwest: 0
    } as Ordinal;
}

export function ordinalSerialize(value: Ordinal): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(ordinalSerializeWithContext(value, ctx));
}
export function ordinalSerializeWithContext(value: Ordinal, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Ordinal"}`,
        __id
    };
    result[`${"north"}`] = value.north;
    result[`${"northeast"}`] = value.northeast;
    result[`${"east"}`] = value.east;
    result[`${"southeast"}`] = value.southeast;
    result[`${"south"}`] = value.south;
    result[`${"southwest"}`] = value.southwest;
    result[`${"west"}`] = value.west;
    result[`${"northwest"}`] = value.northwest;
    return result;
}

export function ordinalDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Ordinal } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = ordinalDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Ordinal.deserialize: root cannot be a forward reference"
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
export function ordinalDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Ordinal | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Ordinal"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"north"}` in obj)) {
        errors.push({
            field: `${"north"}`,
            message: "missing required field"
        });
    }
    if (!(`${"northeast"}` in obj)) {
        errors.push({
            field: `${"northeast"}`,
            message: "missing required field"
        });
    }
    if (!(`${"east"}` in obj)) {
        errors.push({
            field: `${"east"}`,
            message: "missing required field"
        });
    }
    if (!(`${"southeast"}` in obj)) {
        errors.push({
            field: `${"southeast"}`,
            message: "missing required field"
        });
    }
    if (!(`${"south"}` in obj)) {
        errors.push({
            field: `${"south"}`,
            message: "missing required field"
        });
    }
    if (!(`${"southwest"}` in obj)) {
        errors.push({
            field: `${"southwest"}`,
            message: "missing required field"
        });
    }
    if (!(`${"west"}` in obj)) {
        errors.push({
            field: `${"west"}`,
            message: "missing required field"
        });
    }
    if (!(`${"northwest"}` in obj)) {
        errors.push({
            field: `${"northwest"}`,
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
        const __raw_north = obj[`${"north"}`] as number;
        instance.north = __raw_north;
    }
    {
        const __raw_northeast = obj[`${"northeast"}`] as number;
        instance.northeast = __raw_northeast;
    }
    {
        const __raw_east = obj[`${"east"}`] as number;
        instance.east = __raw_east;
    }
    {
        const __raw_southeast = obj[`${"southeast"}`] as number;
        instance.southeast = __raw_southeast;
    }
    {
        const __raw_south = obj[`${"south"}`] as number;
        instance.south = __raw_south;
    }
    {
        const __raw_southwest = obj[`${"southwest"}`] as number;
        instance.southwest = __raw_southwest;
    }
    {
        const __raw_west = obj[`${"west"}`] as number;
        instance.west = __raw_west;
    }
    {
        const __raw_northwest = obj[`${"northwest"}`] as number;
        instance.northwest = __raw_northwest;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Ordinal;
}
export function ordinalValidateField<K extends keyof Ordinal>(_field: K, _value: Ordinal[K]): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function ordinalValidateFields(_partial: Partial<Ordinal>): Array<{
    field: string;
    message: string;
}> {
    return [];
}
export function ordinalHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"north" in o && "northeast" in o && "east" in o && "southeast" in o && "south" in o && "southwest" in o && "west" in o && "northwest" in o';
}
export function ordinalIs(obj: unknown): obj is Ordinal {
    if (!ordinalHasShape(obj)) {
        return false;
    }
    const result = ordinalDeserialize(obj);
    return result.success;
}

export function ordinalFromFormData(formData: FormData): Exit<Ordinal, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const northStr = formData.get(`${"north"}`);
        obj.north = northStr ? parseFloat(northStr as string) : $MfPh5;
        if (obj.north !== undefined && isNaN(obj.north as number)) obj.north = "0";
    }
    {
        const northeastStr = formData.get(`${"northeast"}`);
        obj.northeast = northeastStr ? parseFloat(northeastStr as string) : $MfPh5;
        if (obj.northeast !== undefined && isNaN(obj.northeast as number)) obj.northeast = "0";
    }
    {
        const eastStr = formData.get(`${"east"}`);
        obj.east = eastStr ? parseFloat(eastStr as string) : $MfPh5;
        if (obj.east !== undefined && isNaN(obj.east as number)) obj.east = "0";
    }
    {
        const southeastStr = formData.get(`${"southeast"}`);
        obj.southeast = southeastStr ? parseFloat(southeastStr as string) : $MfPh5;
        if (obj.southeast !== undefined && isNaN(obj.southeast as number)) obj.southeast = "0";
    }
    {
        const southStr = formData.get(`${"south"}`);
        obj.south = southStr ? parseFloat(southStr as string) : $MfPh5;
        if (obj.south !== undefined && isNaN(obj.south as number)) obj.south = "0";
    }
    {
        const southwestStr = formData.get(`${"southwest"}`);
        obj.southwest = southwestStr ? parseFloat(southwestStr as string) : $MfPh5;
        if (obj.southwest !== undefined && isNaN(obj.southwest as number)) obj.southwest = "0";
    }
    {
        const westStr = formData.get(`${"west"}`);
        obj.west = westStr ? parseFloat(westStr as string) : $MfPh5;
        if (obj.west !== undefined && isNaN(obj.west as number)) obj.west = "0";
    }
    {
        const northwestStr = formData.get(`${"northwest"}`);
        obj.northwest = northwestStr ? parseFloat(northwestStr as string) : $MfPh5;
        if (obj.northwest !== undefined && isNaN(obj.northwest as number)) obj.northwest = "0";
    }
    return toExit("ordinalDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Ordinal;
    readonly errors: OrdinalErrors;
    readonly tainted: OrdinalTainted;
    readonly fields: OrdinalFieldControllers;
    validate(): Exit<Ordinal, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Ordinal>): void;
}
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
 }; export function ordinalCreateForm(overrides: Partial<Ordinal>): OrdinalGigaform {}
let data = $state({
    ...ordinalDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as OrdinalErrors);
let tainted = $state<$MfPh3>({} as OrdinalTainted);
const fields = {} as OrdinalFieldControllers;
fields.north = {
    label: `${"north"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.northeast = {
    label: `${"northeast"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.east = {
    label: `${"east"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.southeast = {
    label: `${"southeast"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.south = {
    label: `${"south"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.southwest = {
    label: `${"southwest"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.west = {
    label: `${"west"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.northwest = {
    label: `${"northwest"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
function validate(): Exit<Ordinal, Array<{
    field: string;
    message: string;
}>> {
    return toExit("ordinalDeserialize(data)");
    data = {
        ...ordinalDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Ordinal = {
  defaultValue: ordinalDefaultValue,
  serialize: ordinalSerialize,
  serializeWithContext: ordinalSerializeWithContext,
  deserialize: ordinalDeserialize,
  deserializeWithContext: ordinalDeserializeWithContext,
  validateFields: ordinalValidateFields,
  hasShape: ordinalHasShape,
  is: ordinalIs,
  fromFormData: ordinalFromFormData,
  createForm: ordinalCreateForm
} as const;