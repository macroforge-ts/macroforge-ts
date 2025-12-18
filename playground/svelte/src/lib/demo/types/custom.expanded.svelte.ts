import { SerializeContext } from "macroforge/serde";
import { directionHueSerializeWithContext } from "./direction-hue.svelte";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
import { Result } from "macroforge/utils";
import { Option } from "macroforge/utils";
import type { FieldController } from "@playground/macro/gigaform";
import type { ArrayFieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { DirectionHue } from './direction-hue.svelte';


export interface Custom {
    mappings: DirectionHue[];
}

export function customDefaultValue(): Custom {return {mappings: [], }as Custom;}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */export function customSerialize(value: Custom): string {const ctx = SerializeContext.create(); return JSON.stringify(customSerializeWithContext(value, ctx));}/** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */export function customSerializeWithContext(value: Custom, ctx: SerializeContext): Record<string, unknown>{const existingId = ctx.getId(value); if(existingId!== undefined){return {__ref: existingId};}const __id = ctx.register(value); const result: Record<string, unknown>= {__type: "Custom" , __id,}; result["mappings" ]= value.mappings.map((item)=>directionHueSerializeWithContext(item, ctx)); return result;}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */export function customDeserialize(input: unknown, opts?: DeserializeOptions): { success: true; value: Custom } | { success: false; errors: Array<{ field: string; message: string }> } {try {const data = typeof input === "string" ? JSON.parse(input): input; const ctx = DeserializeContext.create(); const resultOrRef = customDeserializeWithContext(data, ctx); if(PendingRef.is(resultOrRef)){return { success: false, errors: [{ field: "_root", message: "Custom.deserialize: root cannot be a forward reference" }] };}ctx.applyPatches(); if(opts?.freeze){ctx.freezeAll();}return { success: true, value: resultOrRef };}catch(e){if(e instanceof DeserializeError){return { success: false, errors: e.errors };}const message = e instanceof Error? e.message: String(e); return { success: false, errors: [{ field: "_root", message }] };}}/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */export function customDeserializeWithContext(value: any, ctx: DeserializeContext): Custom | PendingRef {if(value?.__ref!== undefined){return ctx.getOrDefer(value.__ref);}if(typeof value!== "object" || value === null || Array.isArray(value)){throw new DeserializeError([{field: "_root" , message: "Custom.deserializeWithContext: expected an object" }]);}const obj = value as Record<string, unknown>; const errors: Array<{field: string; message: string}>= []; if(!("mappings" in obj)){errors.push({field: "mappings" , message: "missing required field" });}if(errors.length>0){throw new DeserializeError(errors);}const instance: any = {}; if(obj.__id!== undefined){ctx.register(obj.__id as number, instance);}ctx.trackForFreeze(instance); {const __raw_mappings = obj["mappings" ]as DirectionHue[]; if(Array.isArray(__raw_mappings)){instance.mappings = __raw_mappings as DirectionHue[];}}if(errors.length>0){throw new DeserializeError(errors);}return instance as Custom;}export function customValidateField<K extends keyof Custom>(field: K, value: Custom[K]): Array<{field: string; message: string}>{return[]; }export function customValidateFields(partial: Partial<Custom>): Array<{field: string; message: string}>{return[]; }export function customHasShape(obj: unknown): boolean {if(typeof obj!== "object" || obj === null || Array.isArray(obj)){return false;}const o = obj as Record<string, unknown>; return "mappings" in o;}export function customIs(obj: unknown): obj is Custom {if(!customHasShape(obj)){return false;}const result = customDeserialize(obj); return result.success;}

/** Nested error structure matching the data shape */export type CustomErrors = {_errors: Option<Array<string>>; mappings: Option<Array<string>>; }; /** Nested boolean structure for tracking touched/dirty fields */export type CustomTainted = {mappings: Option<boolean>; }; /** Type-safe field controllers for this form */export interface CustomFieldControllers {readonly mappings: ArrayFieldController<DirectionHue>; }/** Gigaform instance containing reactive state and field controllers */export interface CustomGigaform {readonly data: Custom; readonly errors: CustomErrors; readonly tainted: CustomTainted; readonly fields: CustomFieldControllers; validate(): Result<Custom, Array<{field: string; message: string}>>; reset(overrides?: Partial<Custom>): void;}/** Creates a new Gigaform instance with reactive state and field controllers. */export function customCreateForm(overrides?: Partial<Custom>): CustomGigaform {let data = $state({...customDefaultValue(),...overrides}); let errors = $state<CustomErrors>({_errors: Option.none(), mappings: Option.none(), }); let tainted = $state<CustomTainted>({mappings: Option.none(), }); const fields: CustomFieldControllers = {mappings: {path: ["mappings" ]as const, name: "mappings" , constraints: { required: true }, get: ()=>data.mappings, set: (value: DirectionHue[])=>{data.mappings = value;}, transform: (value: DirectionHue[]): DirectionHue[] =>value,getError: ()=>errors.mappings, setError: (value: Option<Array<string>>)=>{errors.mappings = value;}, getTainted: ()=>tainted.mappings, setTainted: (value: Option<boolean>)=>{tainted.mappings = value;}, validate: (): Array<string>=>{const fieldErrors = customValidateField("mappings", data.mappings); return fieldErrors.map((e: {field: string; message: string})=>e.message);},at: (index: number)=>({path: ["mappings" , index]as const, name: `mappings.${index}` , constraints: {required: true}, get: ()=>data.mappings[index]!, set: (value: DirectionHue)=>{data.mappings[index]= value;}, transform: (value: DirectionHue): DirectionHue =>value, getError: ()=>errors.mappings, setError: (value: Option<Array<string>>)=>{errors.mappings = value;}, getTainted: ()=>tainted.mappings, setTainted: (value: Option<boolean>)=>{tainted.mappings = value;}, validate: (): Array<string>=>[],}), push: (item: DirectionHue)=>{data.mappings.push(item);}, remove: (index: number)=>{data.mappings.splice(index, 1);}, swap: (a: number, b: number)=>{const tmp = data.mappings[a]!; data.mappings[a]= data.mappings[b]!; data.mappings[b]= tmp;},},}; function validate(): Result<Custom, Array<{field: string; message: string}>>{return customDeserialize(data);}function reset(newOverrides?: Partial<Custom>): void {data = {...customDefaultValue(),...newOverrides}; errors = {_errors: Option.none(), mappings: Option.none(), }; tainted = {mappings: Option.none(), };}return {get data(){return data;}, set data(v){data = v;}, get errors(){return errors;}, set errors(v){errors = v;}, get tainted(){return tainted;}, set tainted(v){tainted = v;}, fields, validate, reset,};}/** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */export function customFromFormData(formData: FormData): Result<Custom, Array<{field: string; message: string}>>{const obj: Record<string, unknown>= {}; {const mappings Items: Array<Record<string, unknown>>= []; let idx = 0; while(formData.has("mappings." + idx + "." )|| idx === 0){const hasAny = Array.from(formData.keys()).some(k =>k.startsWith("mappings." + idx + "." )); if(!hasAny && idx>0)break; if(hasAny){const item: Record<string, unknown>= {}; for(const[key, value]of Array.from(formData.entries())){if(key.startsWith("mappings." + idx + "." )){const fieldName = key.slice("mappings." .length + String(idx).length + 1); item[fieldName]= value;}}mappings Items.push(item);}idx ++; if(idx>1000)break;}obj.mappings = mappings Items;}return customDeserialize(obj);}

export const Custom = {
  defaultValue: customDefaultValue,
  serialize: customSerialize,
  serializeWithContext: customSerializeWithContext,
  deserialize: customDeserialize,
  deserializeWithContext: customDeserializeWithContext,
  validateFields: customValidateFields,
  hasShape: customHasShape,
  is: customIs,
  createForm: customCreateForm,
  fromFormData: customFromFormData
} as const;