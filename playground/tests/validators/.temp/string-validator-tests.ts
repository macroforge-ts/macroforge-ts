import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * String validator test classes for comprehensive deserializer validation testing.
 * Each class tests a single validator for isolation.
 */

// Email validator

export class EmailValidator {
    
    email: string;

    constructor(props: {
    email: string;
}){
    this.email = props.email;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<EmailValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return EmailValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<EmailValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = EmailValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "EmailValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): EmailValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "EmailValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("email" in obj)) {
        errors.push({
            field: "email",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(EmailValidator.prototype) as EmailValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_email = obj["email"];
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__raw_email)) {
            errors.push({
                field: "email",
                message: "must be a valid email"
            });
        }
        instance.email = __raw_email;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof EmailValidator>(field: K, value: EmailValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "email":
            {
                const __val = value as string;
                if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
                    errors.push({
                        field: "email",
                        message: "must be a valid email"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<EmailValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("email" in partial && partial.email !== undefined) {
        const __val = partial.email as string;
        if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val)) {
            errors.push({
                field: "email",
                message: "must be a valid email"
            });
        }
    }
    return errors;
}
}

// URL validator

export class UrlValidator {
    
    url: string;

    constructor(props: {
    url: string;
}){
    this.url = props.url;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UrlValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return UrlValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<UrlValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = UrlValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "UrlValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): UrlValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "UrlValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("url" in obj)) {
        errors.push({
            field: "url",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
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
            errors.push({
                field: "url",
                message: "must be a valid URL"
            });
        }
        instance.url = __raw_url;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof UrlValidator>(field: K, value: UrlValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "url":
            {
                const __val = value as string;
                if ((()=>{
                    try {
                        new URL(__val);
                        return false;
                    } catch  {
                        return true;
                    }
                })()) {
                    errors.push({
                        field: "url",
                        message: "must be a valid URL"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<UrlValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("url" in partial && partial.url !== undefined) {
        const __val = partial.url as string;
        if ((()=>{
            try {
                new URL(__val);
                return false;
            } catch  {
                return true;
            }
        })()) {
            errors.push({
                field: "url",
                message: "must be a valid URL"
            });
        }
    }
    return errors;
}
}

// UUID validator

export class UuidValidator {
    
    id: string;

    constructor(props: {
    id: string;
}){
    this.id = props.id;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UuidValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return UuidValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<UuidValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = UuidValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "UuidValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): UuidValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "UuidValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("id" in obj)) {
        errors.push({
            field: "id",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(UuidValidator.prototype) as UuidValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj["id"];
        if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(__raw_id)) {
            errors.push({
                field: "id",
                message: "must be a valid UUID"
            });
        }
        instance.id = __raw_id;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof UuidValidator>(field: K, value: UuidValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "id":
            {
                const __val = value as string;
                if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(__val)) {
                    errors.push({
                        field: "id",
                        message: "must be a valid UUID"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<UuidValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("id" in partial && partial.id !== undefined) {
        const __val = partial.id as string;
        if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(__val)) {
            errors.push({
                field: "id",
                message: "must be a valid UUID"
            });
        }
    }
    return errors;
}
}

// MaxLength validator

export class MaxLengthValidator {
    
    shortText: string;

    constructor(props: {
    shortText: string;
}){
    this.shortText = props.shortText;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MaxLengthValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return MaxLengthValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<MaxLengthValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = MaxLengthValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "MaxLengthValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): MaxLengthValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "MaxLengthValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("shortText" in obj)) {
        errors.push({
            field: "shortText",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(MaxLengthValidator.prototype) as MaxLengthValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_shortText = obj["shortText"];
        if (__raw_shortText.length > 10) {
            errors.push({
                field: "shortText",
                message: "must have at most 10 characters"
            });
        }
        instance.shortText = __raw_shortText;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof MaxLengthValidator>(field: K, value: MaxLengthValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "shortText":
            {
                const __val = value as string;
                if (__val.length > 10) {
                    errors.push({
                        field: "shortText",
                        message: "must have at most 10 characters"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<MaxLengthValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("shortText" in partial && partial.shortText !== undefined) {
        const __val = partial.shortText as string;
        if (__val.length > 10) {
            errors.push({
                field: "shortText",
                message: "must have at most 10 characters"
            });
        }
    }
    return errors;
}
}

// MinLength validator

export class MinLengthValidator {
    
    longText: string;

    constructor(props: {
    longText: string;
}){
    this.longText = props.longText;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<MinLengthValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return MinLengthValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<MinLengthValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = MinLengthValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "MinLengthValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): MinLengthValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "MinLengthValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("longText" in obj)) {
        errors.push({
            field: "longText",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(MinLengthValidator.prototype) as MinLengthValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_longText = obj["longText"];
        if (__raw_longText.length < 5) {
            errors.push({
                field: "longText",
                message: "must have at least 5 characters"
            });
        }
        instance.longText = __raw_longText;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof MinLengthValidator>(field: K, value: MinLengthValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "longText":
            {
                const __val = value as string;
                if (__val.length < 5) {
                    errors.push({
                        field: "longText",
                        message: "must have at least 5 characters"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<MinLengthValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("longText" in partial && partial.longText !== undefined) {
        const __val = partial.longText as string;
        if (__val.length < 5) {
            errors.push({
                field: "longText",
                message: "must have at least 5 characters"
            });
        }
    }
    return errors;
}
}

// Length validator (exact)

export class LengthValidator {
    
    fixedText: string;

    constructor(props: {
    fixedText: string;
}){
    this.fixedText = props.fixedText;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LengthValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return LengthValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<LengthValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = LengthValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LengthValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): LengthValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LengthValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("fixedText" in obj)) {
        errors.push({
            field: "fixedText",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(LengthValidator.prototype) as LengthValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_fixedText = obj["fixedText"];
        if (__raw_fixedText.length !== 8) {
            errors.push({
                field: "fixedText",
                message: "must have exactly 8 characters"
            });
        }
        instance.fixedText = __raw_fixedText;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof LengthValidator>(field: K, value: LengthValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "fixedText":
            {
                const __val = value as string;
                if (__val.length !== 8) {
                    errors.push({
                        field: "fixedText",
                        message: "must have exactly 8 characters"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<LengthValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("fixedText" in partial && partial.fixedText !== undefined) {
        const __val = partial.fixedText as string;
        if (__val.length !== 8) {
            errors.push({
                field: "fixedText",
                message: "must have exactly 8 characters"
            });
        }
    }
    return errors;
}
}

// LengthRange validator (use length with 2 args)

export class LengthRangeValidator {
    
    rangedText: string;

    constructor(props: {
    rangedText: string;
}){
    this.rangedText = props.rangedText;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LengthRangeValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return LengthRangeValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<LengthRangeValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = LengthRangeValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LengthRangeValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): LengthRangeValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LengthRangeValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("rangedText" in obj)) {
        errors.push({
            field: "rangedText",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(LengthRangeValidator.prototype) as LengthRangeValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_rangedText = obj["rangedText"];
        if (__raw_rangedText.length < 5 || __raw_rangedText.length > 10) {
            errors.push({
                field: "rangedText",
                message: "must have between 5 and 10 characters"
            });
        }
        instance.rangedText = __raw_rangedText;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof LengthRangeValidator>(field: K, value: LengthRangeValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "rangedText":
            {
                const __val = value as string;
                if (__val.length < 5 || __val.length > 10) {
                    errors.push({
                        field: "rangedText",
                        message: "must have between 5 and 10 characters"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<LengthRangeValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("rangedText" in partial && partial.rangedText !== undefined) {
        const __val = partial.rangedText as string;
        if (__val.length < 5 || __val.length > 10) {
            errors.push({
                field: "rangedText",
                message: "must have between 5 and 10 characters"
            });
        }
    }
    return errors;
}
}

// Pattern validator

export class PatternValidator {
    
    code: string;

    constructor(props: {
    code: string;
}){
    this.code = props.code;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<PatternValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return PatternValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<PatternValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = PatternValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "PatternValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): PatternValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "PatternValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("code" in obj)) {
        errors.push({
            field: "code",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(PatternValidator.prototype) as PatternValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_code = obj["code"];
        if (!/^[A-Z]{3}$/.test(__raw_code)) {
            errors.push({
                field: "code",
                message: "must match the required pattern"
            });
        }
        instance.code = __raw_code;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof PatternValidator>(field: K, value: PatternValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "code":
            {
                const __val = value as string;
                if (!/^[A-Z]{3}$/.test(__val)) {
                    errors.push({
                        field: "code",
                        message: "must match the required pattern"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<PatternValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("code" in partial && partial.code !== undefined) {
        const __val = partial.code as string;
        if (!/^[A-Z]{3}$/.test(__val)) {
            errors.push({
                field: "code",
                message: "must match the required pattern"
            });
        }
    }
    return errors;
}
}

// NonEmpty validator

export class NonEmptyValidator {
    
    required: string;

    constructor(props: {
    required: string;
}){
    this.required = props.required;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<NonEmptyValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return NonEmptyValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<NonEmptyValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = NonEmptyValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "NonEmptyValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): NonEmptyValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "NonEmptyValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("required" in obj)) {
        errors.push({
            field: "required",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(NonEmptyValidator.prototype) as NonEmptyValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_required = obj["required"];
        if (__raw_required.length === 0) {
            errors.push({
                field: "required",
                message: "must not be empty"
            });
        }
        instance.required = __raw_required;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof NonEmptyValidator>(field: K, value: NonEmptyValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "required":
            {
                const __val = value as string;
                if (__val.length === 0) {
                    errors.push({
                        field: "required",
                        message: "must not be empty"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<NonEmptyValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("required" in partial && partial.required !== undefined) {
        const __val = partial.required as string;
        if (__val.length === 0) {
            errors.push({
                field: "required",
                message: "must not be empty"
            });
        }
    }
    return errors;
}
}

// Trimmed validator

export class TrimmedValidator {
    
    trimmed: string;

    constructor(props: {
    trimmed: string;
}){
    this.trimmed = props.trimmed;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<TrimmedValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return TrimmedValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<TrimmedValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = TrimmedValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "TrimmedValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): TrimmedValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "TrimmedValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("trimmed" in obj)) {
        errors.push({
            field: "trimmed",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(TrimmedValidator.prototype) as TrimmedValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_trimmed = obj["trimmed"];
        if (__raw_trimmed !== __raw_trimmed.trim()) {
            errors.push({
                field: "trimmed",
                message: "must be trimmed (no leading/trailing whitespace)"
            });
        }
        instance.trimmed = __raw_trimmed;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof TrimmedValidator>(field: K, value: TrimmedValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "trimmed":
            {
                const __val = value as string;
                if (__val !== __val.trim()) {
                    errors.push({
                        field: "trimmed",
                        message: "must be trimmed (no leading/trailing whitespace)"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<TrimmedValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("trimmed" in partial && partial.trimmed !== undefined) {
        const __val = partial.trimmed as string;
        if (__val !== __val.trim()) {
            errors.push({
                field: "trimmed",
                message: "must be trimmed (no leading/trailing whitespace)"
            });
        }
    }
    return errors;
}
}

// Lowercase validator

export class LowercaseValidator {
    
    lower: string;

    constructor(props: {
    lower: string;
}){
    this.lower = props.lower;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<LowercaseValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return LowercaseValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<LowercaseValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = LowercaseValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LowercaseValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): LowercaseValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LowercaseValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("lower" in obj)) {
        errors.push({
            field: "lower",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(LowercaseValidator.prototype) as LowercaseValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_lower = obj["lower"];
        if (__raw_lower !== __raw_lower.toLowerCase()) {
            errors.push({
                field: "lower",
                message: "must be lowercase"
            });
        }
        instance.lower = __raw_lower;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof LowercaseValidator>(field: K, value: LowercaseValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "lower":
            {
                const __val = value as string;
                if (__val !== __val.toLowerCase()) {
                    errors.push({
                        field: "lower",
                        message: "must be lowercase"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<LowercaseValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("lower" in partial && partial.lower !== undefined) {
        const __val = partial.lower as string;
        if (__val !== __val.toLowerCase()) {
            errors.push({
                field: "lower",
                message: "must be lowercase"
            });
        }
    }
    return errors;
}
}

// Uppercase validator

export class UppercaseValidator {
    
    upper: string;

    constructor(props: {
    upper: string;
}){
    this.upper = props.upper;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UppercaseValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return UppercaseValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<UppercaseValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = UppercaseValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "UppercaseValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): UppercaseValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "UppercaseValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("upper" in obj)) {
        errors.push({
            field: "upper",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(UppercaseValidator.prototype) as UppercaseValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_upper = obj["upper"];
        if (__raw_upper !== __raw_upper.toUpperCase()) {
            errors.push({
                field: "upper",
                message: "must be uppercase"
            });
        }
        instance.upper = __raw_upper;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof UppercaseValidator>(field: K, value: UppercaseValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "upper":
            {
                const __val = value as string;
                if (__val !== __val.toUpperCase()) {
                    errors.push({
                        field: "upper",
                        message: "must be uppercase"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<UppercaseValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("upper" in partial && partial.upper !== undefined) {
        const __val = partial.upper as string;
        if (__val !== __val.toUpperCase()) {
            errors.push({
                field: "upper",
                message: "must be uppercase"
            });
        }
    }
    return errors;
}
}

// Capitalized validator

export class CapitalizedValidator {
    
    cap: string;

    constructor(props: {
    cap: string;
}){
    this.cap = props.cap;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<CapitalizedValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return CapitalizedValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<CapitalizedValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = CapitalizedValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "CapitalizedValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): CapitalizedValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "CapitalizedValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("cap" in obj)) {
        errors.push({
            field: "cap",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(CapitalizedValidator.prototype) as CapitalizedValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_cap = obj["cap"];
        if (__raw_cap.length > 0 && __raw_cap[0] !== __raw_cap[0].toUpperCase()) {
            errors.push({
                field: "cap",
                message: "must be capitalized"
            });
        }
        instance.cap = __raw_cap;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof CapitalizedValidator>(field: K, value: CapitalizedValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "cap":
            {
                const __val = value as string;
                if (__val.length > 0 && __val[0] !== __val[0].toUpperCase()) {
                    errors.push({
                        field: "cap",
                        message: "must be capitalized"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<CapitalizedValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("cap" in partial && partial.cap !== undefined) {
        const __val = partial.cap as string;
        if (__val.length > 0 && __val[0] !== __val[0].toUpperCase()) {
            errors.push({
                field: "cap",
                message: "must be capitalized"
            });
        }
    }
    return errors;
}
}

// Uncapitalized validator

export class UncapitalizedValidator {
    
    uncap: string;

    constructor(props: {
    uncap: string;
}){
    this.uncap = props.uncap;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UncapitalizedValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return UncapitalizedValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<UncapitalizedValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = UncapitalizedValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "UncapitalizedValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): UncapitalizedValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "UncapitalizedValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("uncap" in obj)) {
        errors.push({
            field: "uncap",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(UncapitalizedValidator.prototype) as UncapitalizedValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_uncap = obj["uncap"];
        if (__raw_uncap.length > 0 && __raw_uncap[0] !== __raw_uncap[0].toLowerCase()) {
            errors.push({
                field: "uncap",
                message: "must not be capitalized"
            });
        }
        instance.uncap = __raw_uncap;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof UncapitalizedValidator>(field: K, value: UncapitalizedValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "uncap":
            {
                const __val = value as string;
                if (__val.length > 0 && __val[0] !== __val[0].toLowerCase()) {
                    errors.push({
                        field: "uncap",
                        message: "must not be capitalized"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<UncapitalizedValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("uncap" in partial && partial.uncap !== undefined) {
        const __val = partial.uncap as string;
        if (__val.length > 0 && __val[0] !== __val[0].toLowerCase()) {
            errors.push({
                field: "uncap",
                message: "must not be capitalized"
            });
        }
    }
    return errors;
}
}

// StartsWith validator

export class StartsWithValidator {
    
    secureUrl: string;

    constructor(props: {
    secureUrl: string;
}){
    this.secureUrl = props.secureUrl;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<StartsWithValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return StartsWithValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<StartsWithValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = StartsWithValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "StartsWithValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): StartsWithValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "StartsWithValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("secureUrl" in obj)) {
        errors.push({
            field: "secureUrl",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(StartsWithValidator.prototype) as StartsWithValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_secureUrl = obj["secureUrl"];
        if (!__raw_secureUrl.startsWith("https://")) {
            errors.push({
                field: "secureUrl",
                message: "must start with 'https://'"
            });
        }
        instance.secureUrl = __raw_secureUrl;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof StartsWithValidator>(field: K, value: StartsWithValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "secureUrl":
            {
                const __val = value as string;
                if (!__val.startsWith("https://")) {
                    errors.push({
                        field: "secureUrl",
                        message: "must start with 'https://'"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<StartsWithValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("secureUrl" in partial && partial.secureUrl !== undefined) {
        const __val = partial.secureUrl as string;
        if (!__val.startsWith("https://")) {
            errors.push({
                field: "secureUrl",
                message: "must start with 'https://'"
            });
        }
    }
    return errors;
}
}

// EndsWith validator

export class EndsWithValidator {
    
    filename: string;

    constructor(props: {
    filename: string;
}){
    this.filename = props.filename;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<EndsWithValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return EndsWithValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<EndsWithValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = EndsWithValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "EndsWithValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): EndsWithValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "EndsWithValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("filename" in obj)) {
        errors.push({
            field: "filename",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(EndsWithValidator.prototype) as EndsWithValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_filename = obj["filename"];
        if (!__raw_filename.endsWith(".json")) {
            errors.push({
                field: "filename",
                message: "must end with '.json'"
            });
        }
        instance.filename = __raw_filename;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof EndsWithValidator>(field: K, value: EndsWithValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "filename":
            {
                const __val = value as string;
                if (!__val.endsWith(".json")) {
                    errors.push({
                        field: "filename",
                        message: "must end with '.json'"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<EndsWithValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("filename" in partial && partial.filename !== undefined) {
        const __val = partial.filename as string;
        if (!__val.endsWith(".json")) {
            errors.push({
                field: "filename",
                message: "must end with '.json'"
            });
        }
    }
    return errors;
}
}

// Includes validator

export class IncludesValidator {
    
    emailLike: string;

    constructor(props: {
    emailLike: string;
}){
    this.emailLike = props.emailLike;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<IncludesValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return IncludesValidator.fromObject(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<IncludesValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = IncludesValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "IncludesValidator.fromObject: root cannot be a forward reference"
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([
            {
                field: "_root",
                message
            }
        ]);
    }
}

    static __deserialize(value: any, ctx: DeserializeContext): IncludesValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "IncludesValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("emailLike" in obj)) {
        errors.push({
            field: "emailLike",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(IncludesValidator.prototype) as IncludesValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_emailLike = obj["emailLike"];
        if (!__raw_emailLike.includes("@")) {
            errors.push({
                field: "emailLike",
                message: "must include '@'"
            });
        }
        instance.emailLike = __raw_emailLike;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof IncludesValidator>(field: K, value: IncludesValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "emailLike":
            {
                const __val = value as string;
                if (!__val.includes("@")) {
                    errors.push({
                        field: "emailLike",
                        message: "must include '@'"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<IncludesValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("emailLike" in partial && partial.emailLike !== undefined) {
        const __val = partial.emailLike as string;
        if (!__val.includes("@")) {
            errors.push({
                field: "emailLike",
                message: "must include '@'"
            });
        }
    }
    return errors;
}
}