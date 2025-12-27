import { coordinatesDefaultValue } from "./coordinates.svelte";
import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { coordinatesSerializeWithContext } from "./coordinates.svelte";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import { coordinatesDeserializeWithContext } from "./coordinates.svelte";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { Coordinates } from './coordinates.svelte';


export interface Site {
    id: string;
    
    addressLine1: string;
    addressLine2: string | null;
    sublocalityLevel1: string | null;
    
    locality: string;
    administrativeAreaLevel3: string | null;
    administrativeAreaLevel2: string | null;
    
    administrativeAreaLevel1: string;
    
    country: string;
    
    postalCode: string;
    postalCodeSuffix: string | null;
    coordinates: Coordinates;
}

export function siteDefaultValue(): Site {
    return {
        id: "",
        addressLine1: "",
        addressLine2: null,
        sublocalityLevel1: null,
        locality: "",
        administrativeAreaLevel3: null,
        administrativeAreaLevel2: null,
        administrativeAreaLevel1: "",
        country: "",
        postalCode: "",
        postalCodeSuffix: null,
        coordinates: coordinatesDefaultValue()
    } as Site;
}

export function siteSerialize(value: Site): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(siteSerializeWithContext(value, ctx));
}
export function siteSerializeWithContext(value: Site, ctx: __mf_SerializeContext): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return {
            __ref: existingId
        };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = {
        __type: `${"Site"}`,
        __id
    };
    result[`${"id"}`] = value.id;
    result[`${"addressLine1"}`] = value.addressLine1;
    result[`${"addressLine2"}`] = value.addressLine2;
    result[`${"sublocalityLevel1"}`] = value.sublocalityLevel1;
    result[`${"locality"}`] = value.locality;
    result[`${"administrativeAreaLevel3"}`] = value.administrativeAreaLevel3;
    result[`${"administrativeAreaLevel2"}`] = value.administrativeAreaLevel2;
    result[`${"administrativeAreaLevel1"}`] = value.administrativeAreaLevel1;
    result[`${"country"}`] = value.country;
    result[`${"postalCode"}`] = value.postalCode;
    result[`${"postalCodeSuffix"}`] = value.postalCodeSuffix;
    result[`${"coordinates"}`] = coordinatesSerializeWithContext(value.coordinates, ctx);
    return result;
}

export function siteDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Site } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = siteDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Site.deserialize: root cannot be a forward reference"
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
export function siteDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Site | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `${"Site"}.deserializeWithContext: expected an object`
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
    if (!(`${"addressLine1"}` in obj)) {
        errors.push({
            field: `${"addressLine1"}`,
            message: "missing required field"
        });
    }
    if (!(`${"addressLine2"}` in obj)) {
        errors.push({
            field: `${"addressLine2"}`,
            message: "missing required field"
        });
    }
    if (!(`${"sublocalityLevel1"}` in obj)) {
        errors.push({
            field: `${"sublocalityLevel1"}`,
            message: "missing required field"
        });
    }
    if (!(`${"locality"}` in obj)) {
        errors.push({
            field: `${"locality"}`,
            message: "missing required field"
        });
    }
    if (!(`${"administrativeAreaLevel3"}` in obj)) {
        errors.push({
            field: `${"administrativeAreaLevel3"}`,
            message: "missing required field"
        });
    }
    if (!(`${"administrativeAreaLevel2"}` in obj)) {
        errors.push({
            field: `${"administrativeAreaLevel2"}`,
            message: "missing required field"
        });
    }
    if (!(`${"administrativeAreaLevel1"}` in obj)) {
        errors.push({
            field: `${"administrativeAreaLevel1"}`,
            message: "missing required field"
        });
    }
    if (!(`${"country"}` in obj)) {
        errors.push({
            field: `${"country"}`,
            message: "missing required field"
        });
    }
    if (!(`${"postalCode"}` in obj)) {
        errors.push({
            field: `${"postalCode"}`,
            message: "missing required field"
        });
    }
    if (!(`${"postalCodeSuffix"}` in obj)) {
        errors.push({
            field: `${"postalCodeSuffix"}`,
            message: "missing required field"
        });
    }
    if (!(`${"coordinates"}` in obj)) {
        errors.push({
            field: `${"coordinates"}`,
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
        const __raw_addressLine1 = obj[`${"addressLine1"}`] as string;
        if (__raw_addressLine1.trim().length === 0) {
            errors.push({
                field: "addressLine1",
                message: "Site.addressLine1 must not be empty"
            });
        }
        instance.addressLine1 = __raw_addressLine1;
    }
    {
        const __raw_addressLine2 = obj[`${"addressLine2"}`] as string | null;
        instance.addressLine2 = __raw_addressLine2;
    }
    {
        const __raw_sublocalityLevel1 = obj[`${"sublocalityLevel1"}`] as string | null;
        instance.sublocalityLevel1 = __raw_sublocalityLevel1;
    }
    {
        const __raw_locality = obj[`${"locality"}`] as string;
        if (__raw_locality.trim().length === 0) {
            errors.push({
                field: "locality",
                message: "Site.locality must not be empty"
            });
        }
        instance.locality = __raw_locality;
    }
    {
        const __raw_administrativeAreaLevel3 = obj[`${"administrativeAreaLevel3"}`] as string | null;
        instance.administrativeAreaLevel3 = __raw_administrativeAreaLevel3;
    }
    {
        const __raw_administrativeAreaLevel2 = obj[`${"administrativeAreaLevel2"}`] as string | null;
        instance.administrativeAreaLevel2 = __raw_administrativeAreaLevel2;
    }
    {
        const __raw_administrativeAreaLevel1 = obj[`${"administrativeAreaLevel1"}`] as string;
        if (__raw_administrativeAreaLevel1.trim().length === 0) {
            errors.push({
                field: "administrativeAreaLevel1",
                message: "Site.administrativeAreaLevel1 must not be empty"
            });
        }
        instance.administrativeAreaLevel1 = __raw_administrativeAreaLevel1;
    }
    {
        const __raw_country = obj[`${"country"}`] as string;
        if (__raw_country.trim().length === 0) {
            errors.push({
                field: "country",
                message: "Site.country must not be empty"
            });
        }
        instance.country = __raw_country;
    }
    {
        const __raw_postalCode = obj[`${"postalCode"}`] as string;
        if (__raw_postalCode.trim().length === 0) {
            errors.push({
                field: "postalCode",
                message: "Site.postalCode must not be empty"
            });
        }
        instance.postalCode = __raw_postalCode;
    }
    {
        const __raw_postalCodeSuffix = obj[`${"postalCodeSuffix"}`] as string | null;
        instance.postalCodeSuffix = __raw_postalCodeSuffix;
    }
    {
        const __raw_coordinates = obj[`${"coordinates"}`] as Coordinates;
        {
            const __result = coordinatesDeserializeWithContext(__raw_coordinates, ctx);
            ctx.assignOrDefer(instance, `${"coordinates"}`, __result);
        }
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as Site;
}
export function siteValidateField<K extends keyof Site>(_field: K, _value: Site[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (_field === `${"addressLine1"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "addressLine1",
                message: "Site.addressLine1 must not be empty"
            });
        }
    }
    if (_field === `${"locality"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "locality",
                message: "Site.locality must not be empty"
            });
        }
    }
    if (_field === `${"administrativeAreaLevel1"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "administrativeAreaLevel1",
                message: "Site.administrativeAreaLevel1 must not be empty"
            });
        }
    }
    if (_field === `${"country"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "country",
                message: "Site.country must not be empty"
            });
        }
    }
    if (_field === `${"postalCode"}`) {
        const __val = _value as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "postalCode",
                message: "Site.postalCode must not be empty"
            });
        }
    }
    return errors;
}
export function siteValidateFields(_partial: Partial<Site>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (`${"addressLine1"}` in _partial && _partial.addressLine1 !== undefined) {
        const __val = _partial.addressLine1 as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "addressLine1",
                message: "Site.addressLine1 must not be empty"
            });
        }
    }
    if (`${"locality"}` in _partial && _partial.locality !== undefined) {
        const __val = _partial.locality as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "locality",
                message: "Site.locality must not be empty"
            });
        }
    }
    if (`${"administrativeAreaLevel1"}` in _partial && _partial.administrativeAreaLevel1 !== undefined) {
        const __val = _partial.administrativeAreaLevel1 as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "administrativeAreaLevel1",
                message: "Site.administrativeAreaLevel1 must not be empty"
            });
        }
    }
    if (`${"country"}` in _partial && _partial.country !== undefined) {
        const __val = _partial.country as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "country",
                message: "Site.country must not be empty"
            });
        }
    }
    if (`${"postalCode"}` in _partial && _partial.postalCode !== undefined) {
        const __val = _partial.postalCode as string;
        if (__val.trim().length === 0) {
            errors.push({
                field: "postalCode",
                message: "Site.postalCode must not be empty"
            });
        }
    }
    return errors;
}
export function siteHasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return '"id" in o && "addressLine1" in o && "addressLine2" in o && "sublocalityLevel1" in o && "locality" in o && "administrativeAreaLevel3" in o && "administrativeAreaLevel2" in o && "administrativeAreaLevel1" in o && "country" in o && "postalCode" in o && "postalCodeSuffix" in o && "coordinates" in o';
}
export function siteIs(obj: unknown): obj is Site {
    if (!siteHasShape(obj)) {
        return false;
    }
    const result = siteDeserialize(obj);
    return result.success;
}

export function siteFromFormData(formData: FormData): Exit<Site, Array<{
    field: string;
    message: string;
}>> {
    const obj: Record<string, unknown> = {};
    obj.id = formData.get(`${"id"}`) ?? "";
    obj.addressLine1 = formData.get(`${"addressLine1"}`) ?? "";
    obj.addressLine2 = formData.get(`${"addressLine2"}`) ?? "";
    obj.sublocalityLevel1 = formData.get(`${"sublocalityLevel1"}`) ?? "";
    obj.locality = formData.get(`${"locality"}`) ?? "";
    obj.administrativeAreaLevel3 = formData.get(`${"administrativeAreaLevel3"}`) ?? "";
    obj.administrativeAreaLevel2 = formData.get(`${"administrativeAreaLevel2"}`) ?? "";
    obj.administrativeAreaLevel1 = formData.get(`${"administrativeAreaLevel1"}`) ?? "";
    obj.country = formData.get(`${"country"}`) ?? "";
    obj.postalCode = formData.get(`${"postalCode"}`) ?? "";
    obj.postalCodeSuffix = formData.get(`${"postalCodeSuffix"}`) ?? "";
    {
        const coordinatesObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())){
            if (key.startsWith(`${"coordinates"}.`)) {
                const fieldName = key.slice(`${"coordinates"}.`.length);
                const parts = fieldName.split(".");
                let current = coordinatesObj;
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
        obj.coordinates = coordinatesObj;
    }
    return toExit("siteDeserialize(obj)");
}
export type $MfPh0 = {
    _errors: __gf_Option<Array<string>>;
};
export type $MfPh1 = {
};
export interface $MfPh2 {
}
export interface $MfPh3 {
    readonly data: Site;
    readonly errors: SiteErrors;
    readonly tainted: SiteTainted;
    readonly fields: SiteFieldControllers;
    validate(): Exit<Site, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides: Partial<Site>): void;
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
 }; export function siteCreateForm(overrides: Partial<Site>): SiteGigaform {}
let data = $state({
    ...siteDefaultValue(),
    ...overrides
});
let errors = $state<$MfPh1>({
    _errors: optionNone()
} as SiteErrors);
let tainted = $state<$MfPh3>({} as SiteTainted);
const fields = {} as SiteFieldControllers;
fields.id = {
    label: `${"id"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.addressLine1 = {
    label: `${"addressLine1"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.addressLine2 = {
    label: `${"addressLine2"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.sublocalityLevel1 = {
    label: `${"sublocalityLevel1"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.locality = {
    label: `${"locality"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.administrativeAreaLevel3 = {
    label: `${"administrativeAreaLevel3"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.administrativeAreaLevel2 = {
    label: `${"administrativeAreaLevel2"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.administrativeAreaLevel1 = {
    label: `${"administrativeAreaLevel1"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.country = {
    label: `${"country"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.postalCode = {
    label: `${"postalCode"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.postalCodeSuffix = {
    label: `${"postalCodeSuffix"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
fields.coordinates = {
    label: `${"coordinates"}`,
    type: `${"text"}`,
    optional: false,
    array: false
};
function validate(): Exit<Site, Array<{
    field: string;
    message: string;
}>> {
    return toExit("siteDeserialize(data)");
    data = {
        ...siteDefaultValue(),
        ...newOverrides
    };
}
 return     {         get data() { return data; }, set data(v) { data = v; }, get errors()         { return errors; }, set errors(v) { errors = v; }, get tainted()         { return tainted; }, set tainted(v) { tainted = v; }, fields,         validate, reset,     }; }

export const Site = {
  defaultValue: siteDefaultValue,
  serialize: siteSerialize,
  serializeWithContext: siteSerializeWithContext,
  deserialize: siteDeserialize,
  deserializeWithContext: siteDeserializeWithContext,
  validateFields: siteValidateFields,
  hasShape: siteHasShape,
  is: siteIs,
  fromFormData: siteFromFormData,
  createForm: siteCreateForm
} as const;