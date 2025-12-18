import { dataPathDefaultValue } from "./data-path.svelte";
import { SerializeContext } from "macroforge/serde";
import { dataPathSerializeWithContext } from "./data-path.svelte";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
import { dataPathDeserializeWithContext } from "./data-path.svelte";
import { Result } from "macroforge/utils";
import { Option } from "macroforge/utils";
import type { FieldController } from "@playground/macro/gigaform";
/** import macro {Gigaform} from "@playground/macro"; */

import type { DataPath } from './data-path.svelte';


export interface ColumnConfig {
    
    heading: string;
    dataPath: DataPath;
}

export function columnConfigDefaultValue(): ColumnConfig {return {heading: "",
                            dataPath: dataPathDefaultValue(), }as ColumnConfig;}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */export function columnConfigSerialize(value: ColumnConfig): string {const ctx = SerializeContext.create(); return JSON.stringify(columnConfigSerializeWithContext(value, ctx));}/** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */export function columnConfigSerializeWithContext(value: ColumnConfig, ctx: SerializeContext): Record<string, unknown>{const existingId = ctx.getId(value); if(existingId!== undefined){return {__ref: existingId};}const __id = ctx.register(value); const result: Record<string, unknown>= {__type: "ColumnConfig" , __id,}; result["heading" ]= value.heading; result["dataPath" ]= dataPathSerializeWithContext(value.dataPath, ctx); return result;}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */export function columnConfigDeserialize(input: unknown, opts?: DeserializeOptions): { success: true; value: ColumnConfig } | { success: false; errors: Array<{ field: string; message: string }> } {try {const data = typeof input === "string" ? JSON.parse(input): input; const ctx = DeserializeContext.create(); const resultOrRef = columnConfigDeserializeWithContext(data, ctx); if(PendingRef.is(resultOrRef)){return { success: false, errors: [{ field: "_root", message: "ColumnConfig.deserialize: root cannot be a forward reference" }] };}ctx.applyPatches(); if(opts?.freeze){ctx.freezeAll();}return { success: true, value: resultOrRef };}catch(e){if(e instanceof DeserializeError){return { success: false, errors: e.errors };}const message = e instanceof Error? e.message: String(e); return { success: false, errors: [{ field: "_root", message }] };}}/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */export function columnConfigDeserializeWithContext(value: any, ctx: DeserializeContext): ColumnConfig | PendingRef {if(value?.__ref!== undefined){return ctx.getOrDefer(value.__ref);}if(typeof value!== "object" || value === null || Array.isArray(value)){throw new DeserializeError([{field: "_root" , message: "ColumnConfig.deserializeWithContext: expected an object" }]);}const obj = value as Record<string, unknown>; const errors: Array<{field: string; message: string}>= []; if(!("heading" in obj)){errors.push({field: "heading" , message: "missing required field" });}if(!("dataPath" in obj)){errors.push({field: "dataPath" , message: "missing required field" });}if(errors.length>0){throw new DeserializeError(errors);}const instance: any = {}; if(obj.__id!== undefined){ctx.register(obj.__id as number, instance);}ctx.trackForFreeze(instance); {const __raw_heading = obj["heading" ]as string; 
                if (__raw_heading.length === 0) {
                    errors.push({ field: "heading", message: "must not be empty" });
                }
 instance.heading = __raw_heading; }{const __raw_dataPath = obj["dataPath" ]as DataPath; {const __result = dataPathDeserializeWithContext(__raw_dataPath, ctx); ctx.assignOrDefer(instance, "dataPath" , __result);}}if(errors.length>0){throw new DeserializeError(errors);}return instance as ColumnConfig;}export function columnConfigValidateField<K extends keyof ColumnConfig>(field: K, value: ColumnConfig[K]): Array<{field: string; message: string}>{const errors: Array<{field: string; message: string}>= []; switch(field){case "heading" : {const __val = value as string; 
                if (__val.length === 0) {
                    errors.push({ field: "heading", message: "must not be empty" });
                }
 break;}}return errors; }export function columnConfigValidateFields(partial: Partial<ColumnConfig>): Array<{field: string; message: string}>{const errors: Array<{field: string; message: string}>= []; if("heading" in partial && partial.heading!== undefined){const __val = partial.heading as string; 
                if (__val.length === 0) {
                    errors.push({ field: "heading", message: "must not be empty" });
                }
}return errors; }export function columnConfigHasShape(obj: unknown): boolean {if(typeof obj!== "object" || obj === null || Array.isArray(obj)){return false;}const o = obj as Record<string, unknown>; return "heading" in o && "dataPath" in o;}export function columnConfigIs(obj: unknown): obj is ColumnConfig {if(!columnConfigHasShape(obj)){return false;}const result = columnConfigDeserialize(obj); return result.success;}

/** Nested error structure matching the data shape */export type ColumnConfigErrors = {_errors: Option<Array<string>>; heading: Option<Array<string>>; dataPath: Option<Array<string>>; }; /** Nested boolean structure for tracking touched/dirty fields */export type ColumnConfigTainted = {heading: Option<boolean>; dataPath: Option<boolean>; }; /** Type-safe field controllers for this form */export interface ColumnConfigFieldControllers {readonly heading: FieldController<string>; readonly dataPath: FieldController<DataPath>; }/** Gigaform instance containing reactive state and field controllers */export interface ColumnConfigGigaform {readonly data: ColumnConfig; readonly errors: ColumnConfigErrors; readonly tainted: ColumnConfigTainted; readonly fields: ColumnConfigFieldControllers; validate(): Result<ColumnConfig, Array<{field: string; message: string}>>; reset(overrides?: Partial<ColumnConfig>): void;}/** Creates a new Gigaform instance with reactive state and field controllers. */export function columnConfigCreateForm(overrides?: Partial<ColumnConfig>): ColumnConfigGigaform {let data = $state({...columnConfigDefaultValue(),...overrides}); let errors = $state<ColumnConfigErrors>({_errors: Option.none(), heading: Option.none(), dataPath: Option.none(), }); let tainted = $state<ColumnConfigTainted>({heading: Option.none(), dataPath: Option.none(), }); const fields: ColumnConfigFieldControllers = {heading: {path: ["heading" ]as const, name: "heading" , constraints: { required: true }, get: ()=>data.heading, set: (value: string)=>{data.heading = value;}, transform: (value: string): string =>value,getError: ()=>errors.heading, setError: (value: Option<Array<string>>)=>{errors.heading = value;}, getTainted: ()=>tainted.heading, setTainted: (value: Option<boolean>)=>{tainted.heading = value;}, validate: (): Array<string>=>{const fieldErrors = columnConfigValidateField("heading", data.heading); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},dataPath: {path: ["dataPath" ]as const, name: "dataPath" , constraints: { required: true }, get: ()=>data.dataPath, set: (value: DataPath)=>{data.dataPath = value;}, transform: (value: DataPath): DataPath =>value,getError: ()=>errors.dataPath, setError: (value: Option<Array<string>>)=>{errors.dataPath = value;}, getTainted: ()=>tainted.dataPath, setTainted: (value: Option<boolean>)=>{tainted.dataPath = value;}, validate: (): Array<string>=>{const fieldErrors = columnConfigValidateField("dataPath", data.dataPath); return fieldErrors.map((e: {field: string; message: string})=>e.message);},},}; function validate(): Result<ColumnConfig, Array<{field: string; message: string}>>{return columnConfigDeserialize(data);}function reset(newOverrides?: Partial<ColumnConfig>): void {data = {...columnConfigDefaultValue(),...newOverrides}; errors = {_errors: Option.none(), heading: Option.none(), dataPath: Option.none(), }; tainted = {heading: Option.none(), dataPath: Option.none(), };}return {get data(){return data;}, set data(v){data = v;}, get errors(){return errors;}, set errors(v){errors = v;}, get tainted(){return tainted;}, set tainted(v){tainted = v;}, fields, validate, reset,};}/** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */export function columnConfigFromFormData(formData: FormData): Result<ColumnConfig, Array<{field: string; message: string}>>{const obj: Record<string, unknown>= {}; obj.heading = formData.get("heading" )?? "" ;{const dataPath Obj: Record<string, unknown>= {}; for(const[key, value]of Array.from(formData.entries())){if(key.startsWith("dataPath." )){const fieldName = key.slice("dataPath." .length); const parts = fieldName.split("." ); let current = dataPath Obj; for(let i = 0; i<parts.length - 1; i ++){const part = parts[i]!; if(!(part in current)){current[part]= {};}current = current[part]as Record<string, unknown>;}current[parts[parts.length - 1]!]= value;}}obj.dataPath = dataPath Obj;}return columnConfigDeserialize(obj);}

export const ColumnConfig = {
  defaultValue: columnConfigDefaultValue,
  serialize: columnConfigSerialize,
  serializeWithContext: columnConfigSerializeWithContext,
  deserialize: columnConfigDeserialize,
  deserializeWithContext: columnConfigDeserializeWithContext,
  validateFields: columnConfigValidateFields,
  hasShape: columnConfigHasShape,
  is: columnConfigIs,
  createForm: columnConfigCreateForm,
  fromFormData: columnConfigFromFormData
} as const;