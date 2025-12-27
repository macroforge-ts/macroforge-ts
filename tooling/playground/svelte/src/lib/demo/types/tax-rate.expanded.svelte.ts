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


export interface TaxRate {
    
    id: string;
    
    
    name: string;
    
    
    taxAgency: string;
    
    zip: number;
    
    
    city: string;
    
    
    county: string;
    
    
    state: string;
    
    isActive: boolean;
    
    
    description: string;
    
    
    taxComponents: { [key: string]: number };
}

export function taxRateDefaultValue(): TaxRate {
    return {
        id: "",
        name: "",
        taxAgency: "",
        zip: 0,
        city: "",
        county: "",
        state: "",
        isActive: false,
        description: "",
        taxComponents: {}
    } as TaxRate;
}

export function taxRateSerialize(value: TaxRate): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(taxRateSerializeWithContext(value, ctx));
}
export function taxRateSerializeWithContext(value: TaxRate, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"TaxRate"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"name"}`] = value.name;
    result[`${"taxAgency"}`] = value.taxAgency;
    result[`${"zip"}`] = value.zip;
    result[`${"city"}`] = value.city;
    result[`${"county"}`] = value.county;
    result[`${"state"}`] = value.state;
    result[`${"isActive"}`] = value.isActive;
    result[`${"description"}`] = value.description;
    result[`${"taxComponents"}`] = value.taxComponents;
    return result;
}

export function taxRateDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: TaxRate } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = taxRateDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "TaxRate.deserialize: root cannot be a forward reference"
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
export function taxRateDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): TaxRate | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"TaxRate"}.deserializeWithContext: expected an object`
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
    if (!(`${"name"}` in obj)) {
        errors.push({
            field: `${"name"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxAgency"}` in obj)) {
        errors.push({
            field: `${"taxAgency"}`,
            message: "missing required field"
        });
    }
    if (!(`${"zip"}` in obj)) {
        errors.push({
            field: `${"zip"}`,
            message: "missing required field"
        });
    }
    if (!(`${"city"}` in obj)) {
        errors.push({
            field: `${"city"}`,
            message: "missing required field"
        });
    }
    if (!(`${"county"}` in obj)) {
        errors.push({
            field: `${"county"}`,
            message: "missing required field"
        });
    }
    if (!(`${"state"}` in obj)) {
        errors.push({
            field: `${"state"}`,
            message: "missing required field"
        });
    }
    if (!(`${"isActive"}` in obj)) {
        errors.push({
            field: `${"isActive"}`,
            message: "missing required field"
        });
    }
    if (!(`${"description"}` in obj)) {
        errors.push({
            field: `${"description"}`,
            message: "missing required field"
        });
    }
    if (!(`${"taxComponents"}` in obj)) {
        errors.push({
            field: `${"taxComponents"}`,
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
        const __raw_name = obj[`${"name"}`] as string;
        if (__raw_name.trim().length === 0) {
            errors.push({
                field: "name",
                message: "TaxRate.name must not be empty"
            });
        }
        instance.name = __raw_name;
    }
    {
        const __raw_taxAgency = obj[`${"taxAgency"}`] as string;
        if (__raw_taxAgency.trim().length === 0) {
            errors.push({
                field: "taxAgency",
                message: "TaxRate.taxAgency must not be empty"
            });
        }
        instance.taxAgency = __raw_taxAgency;
    }
    {
        const __raw_zip = obj[`${"zip"}`] as number;
        instance.zip = __raw_zip;
    }
    {
        const __raw_city = obj[`${"city"}`] as string;
        if (__raw_city.trim().length === 0) {
            errors.push({
                field: "city",
                message: "TaxRate.city must not be empty"
            });
        }
        instance.city = __raw_city;
    }
    {
        const __raw_county = obj[`${"county"}`] as string;
        if (__raw_county.trim().length === 0) {
            errors.push({
                field: "county",
                message: "TaxRate.county must not be empty"
            });
        }
        instance.county = __raw_county;
    }
    {
        const __raw_state = obj[`${"state"}`] as string;
        if (__raw_state.trim().length === 0) {
            errors.push({
                field: "state",
                message: "TaxRate.state must not be empty"
            });
        }
        instance.state = __raw_state;
    }
    {
        const __raw_isActive = obj[`${"isActive"}`] as boolean;
        instance.isActive = __raw_isActive;
    }
    {
        const __raw_description = obj[`${"description"}`] as string;
        if (__raw_description.trim().length === 0) {
            errors.push({
                field: "description",
                message: "TaxRate.description must not be empty"
            });
        }
        instance.description = __raw_description;
    }
    {
        const __raw_taxComponents = obj[`${"taxComponents"}`] as { [key: string]: number };
        instance.taxComponents = __raw_taxComponents;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as TaxRate;
}
export function taxRateValidateField<K extends keyof TaxRate>(_field: K, _value: TaxRate[K]): Array<{
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
                message: "TaxRate.name must not be empty"
            });
        }
    }
    if (_field === `${"taxAgency"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "taxAgency",
                message: "TaxRate.taxAgency must not be empty"
            });
        }
    }
    if (_field === `${"city"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "TaxRate.city must not be empty"
            });
        }
    }
    if (_field === `${"county"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "county",
                message: "TaxRate.county must not be empty"
            });
        }
    }
    if (_field === `${"state"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "TaxRate.state must not be empty"
            });
        }
    }
    if (_field === `${"description"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "TaxRate.description must not be empty"
            });
        }
    }
    return errors;
}
export function taxRateValidateFields(_partial: Partial<TaxRate>): Array<{
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
                message: "TaxRate.name must not be empty"
            });
        }
    }
    if (`${"taxAgency"}` in _partial && _partial.taxAgency !== undefined) {
        const __val = _partial.taxAgency as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "taxAgency",
                message: "TaxRate.taxAgency must not be empty"
            });
        }
    }
    if (`${"city"}` in _partial && _partial.city !== undefined) {
        const __val = _partial.city as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "city",
                message: "TaxRate.city must not be empty"
            });
        }
    }
    if (`${"county"}` in _partial && _partial.county !== undefined) {
        const __val = _partial.county as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "county",
                message: "TaxRate.county must not be empty"
            });
        }
    }
    if (`${"state"}` in _partial && _partial.state !== undefined) {
        const __val = _partial.state as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "state",
                message: "TaxRate.state must not be empty"
            });
        }
    }
    if (`${"description"}` in _partial && _partial.description !== undefined) {
        const __val = _partial.description as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "description",
                message: "TaxRate.description must not be empty"
            });
        }
    }
    return errors;
}
export function taxRateHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "name" in o && "taxAgency" in o && "zip" in o && "city" in o && "county" in o && "state" in o && "isActive" in o && "description" in o && "taxComponents" in o';
}
export function taxRateIs(obj: unknown): obj is TaxRate {
    if (!taxRateHasShape(obj)) {
        return false;
    }
    const result = taxRateDeserialize(obj);
    return result.success;
}

export function taxRateFromFormData(formData: FormData): Exit<TaxRate, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.name = formData.get(`${"name"}`) ?? "";
    obj.taxAgency = formData.get(`${"taxAgency"}`) ?? "";
    {
        const zipStr = formData.get(`${"zip"}`);
        obj.zip = zipStr ? parseFloat(zipStr as string) : $MfPh5;
        if (obj.zip !== undefined && isNaN(obj.zip as number)) obj.zip = "0";
    }
    obj.city = formData.get(`${"city"}`) ?? "";
    obj.county = formData.get(`${"county"}`) ?? "";
    obj.state = formData.get(`${"state"}`) ?? "";
    {
        const isActiveVal = formData.get(`${"isActive"}`);
        obj.isActive = isActiveVal === "true" || isActiveVal === "on" || isActiveVal === "1";
    }
    obj.description = formData.get(`${"description"}`) ?? "";
    obj.taxComponents = formData.get(`${"taxComponents"}`) ?? "";
    return toExit("taxRateDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: TaxRate;
    readonly errors: TaxRateErrors;
    readonly tainted: TaxRateTainted;
    readonly fields: TaxRateFieldControllers;
    validate(): Exit<TaxRate, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<TaxRate>): void;
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
 }; export function taxRateCreateForm(overrides: Partial<TaxRate>): TaxRateGigaform {}
let data = $state({
    ...taxRateDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as TaxRateErrors);
let tainted = $state<$MfPh3>({} as TaxRateTainted);
const fields = {} as TaxRateFieldControllers;
fields.id = {
    label: `${"id"}`,
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
fields.taxAgency = {
    label: `${"taxAgency"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.zip = {
    label: `${"zip"}`,
    type: `${"number"}`,
    optional: false,
    array: false
};
fields.city = {
    label: `${"city"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.county = {
    label: `${"county"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.state = {
    label: `${"state"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.isActive = {
    label: `${"isActive"}`,
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
fields.taxComponents = {
    label: `${"taxComponents"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<TaxRate, Array<{
    field: string;
    message: string;
}>> {
    return toExit("taxRateDeserialize(data)");
    data = {
        ...taxRateDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const TaxRate = {
  defaultValue: taxRateDefaultValue,
  serialize: taxRateSerialize,
  serializeWithContext: taxRateSerializeWithContext,
  deserialize: taxRateDeserialize,
  deserializeWithContext: taxRateDeserializeWithContext,
  validateFields: taxRateValidateFields,
  hasShape: taxRateHasShape,
  is: taxRateIs,
  fromFormData: taxRateFromFormData,
  createForm: taxRateCreateForm
} as const;