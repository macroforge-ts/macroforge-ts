import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Date validator test classes for comprehensive deserializer validation testing.
 */

// ValidDate validator

export class ValidDateValidator {
    
    date: Date;

    constructor(props: {
    date: Date;
}){
    this.date = props.date;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<ValidDateValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = ValidDateValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "ValidDateValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): ValidDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "ValidDateValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("date" in obj)) {
        errors.push({
            field: "date",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(ValidDateValidator.prototype) as ValidDateValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_date = obj["date"] as Date;
        {
            const __dateVal = typeof __raw_date === "string" ? new Date(__raw_date) : __raw_date as Date;
            if (__dateVal == null || isNaN(__dateVal.getTime())) {
                errors.push({
                    field: "date",
                    message: "must be a valid date"
                });
            }
            instance.date = __dateVal;
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof ValidDateValidator>(field: K, value: ValidDateValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "date":
            {
                const __val = value as Date;
                if (__val == null || isNaN(__val.getTime())) {
                    errors.push({
                        field: "date",
                        message: "must be a valid date"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<ValidDateValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("date" in partial && partial.date !== undefined) {
        const __val = partial.date as Date;
        if (__val == null || isNaN(__val.getTime())) {
            errors.push({
                field: "date",
                message: "must be a valid date"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return "date" in o;
}

    static is(obj: unknown): obj is ValidDateValidator {
    if (obj instanceof ValidDateValidator) {
        return true;
    }
    if (!ValidDateValidator.hasShape(obj)) {
        return false;
    }
    const result = ValidDateValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// GreaterThanDate validator

export class GreaterThanDateValidator {
    
    date: Date;

    constructor(props: {
    date: Date;
}){
    this.date = props.date;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<GreaterThanDateValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = GreaterThanDateValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "GreaterThanDateValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): GreaterThanDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "GreaterThanDateValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("date" in obj)) {
        errors.push({
            field: "date",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(GreaterThanDateValidator.prototype) as GreaterThanDateValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_date = obj["date"] as Date;
        {
            const __dateVal = typeof __raw_date === "string" ? new Date(__raw_date) : __raw_date as Date;
            if (__dateVal == null || __dateVal.getTime() <= new Date("2020-01-01").getTime()) {
                errors.push({
                    field: "date",
                    message: "must be after 2020-01-01"
                });
            }
            instance.date = __dateVal;
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof GreaterThanDateValidator>(field: K, value: GreaterThanDateValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "date":
            {
                const __val = value as Date;
                if (__val == null || __val.getTime() <= new Date("2020-01-01").getTime()) {
                    errors.push({
                        field: "date",
                        message: "must be after 2020-01-01"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<GreaterThanDateValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("date" in partial && partial.date !== undefined) {
        const __val = partial.date as Date;
        if (__val == null || __val.getTime() <= new Date("2020-01-01").getTime()) {
            errors.push({
                field: "date",
                message: "must be after 2020-01-01"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return "date" in o;
}

    static is(obj: unknown): obj is GreaterThanDateValidator {
    if (obj instanceof GreaterThanDateValidator) {
        return true;
    }
    if (!GreaterThanDateValidator.hasShape(obj)) {
        return false;
    }
    const result = GreaterThanDateValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// GreaterThanOrEqualToDate validator

export class GreaterThanOrEqualToDateValidator {
    
    date: Date;

    constructor(props: {
    date: Date;
}){
    this.date = props.date;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<GreaterThanOrEqualToDateValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = GreaterThanOrEqualToDateValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "GreaterThanOrEqualToDateValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): GreaterThanOrEqualToDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "GreaterThanOrEqualToDateValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("date" in obj)) {
        errors.push({
            field: "date",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(GreaterThanOrEqualToDateValidator.prototype) as GreaterThanOrEqualToDateValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_date = obj["date"] as Date;
        {
            const __dateVal = typeof __raw_date === "string" ? new Date(__raw_date) : __raw_date as Date;
            if (__dateVal == null || __dateVal.getTime() < new Date("2020-01-01").getTime()) {
                errors.push({
                    field: "date",
                    message: "must be on or after 2020-01-01"
                });
            }
            instance.date = __dateVal;
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof GreaterThanOrEqualToDateValidator>(field: K, value: GreaterThanOrEqualToDateValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "date":
            {
                const __val = value as Date;
                if (__val == null || __val.getTime() < new Date("2020-01-01").getTime()) {
                    errors.push({
                        field: "date",
                        message: "must be on or after 2020-01-01"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<GreaterThanOrEqualToDateValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("date" in partial && partial.date !== undefined) {
        const __val = partial.date as Date;
        if (__val == null || __val.getTime() < new Date("2020-01-01").getTime()) {
            errors.push({
                field: "date",
                message: "must be on or after 2020-01-01"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return "date" in o;
}

    static is(obj: unknown): obj is GreaterThanOrEqualToDateValidator {
    if (obj instanceof GreaterThanOrEqualToDateValidator) {
        return true;
    }
    if (!GreaterThanOrEqualToDateValidator.hasShape(obj)) {
        return false;
    }
    const result = GreaterThanOrEqualToDateValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// LessThanDate validator

export class LessThanDateValidator {
    
    date: Date;

    constructor(props: {
    date: Date;
}){
    this.date = props.date;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<LessThanDateValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = LessThanDateValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LessThanDateValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): LessThanDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LessThanDateValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("date" in obj)) {
        errors.push({
            field: "date",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(LessThanDateValidator.prototype) as LessThanDateValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_date = obj["date"] as Date;
        {
            const __dateVal = typeof __raw_date === "string" ? new Date(__raw_date) : __raw_date as Date;
            if (__dateVal == null || __dateVal.getTime() >= new Date("2030-01-01").getTime()) {
                errors.push({
                    field: "date",
                    message: "must be before 2030-01-01"
                });
            }
            instance.date = __dateVal;
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof LessThanDateValidator>(field: K, value: LessThanDateValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "date":
            {
                const __val = value as Date;
                if (__val == null || __val.getTime() >= new Date("2030-01-01").getTime()) {
                    errors.push({
                        field: "date",
                        message: "must be before 2030-01-01"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<LessThanDateValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("date" in partial && partial.date !== undefined) {
        const __val = partial.date as Date;
        if (__val == null || __val.getTime() >= new Date("2030-01-01").getTime()) {
            errors.push({
                field: "date",
                message: "must be before 2030-01-01"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return "date" in o;
}

    static is(obj: unknown): obj is LessThanDateValidator {
    if (obj instanceof LessThanDateValidator) {
        return true;
    }
    if (!LessThanDateValidator.hasShape(obj)) {
        return false;
    }
    const result = LessThanDateValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// LessThanOrEqualToDate validator

export class LessThanOrEqualToDateValidator {
    
    date: Date;

    constructor(props: {
    date: Date;
}){
    this.date = props.date;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<LessThanOrEqualToDateValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = LessThanOrEqualToDateValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "LessThanOrEqualToDateValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): LessThanOrEqualToDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "LessThanOrEqualToDateValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("date" in obj)) {
        errors.push({
            field: "date",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(LessThanOrEqualToDateValidator.prototype) as LessThanOrEqualToDateValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_date = obj["date"] as Date;
        {
            const __dateVal = typeof __raw_date === "string" ? new Date(__raw_date) : __raw_date as Date;
            if (__dateVal == null || __dateVal.getTime() > new Date("2030-01-01").getTime()) {
                errors.push({
                    field: "date",
                    message: "must be on or before 2030-01-01"
                });
            }
            instance.date = __dateVal;
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof LessThanOrEqualToDateValidator>(field: K, value: LessThanOrEqualToDateValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "date":
            {
                const __val = value as Date;
                if (__val == null || __val.getTime() > new Date("2030-01-01").getTime()) {
                    errors.push({
                        field: "date",
                        message: "must be on or before 2030-01-01"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<LessThanOrEqualToDateValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("date" in partial && partial.date !== undefined) {
        const __val = partial.date as Date;
        if (__val == null || __val.getTime() > new Date("2030-01-01").getTime()) {
            errors.push({
                field: "date",
                message: "must be on or before 2030-01-01"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return "date" in o;
}

    static is(obj: unknown): obj is LessThanOrEqualToDateValidator {
    if (obj instanceof LessThanOrEqualToDateValidator) {
        return true;
    }
    if (!LessThanOrEqualToDateValidator.hasShape(obj)) {
        return false;
    }
    const result = LessThanOrEqualToDateValidator.deserialize(obj);
    return Result.isOk(result);
}
}

// BetweenDate validator

export class BetweenDateValidator {
    
    date: Date;

    constructor(props: {
    date: Date;
}){
    this.date = props.date;
}
/** Deserializes input to an instance of this class.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized instance or validation errors  */

    static deserialize(input: unknown, opts?: DeserializeOptions): Result<BetweenDateValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = BetweenDateValidator.deserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "BetweenDateValidator.deserialize: root cannot be a forward reference"
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
/** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context  */

    static deserializeWithContext(value: any, ctx: DeserializeContext): BetweenDateValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "BetweenDateValidator.deserializeWithContext: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("date" in obj)) {
        errors.push({
            field: "date",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(BetweenDateValidator.prototype) as BetweenDateValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_date = obj["date"] as Date;
        {
            const __dateVal = typeof __raw_date === "string" ? new Date(__raw_date) : __raw_date as Date;
            if (__dateVal == null || __dateVal.getTime() < new Date("2020-01-01").getTime() || __dateVal.getTime() > new Date("2030-01-01").getTime()) {
                errors.push({
                    field: "date",
                    message: "must be between 2020-01-01 and 2030-01-01"
                });
            }
            instance.date = __dateVal;
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof BetweenDateValidator>(field: K, value: BetweenDateValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "date":
            {
                const __val = value as Date;
                if (__val == null || __val.getTime() < new Date("2020-01-01").getTime() || __val.getTime() > new Date("2030-01-01").getTime()) {
                    errors.push({
                        field: "date",
                        message: "must be between 2020-01-01 and 2030-01-01"
                    });
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<BetweenDateValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("date" in partial && partial.date !== undefined) {
        const __val = partial.date as Date;
        if (__val == null || __val.getTime() < new Date("2020-01-01").getTime() || __val.getTime() > new Date("2030-01-01").getTime()) {
            errors.push({
                field: "date",
                message: "must be between 2020-01-01 and 2030-01-01"
            });
        }
    }
    return errors;
}

    static hasShape(obj: unknown): boolean {
    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return "date" in o;
}

    static is(obj: unknown): obj is BetweenDateValidator {
    if (obj instanceof BetweenDateValidator) {
        return true;
    }
    if (!BetweenDateValidator.hasShape(obj)) {
        return false;
    }
    const result = BetweenDateValidator.deserialize(obj);
    return Result.isOk(result);
}
}