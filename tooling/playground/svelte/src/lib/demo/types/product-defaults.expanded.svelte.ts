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


export interface ProductDefaults {
    
    price: number;
    
    
    description: string;
}

export function productDefaultsDefaultValue(): ProductDefaults {
    return {
        price: 0,
        description: ""
    } as ProductDefaults;
}

export function productDefaultsSerialize(value: ProductDefaults): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(productDefaultsSerializeWithContext(value, ctx));
}
export function productDefaultsSerializeWithContext(value: ProductDefaults, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"ProductDefaults"}`,
        __id
    };
    result[`${"price"}`] = value.price;
    result[`${"description"}`] = value.description;
    return result;
}

export function productDefaultsDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: ProductDefaults } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = productDefaultsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "ProductDefaults.deserialize: root cannot be a forward reference"
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
export function productDefaultsDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): ProductDefaults | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"ProductDefaults"}.deserializeWithContext: expected an object`
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!(`${"price"}` in obj)) {
        errors.push({
            field: `${"price"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
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
        const __raw_price = obj[`${"price"}`] as number;
        instance.price = __raw_price;
    }
    {
        const __raw_description = obj[`${"description"}`] as string;
        if (__raw_description.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ProductDefaults.description must not be empty"
            });
        }
        instance.description = __raw_description;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ProductDefaults;
}
export function productDefaultsValidateField<K extends keyof ProductDefaults>(_field: K, _value: ProductDefaults[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"description"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ProductDefaults.description must not be empty"
            });
        }
    }
    return errors;
}
export function productDefaultsValidateFields(_partial: Partial<ProductDefaults>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"description"}` in _partial && _partial.description !== undefined) {
        const __val = _partial.description as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "ProductDefaults.description must not be empty"
            });
        }
    }
    return errors;
}
export function productDefaultsHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"price" in o && "description" in o';
}
export function productDefaultsIs(obj: unknown): obj is ProductDefaults {
    if (!productDefaultsHasShape(obj)) {
        return false;
    }
    const result = productDefaultsDeserialize(obj);
    return result.success;
}

export function productDefaultsFromFormData(formData: FormData): Exit<ProductDefaults, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    {
        const priceStr = formData.get(`${"price"}`);
        obj.price = priceStr ? parseFloat(priceStr as string) : $MfPh5;
        if (obj.price !== undefined && isNaN(obj.price as number)) obj.price = "0";
    }
    obj.description = formData.get(`${"description"}`) ?? "";
    return toExit("productDefaultsDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: ProductDefaults;
    readonly errors: ProductDefaultsErrors;
    readonly tainted: ProductDefaultsTainted;
    readonly fields: ProductDefaultsFieldControllers;
    validate(): Exit<ProductDefaults, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<ProductDefaults>): void;
}
$MfPh0: __gf_Option<Array<string>>;
$MfPh0: __gf_Option<Array<string>>;
 }; $MfPh0: __gf_Option<boolean>;
$MfPh0: __gf_Option<boolean>;
 }; export function productDefaultsCreateForm(overrides: Partial<ProductDefaults>): ProductDefaultsGigaform {}
let data = $state({
    ...productDefaultsDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as ProductDefaultsErrors);
let tainted = $state<$MfPh3>({} as ProductDefaultsTainted);
const fields = {} as ProductDefaultsFieldControllers;
fields.price = {
    label: `${"price"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.description = {
    label: `${"description"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<ProductDefaults, Array<{
    field: string;
    message: string;
}>> {
    return toExit("productDefaultsDeserialize(data)");
    data = {
        ...productDefaultsDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const ProductDefaults = {
  defaultValue: productDefaultsDefaultValue,
  serialize: productDefaultsSerialize,
  serializeWithContext: productDefaultsSerializeWithContext,
  deserialize: productDefaultsDeserialize,
  deserializeWithContext: productDefaultsDeserializeWithContext,
  validateFields: productDefaultsValidateFields,
  hasShape: productDefaultsHasShape,
  is: productDefaultsIs,
  fromFormData: productDefaultsFromFormData,
  createForm: productDefaultsCreateForm
} as const;