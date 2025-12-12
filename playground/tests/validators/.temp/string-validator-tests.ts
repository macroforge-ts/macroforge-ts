import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * String validator test classes for comprehensive deserializer validation testing.
 * Each class tests a single validator for isolation.
 */

// Email validator
/**  */
export class EmailValidator {
  
  email: string;

  constructor(props: {
    email: string;
}){
    this.email = props.email;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<EmailValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = EmailValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "EmailValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): EmailValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("EmailValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("email" in obj)) {
        errors.push("EmailValidator.__deserialize: missing required field \"email\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(EmailValidator.prototype) as EmailValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_email = obj["email"];
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
            errors.push("EmailValidator.fromStringifiedJSON: field 'email' must be a valid email");
        }
        (instance as any).email = __raw_email;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// URL validator
/**  */
export class UrlValidator {
  
  url: string;

  constructor(props: {
    url: string;
}){
    this.url = props.url;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UrlValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = UrlValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "UrlValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): UrlValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("UrlValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("url" in obj)) {
        errors.push("UrlValidator.__deserialize: missing required field \"url\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(UrlValidator.prototype) as UrlValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_url = obj["url"];
        if ((()=>{
            try {
                new URL(__raw_url);
                return false;
            } catch  {
                return true;
            }
        })()) {
            errors.push("UrlValidator.fromStringifiedJSON: field 'url' must be a valid URL");
        }
        (instance as any).url = __raw_url;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// UUID validator
/**  */
export class UuidValidator {
  
  id: string;

  constructor(props: {
    id: string;
}){
    this.id = props.id;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UuidValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = UuidValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "UuidValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): UuidValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("UuidValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("id" in obj)) {
        errors.push("UuidValidator.__deserialize: missing required field \"id\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(UuidValidator.prototype) as UuidValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj["id"];
        if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(__raw_id)) {
            errors.push("UuidValidator.fromStringifiedJSON: field 'id' must be a valid UUID");
        }
        (instance as any).id = __raw_id;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// MaxLength validator
/**  */
export class MaxLengthValidator {
  
  shortText: string;

  constructor(props: {
    shortText: string;
}){
    this.shortText = props.shortText;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MaxLengthValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = MaxLengthValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "MaxLengthValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): MaxLengthValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("MaxLengthValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("shortText" in obj)) {
        errors.push("MaxLengthValidator.__deserialize: missing required field \"shortText\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(MaxLengthValidator.prototype) as MaxLengthValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_shortText = obj["shortText"];
        if (__raw_shortText.length > 10) {
            errors.push("MaxLengthValidator.fromStringifiedJSON: field 'shortText' must have at most 10 characters");
        }
        (instance as any).shortText = __raw_shortText;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// MinLength validator
/**  */
export class MinLengthValidator {
  
  longText: string;

  constructor(props: {
    longText: string;
}){
    this.longText = props.longText;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MinLengthValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = MinLengthValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "MinLengthValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): MinLengthValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("MinLengthValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("longText" in obj)) {
        errors.push("MinLengthValidator.__deserialize: missing required field \"longText\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(MinLengthValidator.prototype) as MinLengthValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_longText = obj["longText"];
        if (__raw_longText.length < 5) {
            errors.push("MinLengthValidator.fromStringifiedJSON: field 'longText' must have at least 5 characters");
        }
        (instance as any).longText = __raw_longText;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Length validator (exact)
/**  */
export class LengthValidator {
  
  fixedText: string;

  constructor(props: {
    fixedText: string;
}){
    this.fixedText = props.fixedText;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LengthValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = LengthValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "LengthValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): LengthValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("LengthValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("fixedText" in obj)) {
        errors.push("LengthValidator.__deserialize: missing required field \"fixedText\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(LengthValidator.prototype) as LengthValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_fixedText = obj["fixedText"];
        if (__raw_fixedText.length !== 8) {
            errors.push("LengthValidator.fromStringifiedJSON: field 'fixedText' must have exactly 8 characters");
        }
        (instance as any).fixedText = __raw_fixedText;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// LengthRange validator (use length with 2 args)
/**  */
export class LengthRangeValidator {
  
  rangedText: string;

  constructor(props: {
    rangedText: string;
}){
    this.rangedText = props.rangedText;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LengthRangeValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = LengthRangeValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "LengthRangeValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): LengthRangeValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("LengthRangeValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("rangedText" in obj)) {
        errors.push("LengthRangeValidator.__deserialize: missing required field \"rangedText\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(LengthRangeValidator.prototype) as LengthRangeValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_rangedText = obj["rangedText"];
        if (__raw_rangedText.length < 5 || __raw_rangedText.length > 10) {
            errors.push("LengthRangeValidator.fromStringifiedJSON: field 'rangedText' must have between 5 and 10 characters");
        }
        (instance as any).rangedText = __raw_rangedText;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Pattern validator
/**  */
export class PatternValidator {
  
  code: string;

  constructor(props: {
    code: string;
}){
    this.code = props.code;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<PatternValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = PatternValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "PatternValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): PatternValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("PatternValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("code" in obj)) {
        errors.push("PatternValidator.__deserialize: missing required field \"code\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(PatternValidator.prototype) as PatternValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_code = obj["code"];
        if (!/^[A-Z]{3}$/.test(__raw_code)) {
            errors.push("PatternValidator.fromStringifiedJSON: field 'code' must match the required pattern");
        }
        (instance as any).code = __raw_code;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// NonEmpty validator
/**  */
export class NonEmptyValidator {
  
  required: string;

  constructor(props: {
    required: string;
}){
    this.required = props.required;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonEmptyValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = NonEmptyValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "NonEmptyValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): NonEmptyValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("NonEmptyValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("required" in obj)) {
        errors.push("NonEmptyValidator.__deserialize: missing required field \"required\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(NonEmptyValidator.prototype) as NonEmptyValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_required = obj["required"];
        if (__raw_required.length === 0) {
            errors.push("NonEmptyValidator.fromStringifiedJSON: field 'required' must not be empty");
        }
        (instance as any).required = __raw_required;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Trimmed validator
/**  */
export class TrimmedValidator {
  
  trimmed: string;

  constructor(props: {
    trimmed: string;
}){
    this.trimmed = props.trimmed;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<TrimmedValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = TrimmedValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "TrimmedValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): TrimmedValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("TrimmedValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("trimmed" in obj)) {
        errors.push("TrimmedValidator.__deserialize: missing required field \"trimmed\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(TrimmedValidator.prototype) as TrimmedValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_trimmed = obj["trimmed"];
        if (__raw_trimmed !== __raw_trimmed.trim()) {
            errors.push("TrimmedValidator.fromStringifiedJSON: field 'trimmed' must be trimmed (no leading/trailing whitespace)");
        }
        (instance as any).trimmed = __raw_trimmed;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Lowercase validator
/**  */
export class LowercaseValidator {
  
  lower: string;

  constructor(props: {
    lower: string;
}){
    this.lower = props.lower;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LowercaseValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = LowercaseValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "LowercaseValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): LowercaseValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("LowercaseValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("lower" in obj)) {
        errors.push("LowercaseValidator.__deserialize: missing required field \"lower\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(LowercaseValidator.prototype) as LowercaseValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_lower = obj["lower"];
        if (__raw_lower !== __raw_lower.toLowerCase()) {
            errors.push("LowercaseValidator.fromStringifiedJSON: field 'lower' must be lowercase");
        }
        (instance as any).lower = __raw_lower;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Uppercase validator
/**  */
export class UppercaseValidator {
  
  upper: string;

  constructor(props: {
    upper: string;
}){
    this.upper = props.upper;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UppercaseValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = UppercaseValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "UppercaseValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): UppercaseValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("UppercaseValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("upper" in obj)) {
        errors.push("UppercaseValidator.__deserialize: missing required field \"upper\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(UppercaseValidator.prototype) as UppercaseValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_upper = obj["upper"];
        if (__raw_upper !== __raw_upper.toUpperCase()) {
            errors.push("UppercaseValidator.fromStringifiedJSON: field 'upper' must be uppercase");
        }
        (instance as any).upper = __raw_upper;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Capitalized validator
/**  */
export class CapitalizedValidator {
  
  cap: string;

  constructor(props: {
    cap: string;
}){
    this.cap = props.cap;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<CapitalizedValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = CapitalizedValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "CapitalizedValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): CapitalizedValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("CapitalizedValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("cap" in obj)) {
        errors.push("CapitalizedValidator.__deserialize: missing required field \"cap\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(CapitalizedValidator.prototype) as CapitalizedValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_cap = obj["cap"];
        if (__raw_cap.length > 0 && __raw_cap[0] !== __raw_cap[0].toUpperCase()) {
            errors.push("CapitalizedValidator.fromStringifiedJSON: field 'cap' must be capitalized");
        }
        (instance as any).cap = __raw_cap;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Uncapitalized validator
/**  */
export class UncapitalizedValidator {
  
  uncap: string;

  constructor(props: {
    uncap: string;
}){
    this.uncap = props.uncap;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UncapitalizedValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = UncapitalizedValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "UncapitalizedValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): UncapitalizedValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("UncapitalizedValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("uncap" in obj)) {
        errors.push("UncapitalizedValidator.__deserialize: missing required field \"uncap\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(UncapitalizedValidator.prototype) as UncapitalizedValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_uncap = obj["uncap"];
        if (__raw_uncap.length > 0 && __raw_uncap[0] !== __raw_uncap[0].toLowerCase()) {
            errors.push("UncapitalizedValidator.fromStringifiedJSON: field 'uncap' must not be capitalized");
        }
        (instance as any).uncap = __raw_uncap;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// StartsWith validator
/**  */
export class StartsWithValidator {
  
  secureUrl: string;

  constructor(props: {
    secureUrl: string;
}){
    this.secureUrl = props.secureUrl;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<StartsWithValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = StartsWithValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "StartsWithValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): StartsWithValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("StartsWithValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("secureUrl" in obj)) {
        errors.push("StartsWithValidator.__deserialize: missing required field \"secureUrl\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(StartsWithValidator.prototype) as StartsWithValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_secureUrl = obj["secureUrl"];
        if (!__raw_secureUrl.startsWith("https://")) {
            errors.push("StartsWithValidator.fromStringifiedJSON: field 'secureUrl' must start with 'https://'");
        }
        (instance as any).secureUrl = __raw_secureUrl;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// EndsWith validator
/**  */
export class EndsWithValidator {
  
  filename: string;

  constructor(props: {
    filename: string;
}){
    this.filename = props.filename;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<EndsWithValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = EndsWithValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "EndsWithValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): EndsWithValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("EndsWithValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("filename" in obj)) {
        errors.push("EndsWithValidator.__deserialize: missing required field \"filename\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(EndsWithValidator.prototype) as EndsWithValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_filename = obj["filename"];
        if (!__raw_filename.endsWith(".json")) {
            errors.push("EndsWithValidator.fromStringifiedJSON: field 'filename' must end with '.json'");
        }
        (instance as any).filename = __raw_filename;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}

// Includes validator
/**  */
export class IncludesValidator {
  
  emailLike: string;

  constructor(props: {
    emailLike: string;
}){
    this.emailLike = props.emailLike;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<IncludesValidator, string[]> {
    try {
        const ctx = DeserializeContext.create();
        const raw = JSON.parse(json);
        const resultOrRef = IncludesValidator.__deserialize(raw, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                "IncludesValidator.fromStringifiedJSON: root cannot be a forward reference"
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        const message = e instanceof Error ? e.message : String(e);
        return Result.err(message.split("; "));
    }
}

  static __deserialize(value: any, ctx: DeserializeContext): IncludesValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new Error("IncludesValidator.__deserialize: expected an object");
    }
    const obj = value as Record<string, unknown>;
    const errors: string[] = [];
    if (!("emailLike" in obj)) {
        errors.push("IncludesValidator.__deserialize: missing required field \"emailLike\"");
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    const instance = Object.create(IncludesValidator.prototype) as IncludesValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_emailLike = obj["emailLike"];
        if (!__raw_emailLike.includes("@")) {
            errors.push("IncludesValidator.fromStringifiedJSON: field 'emailLike' must include '@'");
        }
        (instance as any).emailLike = __raw_emailLike;
    }
    if (errors.length > 0) {
        throw new Error(errors.join("; "));
    }
    return instance;
}
}