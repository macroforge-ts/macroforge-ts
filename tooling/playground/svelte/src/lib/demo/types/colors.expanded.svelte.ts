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


export interface Colors {
    
    main: string;
    
    hover: string;
    
    active: string;
}

export function colorsDefaultValue(): Colors {
    return {
        main: "",
        hover: "",
        active: ""
    } as Colors;
}

export function colorsSerialize(value: Colors): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(colorsSerializeWithContext(value, ctx));
}
export function colorsSerializeWithContext(value: Colors, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Colors"}`,
        __id
    };
    result[`${"main"}`] = value.main;
    result[`${"hover"}`] = value.hover;
    result[`${"active"}`] = value.active;
    return result;
}

export function colorsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Colors } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = colorsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Colors.deserialize: root cannot be a forward reference"
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
export function colorsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Colors | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Colors"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"main"}` in obj)) {
        errors.push({
            field: `${"main"}`,
            message: "missing required field"
        });
    }
    if (!(`${"hover"}` in obj)) {
        errors.push({
            field: `${"hover"}`,
            message: "missing required field"
        });
    }
    if (!(`${"active"}` in obj)) {
        errors.push({
            field: `${"active"}`,
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
        const __raw_main = obj[`${"main"}`] as string;
        if (__raw_main.trim().length === 0) {
            errors.push({
                field: "main",
                message: "Colors.main must not be empty"
            });
        }
        instance.main = __raw_main;
    }
    {
        const __raw_hover = obj[`${"hover"}`] as string;
        if (__raw_hover.trim().length === 0) {
            errors.push({
                field: "hover",
                message: "Colors.hover must not be empty"
            });
        }
        instance.hover = __raw_hover;
    }
    {
        const __raw_active = obj[`${"active"}`] as string;
        if (__raw_active.trim().length === 0) {
            errors.push({
                field: "active",
                message: "Colors.active must not be empty"
            });
        }
        instance.active = __raw_active;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Colors;
}
export function colorsValidateField<K extends keyof Colors>(_field: K, _value: Colors[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"main"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "main",
                message: "Colors.main must not be empty"
            });
        }
    }
    if (_field === `${"hover"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "hover",
                message: "Colors.hover must not be empty"
            });
        }
    }
    if (_field === `${"active"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "active",
                message: "Colors.active must not be empty"
            });
        }
    }
    return errors;
}
export function colorsValidateFields(_partial: Partial<Colors>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"main"}` in _partial && _partial.main !== undefined) {
        const __val = _partial.main as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "main",
                message: "Colors.main must not be empty"
            });
        }
    }
    if (`${"hover"}` in _partial && _partial.hover !== undefined) {
        const __val = _partial.hover as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "hover",
                message: "Colors.hover must not be empty"
            });
        }
    }
    if (`${"active"}` in _partial && _partial.active !== undefined) {
        const __val = _partial.active as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "active",
                message: "Colors.active must not be empty"
            });
        }
    }
    return errors;
}
export function colorsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"main" in o && "hover" in o && "active" in o';
}
export function colorsIs(obj: unknown): obj is Colors {
    if (!colorsHasShape(obj)) {
        return false;
    }
    const result = colorsDeserialize(obj);
    return result.success;
}

export function colorsFromFormData(formData: FormData): Exit<Colors, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.main = formData.get(`${"main"}`) ?? "";
    obj.hover = formData.get(`${"hover"}`) ?? "";
    obj.active = formData.get(`${"active"}`) ?? "";
    return toExit("colorsDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Colors;
    readonly errors: ColorsErrors;
    readonly tainted: ColorsTainted;
    readonly fields: ColorsFieldControllers;
    validate(): Exit<Colors, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Colors>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function colorsCreateForm(overrides: Partial<Colors>): ColorsGigaform {}
let data = $state({
    ...colorsDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as ColorsErrors);
let tainted = $state<$MfPh3>({} as ColorsTainted);
const fields = {} as ColorsFieldControllers;
fields.main = {
    label: `${"main"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.hover = {
    label: `${"hover"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.active = {
    label: `${"active"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Colors, Array<{
    field: string;
    message: string;
}>> {
    return toExit("colorsDeserialize(data)");
    data = {
        ...colorsDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Colors = {
  defaultValue: colorsDefaultValue,
  serialize: colorsSerialize,
  serializeWithContext: colorsSerializeWithContext,
  deserialize: colorsDeserialize,
  deserializeWithContext: colorsDeserializeWithContext,
  validateFields: colorsValidateFields,
  hasShape: colorsHasShape,
  is: colorsIs,
  fromFormData: colorsFromFormData,
  createForm: colorsCreateForm
} as const;