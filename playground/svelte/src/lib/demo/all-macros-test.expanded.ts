import { SerializeContext } from "macroforge/serde";
import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Comprehensive test class for Svelte playground.
 * Uses all available macros for Playwright e2e testing.
 */


export interface SvelteAllMacrosTest {
  
  id: string;
  title: string;
  content: string;
  
  apiKey: string;
  count: number;
  enabled: boolean;
}

export namespace SvelteAllMacrosTest {export function toString(self: SvelteAllMacrosTest): string {const parts: string[]= []; parts.push("testId: " + self.id); parts.push("title: " + self.title); parts.push("content: " + self.content); parts.push("count: " + self.count); parts.push("enabled: " + self.enabled); return "SvelteAllMacrosTest { " + parts.join(", " )+ " }" ; }}

export namespace SvelteAllMacrosTest {export function clone(self: SvelteAllMacrosTest): SvelteAllMacrosTest {return {id: self.id, title: self.title, content: self.content, apiKey: self.apiKey, count: self.count, enabled: self.enabled, };}}

export namespace SvelteAllMacrosTest {export function equals(self: SvelteAllMacrosTest, other: SvelteAllMacrosTest): boolean {if(self === other)return true; return self.id === other.id && self.title === other.title && self.content === other.content && self.apiKey === other.apiKey && self.count === other.count && self.enabled === other.enabled;}}

export namespace SvelteAllMacrosTest {export function toStringifiedJSON(self: SvelteAllMacrosTest): string {const ctx = SerializeContext.create(); return JSON.stringify(__serialize(self, ctx));}export function toObject(self: SvelteAllMacrosTest): Record<string, unknown>{const ctx = SerializeContext.create(); return __serialize(self, ctx);}export function __serialize(self: SvelteAllMacrosTest, ctx: SerializeContext): Record<string, unknown>{const existingId = ctx.getId(self); if(existingId!== undefined){return {__ref: existingId};}const __id = ctx.register(self); const result: Record<string, unknown>= {__type: "SvelteAllMacrosTest" , __id,}; result["id" ]= self.id; result["title" ]= self.title; result["content" ]= self.content; result["apiKey" ]= self.apiKey; result["count" ]= self.count; result["enabled" ]= self.enabled; return result;}}

export namespace SvelteAllMacrosTest {export function fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<SvelteAllMacrosTest, Array<{field: string; message: string}>>{try {const raw = JSON.parse(json); return fromObject(raw, opts);}catch(e){if(e instanceof DeserializeError){return Result.err(e.errors);}const message = e instanceof Error? e.message: String(e); return Result.err([{field: "_root" , message}]);}}export function fromObject(obj: unknown, opts?: DeserializeOptions): Result<SvelteAllMacrosTest, Array<{field: string; message: string}>>{try {const ctx = DeserializeContext.create(); const resultOrRef = __deserialize(obj, ctx); if(PendingRef.is(resultOrRef)){return Result.err([{field: "_root" , message: "SvelteAllMacrosTest.fromObject: root cannot be a forward reference" }]);}ctx.applyPatches(); if(opts?.freeze){ctx.freezeAll();}return Result.ok(resultOrRef);}catch(e){if(e instanceof DeserializeError){return Result.err(e.errors);}const message = e instanceof Error? e.message: String(e); return Result.err([{field: "_root" , message}]);}}export function __deserialize(value: any, ctx: DeserializeContext): SvelteAllMacrosTest | PendingRef {if(value?.__ref!== undefined){return ctx.getOrDefer(value.__ref);}if(typeof value!== "object" || value === null || Array.isArray(value)){throw new DeserializeError([{field: "_root" , message: "SvelteAllMacrosTest.__deserialize: expected an object" }]);}const obj = value as Record<string, unknown>; const errors: Array<{field: string; message: string}>= []; if(!("id" in obj)){errors.push({field: "id" , message: "missing required field" });}if(!("title" in obj)){errors.push({field: "title" , message: "missing required field" });}if(!("content" in obj)){errors.push({field: "content" , message: "missing required field" });}if(!("apiKey" in obj)){errors.push({field: "apiKey" , message: "missing required field" });}if(!("count" in obj)){errors.push({field: "count" , message: "missing required field" });}if(!("enabled" in obj)){errors.push({field: "enabled" , message: "missing required field" });}if(errors.length>0){throw new DeserializeError(errors);}const instance: any = {}; if(obj.__id!== undefined){ctx.register(obj.__id as number, instance);}ctx.trackForFreeze(instance); {const __raw_id = obj["id" ]; instance.id = __raw_id; }{const __raw_title = obj["title" ]; instance.title = __raw_title; }{const __raw_content = obj["content" ]; instance.content = __raw_content; }{const __raw_apiKey = obj["apiKey" ]; instance.apiKey = __raw_apiKey; }{const __raw_count = obj["count" ]; instance.count = __raw_count; }{const __raw_enabled = obj["enabled" ]; instance.enabled = __raw_enabled; }if(errors.length>0){throw new DeserializeError(errors);}return instance as SvelteAllMacrosTest;}}

export namespace SvelteAllMacrosTest {export function hashCode(self: SvelteAllMacrosTest): number {let hash = 17; hash = (hash * 31 + (self.id ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) | 0;
                        hash = (hash * 31 + (self.title ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) | 0;
                        hash = (hash * 31 + (self.content ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) | 0;
                        hash = (hash * 31 + (self.apiKey ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) | 0;
                        hash = (hash * 31 + (Number.isInteger(self.count) ? self.count | 0 : self.count.toString().split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) | 0;
                        hash = (hash * 31 + (self.enabled ? 1231 : 1237)) | 0; return hash;}}

export namespace SvelteAllMacrosTest {
  export function make(
    id: string,
    title: string,
    content: string,
    apiKey: string,
    count: number,
    enabled: boolean,
  ): SvelteAllMacrosTest {
    return { id, title, content, apiKey, count, enabled };
  }
}

// Pre-instantiated test instance for e2e tests
export const svelteTestInstance = SvelteAllMacrosTest.make(
  "svelte-001",
  "Svelte Test",
  "Testing all macros in SvelteKit",
  "sk-secret-key",
  42,
  true,
);