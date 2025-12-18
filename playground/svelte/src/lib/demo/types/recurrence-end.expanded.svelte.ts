import { SerializeContext } from "macroforge/serde";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
import { Result } from "macroforge/utils";
import { Option } from "macroforge/utils";
import type { FieldController } from "@playground/macro/gigaform";

export type RecurrenceEnd = /** @default(0) */ number | string;

export function recurrenceEndDefaultValue(): RecurrenceEnd {return 0;}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */export function recurrenceEndSerialize(value: RecurrenceEnd): string {const ctx = SerializeContext.create(); return JSON.stringify(recurrenceEndSerializeWithContext(value, ctx));}/** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */export function recurrenceEndSerializeWithContext(value: RecurrenceEnd, ctx: SerializeContext): unknown {if(typeof(value as any)?.serializeWithContext === "function" ){return(value as any).serializeWithContext(ctx);}return value;}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */export function recurrenceEndDeserialize(input: unknown, opts?: DeserializeOptions): { success: true; value: RecurrenceEnd } | { success: false; errors: Array<{ field: string; message: string }> } {try {const data = typeof input === "string" ? JSON.parse(input): input; const ctx = DeserializeContext.create(); const resultOrRef = recurrenceEndDeserializeWithContext(data, ctx); if(PendingRef.is(resultOrRef)){return { success: false, errors: [{ field: "_root", message: "RecurrenceEnd.deserialize: root cannot be a forward reference" }] };}ctx.applyPatches(); if(opts?.freeze){ctx.freezeAll();}return { success: true, value: resultOrRef };}catch(e){if(e instanceof DeserializeError){return { success: false, errors: e.errors };}const message = e instanceof Error? e.message: String(e); return { success: false, errors: [{ field: "_root", message }] };}}/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */export function recurrenceEndDeserializeWithContext(value: any, ctx: DeserializeContext): RecurrenceEnd | PendingRef {if(value?.__ref!== undefined){return ctx.getOrDefer(value.__ref)as RecurrenceEnd | PendingRef;}if(typeof value === "number" ){return value as RecurrenceEnd;}if(typeof value === "string" ){return value as RecurrenceEnd;}throw new DeserializeError([{field: "_root" , message: "RecurrenceEnd.deserializeWithContext: expected number, string, got " + typeof value}]); }export function recurrenceEndIs(value: unknown): value is RecurrenceEnd {return typeof value === "number" || typeof value === "string" ; }

/** Per-variant error types */export type RecurrenceEndNumber Errors = {_errors: Option<Array<string>>; }; export type RecurrenceEndString Errors = {_errors: Option<Array<string>>; }; /** Per-variant tainted types */export type RecurrenceEndNumber Tainted = {}; export type RecurrenceEndString Tainted = {}; /** Union error type */export type RecurrenceEndErrors = ({ _type: "number" } & RecurrenceEndNumberErrors) | ({ _type: "string" } & RecurrenceEndStringErrors); /** Union tainted type */export type RecurrenceEndTainted = ({ _type: "number" } & RecurrenceEndNumberTainted) | ({ _type: "string" } & RecurrenceEndStringTainted); /** Per-variant field controller types */export interface RecurrenceEndNumber FieldControllers {}export interface RecurrenceEndString FieldControllers {}/** Union Gigaform interface with variant switching */export interface RecurrenceEndGigaform {readonly currentVariant: "number" | "string"; readonly data: RecurrenceEnd; readonly errors: RecurrenceEndErrors; readonly tainted: RecurrenceEndTainted; readonly variants: RecurrenceEndVariantFields; switchVariant(variant: "number" | "string"): void; validate(): Result<RecurrenceEnd, Array<{field: string; message: string}>>; reset(overrides?: Partial<RecurrenceEnd>): void;}/** Variant fields container */export interface RecurrenceEndVariantFields {readonly number: {readonly fields: RecurrenceEndNumber FieldControllers}; readonly string: {readonly fields: RecurrenceEndString FieldControllers}; }/** Gets default value for a specific variant */function recurrenceEndGetDefaultForVariant(variant: string): RecurrenceEnd {switch(variant){case "number" : return 0 as RecurrenceEnd; case "string" : return "" as RecurrenceEnd; default: return 0 as RecurrenceEnd;}}/** Creates a new discriminated union Gigaform with variant switching */export function recurrenceEndCreateForm(initial?: RecurrenceEnd): RecurrenceEndGigaform {const initialVariant: "number" | "string" = "number"; let currentVariant = $state<"number" | "string">(initialVariant); let data = $state<RecurrenceEnd>(initial?? recurrenceEndGetDefaultForVariant(initialVariant)); let errors = $state<RecurrenceEndErrors>({}as RecurrenceEndErrors); let tainted = $state<RecurrenceEndTainted>({}as RecurrenceEndTainted); const variants: RecurrenceEndVariantFields = {number: {fields: {}as RecurrenceEndNumber FieldControllers}, string: {fields: {}as RecurrenceEndString FieldControllers}, }; function switchVariant(variant: "number" | "string"): void {currentVariant = variant; data = recurrenceEndGetDefaultForVariant(variant); errors = {}as RecurrenceEndErrors; tainted = {}as RecurrenceEndTainted;}function validate(): Result<RecurrenceEnd, Array<{field: string; message: string}>>{return recurrenceEndDeserialize(data);}function reset(overrides?: Partial<RecurrenceEnd>): void {data = overrides ? overrides as typeof data : recurrenceEndGetDefaultForVariant(currentVariant); errors = {}as RecurrenceEndErrors; tainted = {}as RecurrenceEndTainted;}return {get currentVariant(){return currentVariant;}, get data(){return data;}, set data(v){data = v;}, get errors(){return errors;}, set errors(v){errors = v;}, get tainted(){return tainted;}, set tainted(v){tainted = v;}, variants, switchVariant, validate, reset,};}/** Parses FormData for union type, determining variant from discriminant field */export function recurrenceEndFromFormData(formData: FormData): Result<RecurrenceEnd, Array<{field: string; message: string}>>{const discriminant = formData.get("_type" )as "number" | "string" | null; if(!discriminant){return Result.err([{field: "_type" , message: "Missing discriminant field" }]);}const obj: Record<string, unknown>= {}; obj._type = discriminant; if(discriminant === "number" ){}else if(discriminant === "string" ){}return recurrenceEndDeserialize(obj);}

export const RecurrenceEnd = {
  defaultValue: recurrenceEndDefaultValue,
  serialize: recurrenceEndSerialize,
  serializeWithContext: recurrenceEndSerializeWithContext,
  deserialize: recurrenceEndDeserialize,
  deserializeWithContext: recurrenceEndDeserializeWithContext,
  is: recurrenceEndIs,
  createForm: recurrenceEndCreateForm,
  fromFormData: recurrenceEndFromFormData
} as const;