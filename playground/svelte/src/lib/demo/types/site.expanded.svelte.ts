import { coordinatesDefaultValue } from "./coordinates.svelte";
import { SerializeContext } from "macroforge/serde";
import { coordinatesSerializeWithContext } from "./coordinates.svelte";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
import { coordinatesDeserializeWithContext } from "./coordinates.svelte";
import { Result } from "macroforge/utils";
import { Option } from "macroforge/utils";
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

export function siteDefaultValue(): Site {return {id: "",
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
                            coordinates: coordinatesDefaultValue(), }as Site;}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */export function siteSerialize(value: Site): string {const ctx = SerializeContext.create(); return JSON.stringify(siteSerializeWithContext(value, ctx));}/** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */export function siteSerializeWithContext(value: Site, ctx: SerializeContext): Record<string, unknown>{const existingId = ctx.getId(value); if(existingId!== undefined){return {__ref: existingId};}const __id = ctx.register(value); const result: Record<string, unknown>= {__type: "Site" , __id,}; result["id" ]= value.id; result["addressLine1" ]= value.addressLine1; result["addressLine2" ]= value.addressLine2; result["sublocalityLevel1" ]= value.sublocalityLevel1; result["locality" ]= value.locality; result["administrativeAreaLevel3" ]= value.administrativeAreaLevel3; result["administrativeAreaLevel2" ]= value.administrativeAreaLevel2; result["administrativeAreaLevel1" ]= value.administrativeAreaLevel1; result["country" ]= value.country; result["postalCode" ]= value.postalCode; result["postalCodeSuffix" ]= value.postalCodeSuffix; result["coordinates" ]= coordinatesSerializeWithContext(value.coordinates, ctx); return result;}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */export function siteDeserialize(input: unknown, opts?: DeserializeOptions): { success: true; value: Site } | { success: false; errors: Array<{ field: string; message: string }> } {try {const data = typeof input === "string" ? JSON.parse(input): input; const ctx = DeserializeContext.create(); const resultOrRef = siteDeserializeWithContext(data, ctx); if(PendingRef.is(resultOrRef)){return { success: false, errors: [{ field: "_root", message: "Site.deserialize: root cannot be a forward reference" }] };}ctx.applyPatches(); if(opts?.freeze){ctx.freezeAll();}return { success: true, value: resultOrRef };}catch(e){if(e instanceof DeserializeError){return { success: false, errors: e.errors };}const message = e instanceof Error? e.message: String(e); return { success: false, errors: [{ field: "_root", message }] };}}/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */export function siteDeserializeWithContext(value: any, ctx: DeserializeContext): Site | PendingRef {if(value?.__ref!== undefined){return ctx.getOrDefer(value.__ref);}if(typeof value!== "object" || value === null || Array.isArray(value)){throw new DeserializeError([{field: "_root" , message: "Site.deserializeWithContext: expected an object" }]);}const obj = value as Record<string, unknown>; const errors: Array<{field: string; message: string}>= []; if(!("id" in obj)){errors.push({field: "id" , message: "missing required field" });}if(!("addressLine1" in obj)){errors.push({field: "addressLine1" , message: "missing required field" });}if(!("addressLine2" in obj)){errors.push({field: "addressLine2" , message: "missing required field" });}if(!("sublocalityLevel1" in obj)){errors.push({field: "sublocalityLevel1" , message: "missing required field" });}if(!("locality" in obj)){errors.push({field: "locality" , message: "missing required field" });}if(!("administrativeAreaLevel3" in obj)){errors.push({field: "administrativeAreaLevel3" , message: "missing required field" });}if(!("administrativeAreaLevel2" in obj)){errors.push({field: "administrativeAreaLevel2" , message: "missing required field" });}if(!("administrativeAreaLevel1" in obj)){errors.push({field: "administrativeAreaLevel1" , message: "missing required field" });}if(!("country" in obj)){errors.push({field: "country" , message: "missing required field" });}if(!("postalCode" in obj)){errors.push({field: "postalCode" , message: "missing required field" });}if(!("postalCodeSuffix" in obj)){errors.push({field: "postalCodeSuffix" , message: "missing required field" });}if(!("coordinates" in obj)){errors.push({field: "coordinates" , message: "missing required field" });}if(errors.length>0){throw new DeserializeError(errors);}const instance: any = {}; if(obj.__id!== undefined){ctx.register(obj.__id as number, instance);}ctx.trackForFreeze(instance); {const __raw_id = obj["id" ]as string; instance.id = __raw_id; }{const __raw_addressLine1 = obj["addressLine1" ]as string; 
                if (__raw_addressLine1.length === 0) {
                    errors.push({ field: "addressLine1", message: "must not be empty" });
                }
 instance.addressLine1 = __raw_addressLine1; }{const __raw_addressLine2 = obj["addressLine2" ]as string | null; instance.addressLine2 = __raw_addressLine2; }{const __raw_sublocalityLevel1 = obj["sublocalityLevel1" ]as string | null; instance.sublocalityLevel1 = __raw_sublocalityLevel1; }{const __raw_locality = obj["locality" ]as string; 
                if (__raw_locality.length === 0) {
                    errors.push({ field: "locality", message: "must not be empty" });
                }
 instance.locality = __raw_locality; }{const __raw_administrativeAreaLevel3 = obj["administrativeAreaLevel3" ]as string | null; instance.administrativeAreaLevel3 = __raw_administrativeAreaLevel3; }{const __raw_administrativeAreaLevel2 = obj["administrativeAreaLevel2" ]as string | null; instance.administrativeAreaLevel2 = __raw_administrativeAreaLevel2; }{const __raw_administrativeAreaLevel1 = obj["administrativeAreaLevel1" ]as string; 
                if (__raw_administrativeAreaLevel1.length === 0) {
                    errors.push({ field: "administrativeAreaLevel1", message: "must not be empty" });
                }
 instance.administrativeAreaLevel1 = __raw_administrativeAreaLevel1; }{const __raw_country = obj["country" ]as string; 
                if (__raw_country.length === 0) {
                    errors.push({ field: "country", message: "must not be empty" });
                }
 instance.country = __raw_country; }{const __raw_postalCode = obj["postalCode" ]as string; 
                if (__raw_postalCode.length === 0) {
                    errors.push({ field: "postalCode", message: "must not be empty" });
                }
 instance.postalCode = __raw_postalCode; }{const __raw_postalCodeSuffix = obj["postalCodeSuffix" ]as string | null; instance.postalCodeSuffix = __raw_postalCodeSuffix; }{const __raw_coordinates = obj["coordinates" ]as Coordinates; {const __result = coordinatesDeserializeWithContext(__raw_coordinates, ctx); ctx.assignOrDefer(instance, "coordinates" , __result);}}if(errors.length>0){throw new DeserializeError(errors);}return instance as Site;}export function siteValidateField<K extends keyof Site>(field: K, value: Site[K]): Array<{field: string; message: string}>{const errors: Array<{field: string; message: string}>= []; switch(field){case "addressLine1" : {const __val = value as string; 
                if (__val.length === 0) {
                    errors.push({ field: "addressLine1", message: "must not be empty" });
                }
 break;}case "locality" : {const __val = value as string; 
                if (__val.length === 0) {
                    errors.push({ field: "locality", message: "must not be empty" });
                }
 break;}case "administrativeAreaLevel1" : {const __val = value as string; 
                if (__val.length === 0) {
                    errors.push({ field: "administrativeAreaLevel1", message: "must not be empty" });
                }
 break;}case "country" : {const __val = value as string; 
                if (__val.length === 0) {
                    errors.push({ field: "country", message: "must not be empty" });
                }
 break;}case "postalCode" : {const __val = value as string; 
                if (__val.length === 0) {
                    errors.push({ field: "postalCode", message: "must not be empty" });
                }
 break;}}return errors; }export function siteValidateFields(partial: Partial<Site>): Array<{field: string; message: string}>{const errors: Array<{field: string; message: string}>= []; if("addressLine1" in partial && partial.addressLine1!== undefined){const __val = partial.addressLine1 as string; 
                if (__val.length === 0) {
                    errors.push({ field: "addressLine1", message: "must not be empty" });
                }
}if("locality" in partial && partial.locality!== undefined){const __val = partial.locality as string; 
                if (__val.length === 0) {
                    errors.push({ field: "locality", message: "must not be empty" });
                }
}if("administrativeAreaLevel1" in partial && partial.administrativeAreaLevel1!== undefined){const __val = partial.administrativeAreaLevel1 as string; 
                if (__val.length === 0) {
                    errors.push({ field: "administrativeAreaLevel1", message: "must not be empty" });
                }
}if("country" in partial && partial.country!== undefined){const __val = partial.country as string; 
                if (__val.length === 0) {
                    errors.push({ field: "country", message: "must not be empty" });
                }
}if("postalCode" in partial && partial.postalCode!== undefined){const __val = partial.postalCode as string; 
                if (__val.length === 0) {
                    errors.push({ field: "postalCode", message: "must not be empty" });
                }
}return errors; }export function siteHasShape(obj: unknown): boolean {if(typeof obj!== "object" || obj === null || Array.isArray(obj)){return false;}const o = obj as Record<string, unknown>; return "id" in o && "addressLine1" in o && "addressLine2" in o && "sublocalityLevel1" in o && "locality" in o && "administrativeAreaLevel3" in o && "administrativeAreaLevel2" in o && "administrativeAreaLevel1" in o && "country" in o && "postalCode" in o && "postalCodeSuffix" in o && "coordinates" in o;}export function siteIs(obj: unknown): obj is Site {if(!siteHasShape(obj)){return false;}const result = siteDeserialize(obj); return result.success;}

/** Nested error structure matching the data shape */export type SiteErrors = {_errors: Option<Array<string>>; id: Option<Array<string>>; addressLine1: Option<Array<string>>; addressLine2: Option<Array<string>>; sublocalityLevel1: Option<Array<string>>; locality: Option<Array<string>>; administrativeAreaLevel3: Option<Array<string>>; administrativeAreaLevel2: Option<Array<string>>; administrativeAreaLevel1: Option<Array<string>>; country: Option<Array<string>>; postalCode: Option<Array<string>>; postalCodeSuffix: Option<Array<string>>; coordinates: Option<Array<string>>; }; /** Nested boolean structure for tracking touched/dirty fields */export type SiteTainted = {id: Option<boolean>; addressLine1: Option<boolean>; addressLine2: Option<boolean>; sublocalityLevel1: Option<boolean>; locality: Option<boolean>; administrativeAreaLevel3: Option<boolean>; administrativeAreaLevel2: Option<boolean>; administrativeAreaLevel1: Option<boolean>; country: Option<boolean>; postalCode: Option<boolean>; postalCodeSuffix: Option<boolean>; coordinates: Option<boolean>; }; /** Type-safe field controllers for this form */export interface SiteFieldControllers {readonly id: FieldController<string>; readonly addressLine1: FieldController<string>; readonly addressLine2: FieldController<string | null>; readonly sublocalityLevel1: FieldController<string | null>; readonly locality: FieldController<string>; readonly administrativeAreaLevel3: FieldController<string | null>; readonly administrativeAreaLevel2: FieldController<string | null>; readonly administrativeAreaLevel1: FieldController<string>; readonly country: FieldController<string>; readonly postalCode: FieldController<string>; readonly postalCodeSuffix: FieldController<string | null>; readonly coordinates: FieldController<Coordinates>; }/** Gigaform instance containing reactive state and field controllers */export interface SiteGigaform {readonly data: Site; readonly errors: SiteErrors; readonly tainted: SiteTainted; readonly fields: SiteFieldControllers; validate(): Result<Site, Array<{field: string; message: string}>>; reset(overrides?: Partial<Site>): void;}/** Creates a new Gigaform instance with reactive state and field controllers. */export function siteCreateForm(overrides?: Partial<Site>): SiteGigaform {let data = $state({...siteDefaultValue(),...overrides}); let errors = $state<SiteErrors>({_errors: Option.none(), id: Option.none(), addressLine1: Option.none(), addressLine2: Option.none(), sublocalityLevel1: Option.none(), locality: Option.none(), administrativeAreaLevel3: Option.none(), administrativeAreaLevel2: Option.none(), administrativeAreaLevel1: Option.none(), country: Option.none(), postalCode: Option.none(), postalCodeSuffix: Option.none(), coordinates: Option.none(), }); let tainted = $state<SiteTainted>({id: Option.none(), addressLine1: Option.none(), addressLine2: Option.none(), sublocalityLevel1: Option.none(), locality: Option.none(), administrativeAreaLevel3: Option.none(), administrativeAreaLevel2: Option.none(), administrativeAreaLevel1: Option.none(), country: Option.none(), postalCode: Option.none(), postalCodeSuffix: Option.none(), coordinates: Option.none(), }); const fields: SiteFieldControllers = {id: {path: ["id" ]as const, name: "id" , constraints: { required: true }, get: ()=>data.id, set: (value: string)=>{data.id = value;}, transform: (value: string): string =>value,getError: ()=>errors.id, setError: (value: Option<Array<string>>)=>{errors.id = value;}, getTainted: ()=>tainted.id, setTainted: (value: Option<boolean>)=>{tainted.id = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("id", data.id); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},addressLine1: {path: ["addressLine1" ]as const, name: "addressLine1" , constraints: { required: true }, get: ()=>data.addressLine1, set: (value: string)=>{data.addressLine1 = value;}, transform: (value: string): string =>value,getError: ()=>errors.addressLine1, setError: (value: Option<Array<string>>)=>{errors.addressLine1 = value;}, getTainted: ()=>tainted.addressLine1, setTainted: (value: Option<boolean>)=>{tainted.addressLine1 = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("addressLine1", data.addressLine1); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},addressLine2: {path: ["addressLine2" ]as const, name: "addressLine2" , constraints: { required: true }, get: ()=>data.addressLine2, set: (value: string | null)=>{data.addressLine2 = value;}, transform: (value: string | null): string | null =>value,getError: ()=>errors.addressLine2, setError: (value: Option<Array<string>>)=>{errors.addressLine2 = value;}, getTainted: ()=>tainted.addressLine2, setTainted: (value: Option<boolean>)=>{tainted.addressLine2 = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("addressLine2", data.addressLine2); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},sublocalityLevel1: {path: ["sublocalityLevel1" ]as const, name: "sublocalityLevel1" , constraints: { required: true }, get: ()=>data.sublocalityLevel1, set: (value: string | null)=>{data.sublocalityLevel1 = value;}, transform: (value: string | null): string | null =>value,getError: ()=>errors.sublocalityLevel1, setError: (value: Option<Array<string>>)=>{errors.sublocalityLevel1 = value;}, getTainted: ()=>tainted.sublocalityLevel1, setTainted: (value: Option<boolean>)=>{tainted.sublocalityLevel1 = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("sublocalityLevel1", data.sublocalityLevel1); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},locality: {path: ["locality" ]as const, name: "locality" , constraints: { required: true }, get: ()=>data.locality, set: (value: string)=>{data.locality = value;}, transform: (value: string): string =>value,getError: ()=>errors.locality, setError: (value: Option<Array<string>>)=>{errors.locality = value;}, getTainted: ()=>tainted.locality, setTainted: (value: Option<boolean>)=>{tainted.locality = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("locality", data.locality); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},administrativeAreaLevel3: {path: ["administrativeAreaLevel3" ]as const, name: "administrativeAreaLevel3" , constraints: { required: true }, get: ()=>data.administrativeAreaLevel3, set: (value: string | null)=>{data.administrativeAreaLevel3 = value;}, transform: (value: string | null): string | null =>value,getError: ()=>errors.administrativeAreaLevel3, setError: (value: Option<Array<string>>)=>{errors.administrativeAreaLevel3 = value;}, getTainted: ()=>tainted.administrativeAreaLevel3, setTainted: (value: Option<boolean>)=>{tainted.administrativeAreaLevel3 = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("administrativeAreaLevel3", data.administrativeAreaLevel3); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},administrativeAreaLevel2: {path: ["administrativeAreaLevel2" ]as const, name: "administrativeAreaLevel2" , constraints: { required: true }, get: ()=>data.administrativeAreaLevel2, set: (value: string | null)=>{data.administrativeAreaLevel2 = value;}, transform: (value: string | null): string | null =>value,getError: ()=>errors.administrativeAreaLevel2, setError: (value: Option<Array<string>>)=>{errors.administrativeAreaLevel2 = value;}, getTainted: ()=>tainted.administrativeAreaLevel2, setTainted: (value: Option<boolean>)=>{tainted.administrativeAreaLevel2 = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("administrativeAreaLevel2", data.administrativeAreaLevel2); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},administrativeAreaLevel1: {path: ["administrativeAreaLevel1" ]as const, name: "administrativeAreaLevel1" , constraints: { required: true }, get: ()=>data.administrativeAreaLevel1, set: (value: string)=>{data.administrativeAreaLevel1 = value;}, transform: (value: string): string =>value,getError: ()=>errors.administrativeAreaLevel1, setError: (value: Option<Array<string>>)=>{errors.administrativeAreaLevel1 = value;}, getTainted: ()=>tainted.administrativeAreaLevel1, setTainted: (value: Option<boolean>)=>{tainted.administrativeAreaLevel1 = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("administrativeAreaLevel1", data.administrativeAreaLevel1); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},country: {path: ["country" ]as const, name: "country" , constraints: { required: true }, get: ()=>data.country, set: (value: string)=>{data.country = value;}, transform: (value: string): string =>value,getError: ()=>errors.country, setError: (value: Option<Array<string>>)=>{errors.country = value;}, getTainted: ()=>tainted.country, setTainted: (value: Option<boolean>)=>{tainted.country = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("country", data.country); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},postalCode: {path: ["postalCode" ]as const, name: "postalCode" , constraints: { required: true }, get: ()=>data.postalCode, set: (value: string)=>{data.postalCode = value;}, transform: (value: string): string =>value,getError: ()=>errors.postalCode, setError: (value: Option<Array<string>>)=>{errors.postalCode = value;}, getTainted: ()=>tainted.postalCode, setTainted: (value: Option<boolean>)=>{tainted.postalCode = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("postalCode", data.postalCode); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},postalCodeSuffix: {path: ["postalCodeSuffix" ]as const, name: "postalCodeSuffix" , constraints: { required: true }, get: ()=>data.postalCodeSuffix, set: (value: string | null)=>{data.postalCodeSuffix = value;}, transform: (value: string | null): string | null =>value,getError: ()=>errors.postalCodeSuffix, setError: (value: Option<Array<string>>)=>{errors.postalCodeSuffix = value;}, getTainted: ()=>tainted.postalCodeSuffix, setTainted: (value: Option<boolean>)=>{tainted.postalCodeSuffix = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("postalCodeSuffix", data.postalCodeSuffix); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},coordinates: {path: ["coordinates" ]as const, name: "coordinates" , constraints: { required: true }, get: ()=>data.coordinates, set: (value: Coordinates)=>{data.coordinates = value;}, transform: (value: Coordinates): Coordinates =>value,getError: ()=>errors.coordinates, setError: (value: Option<Array<string>>)=>{errors.coordinates = value;}, getTainted: ()=>tainted.coordinates, setTainted: (value: Option<boolean>)=>{tainted.coordinates = value;}, validate: (): Array<string>=>{const fieldErrors = siteValidateField("coordinates", data.coordinates); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},}; function validate(): Result<Site, Array<{field: string; message: string}>>{return siteDeserialize(data);}function reset(newOverrides?: Partial<Site>): void {data = {...siteDefaultValue(),...newOverrides}; errors = {_errors: Option.none(), id: Option.none(), addressLine1: Option.none(), addressLine2: Option.none(), sublocalityLevel1: Option.none(), locality: Option.none(), administrativeAreaLevel3: Option.none(), administrativeAreaLevel2: Option.none(), administrativeAreaLevel1: Option.none(), country: Option.none(), postalCode: Option.none(), postalCodeSuffix: Option.none(), coordinates: Option.none(), }; tainted = {id: Option.none(), addressLine1: Option.none(), addressLine2: Option.none(), sublocalityLevel1: Option.none(), locality: Option.none(), administrativeAreaLevel3: Option.none(), administrativeAreaLevel2: Option.none(), administrativeAreaLevel1: Option.none(), country: Option.none(), postalCode: Option.none(), postalCodeSuffix: Option.none(), coordinates: Option.none(), };}return {get data(){return data;}, set data(v){data = v;}, get errors(){return errors;}, set errors(v){errors = v;}, get tainted(){return tainted;}, set tainted(v){tainted = v;}, fields, validate, reset,};}/** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */export function siteFromFormData(formData: FormData): Result<Site, Array<{field: string; message: string}>>{const obj: Record<string, unknown>= {}; obj.id = formData.get("id" )?? "" ;obj.addressLine1 = formData.get("addressLine1" )?? "" ;obj.addressLine2 = formData.get("addressLine2" )?? "" ;obj.sublocalityLevel1 = formData.get("sublocalityLevel1" )?? "" ;obj.locality = formData.get("locality" )?? "" ;obj.administrativeAreaLevel3 = formData.get("administrativeAreaLevel3" )?? "" ;obj.administrativeAreaLevel2 = formData.get("administrativeAreaLevel2" )?? "" ;obj.administrativeAreaLevel1 = formData.get("administrativeAreaLevel1" )?? "" ;obj.country = formData.get("country" )?? "" ;obj.postalCode = formData.get("postalCode" )?? "" ;obj.postalCodeSuffix = formData.get("postalCodeSuffix" )?? "" ;{const coordinates Obj: Record<string, unknown>= {}; for(const[key, value]of Array.from(formData.entries())){if(key.startsWith("coordinates." )){const fieldName = key.slice("coordinates." .length); const parts = fieldName.split("." ); let current = coordinates Obj; for(let i = 0; i<parts.length - 1; i ++){const part = parts[i]!; if(!(part in current)){current[part]= {};}current = current[part]as Record<string, unknown>;}current[parts[parts.length - 1]!]= value;}}obj.coordinates = coordinates Obj;}return siteDeserialize(obj);}

export const Site = {
  defaultValue: siteDefaultValue,
  serialize: siteSerialize,
  serializeWithContext: siteSerializeWithContext,
  deserialize: siteDeserialize,
  deserializeWithContext: siteDeserializeWithContext,
  validateFields: siteValidateFields,
  hasShape: siteHasShape,
  is: siteIs,
  createForm: siteCreateForm,
  fromFormData: siteFromFormData
} as const;