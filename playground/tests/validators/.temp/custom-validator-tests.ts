import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Custom validator test classes for comprehensive deserializer validation testing.
 */

// Custom validator function for even numbers
export function isEven(value: number): boolean {
    return value % 2 === 0;
}

// Custom validator function for valid usernames
export function isValidUsername(value: string): boolean {
    return /^[a-z][a-z0-9_]{2,15}$/.test(value);
}

// Custom number validator

export class CustomNumberValidator {
    
    evenNumber: number;

    constructor(props: {
    evenNumber: number;
}){
    this.evenNumber = props.evenNumber;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<CustomNumberValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return CustomNumberValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<CustomNumberValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = CustomNumberValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "CustomNumberValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): CustomNumberValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "CustomNumberValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("evenNumber" in obj)) {
        errors.push({
            field: "evenNumber",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(CustomNumberValidator.prototype) as CustomNumberValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_evenNumber = obj["evenNumber"];
        {
            const __customResult = isEven(__raw_evenNumber);
            if (__customResult === false) {
                errors.push({
                    field: "evenNumber",
                    message: "failed custom validation"
                });
            }
        }
        instance.evenNumber = __raw_evenNumber;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof CustomNumberValidator>(field: K, value: CustomNumberValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "evenNumber":
            {
                const __val = value as number;
                {
                    const __customResult = isEven(__val);
                    if (__customResult === false) {
                        errors.push({
                            field: "evenNumber",
                            message: "failed custom validation"
                        });
                    }
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<CustomNumberValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("evenNumber" in partial && partial.evenNumber !== undefined) {
        const __val = partial.evenNumber as number;
        {
            const __customResult = isEven(__val);
            if (__customResult === false) {
                errors.push({
                    field: "evenNumber",
                    message: "failed custom validation"
                });
            }
        }
    }
    return errors;
}
}

// Custom string validator

export class CustomStringValidator {
    
    username: string;

    constructor(props: {
    username: string;
}){
    this.username = props.username;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<CustomStringValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return CustomStringValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<CustomStringValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = CustomStringValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "CustomStringValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): CustomStringValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "CustomStringValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("username" in obj)) {
        errors.push({
            field: "username",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(CustomStringValidator.prototype) as CustomStringValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_username = obj["username"];
        {
            const __customResult = isValidUsername(__raw_username);
            if (__customResult === false) {
                errors.push({
                    field: "username",
                    message: "failed custom validation"
                });
            }
        }
        instance.username = __raw_username;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof CustomStringValidator>(field: K, value: CustomStringValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "username":
            {
                const __val = value as string;
                {
                    const __customResult = isValidUsername(__val);
                    if (__customResult === false) {
                        errors.push({
                            field: "username",
                            message: "failed custom validation"
                        });
                    }
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<CustomStringValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("username" in partial && partial.username !== undefined) {
        const __val = partial.username as string;
        {
            const __customResult = isValidUsername(__val);
            if (__customResult === false) {
                errors.push({
                    field: "username",
                    message: "failed custom validation"
                });
            }
        }
    }
    return errors;
}
}

// Custom validator with custom message

export class CustomWithMessageValidator {
    
    evenNumber: number;

    constructor(props: {
    evenNumber: number;
}){
    this.evenNumber = props.evenNumber;
}

    static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<CustomWithMessageValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return CustomWithMessageValidator.fromObject(raw, opts);
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

    static fromObject(obj: unknown, opts?: DeserializeOptions): Result<CustomWithMessageValidator, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = CustomWithMessageValidator.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "CustomWithMessageValidator.fromObject: root cannot be a forward reference"
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

    static __deserialize(value: any, ctx: DeserializeContext): CustomWithMessageValidator | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "CustomWithMessageValidator.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("evenNumber" in obj)) {
        errors.push({
            field: "evenNumber",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(CustomWithMessageValidator.prototype) as CustomWithMessageValidator;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_evenNumber = obj["evenNumber"];
        {
            const __customResult = isEven(__raw_evenNumber);
            if (__customResult === false) {
                errors.push({
                    field: "evenNumber",
                    message: "Number must be even"
                });
            }
        }
        instance.evenNumber = __raw_evenNumber;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}

    static validateField<K extends keyof CustomWithMessageValidator>(field: K, value: CustomWithMessageValidator[K]): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    switch(field){
        case "evenNumber":
            {
                const __val = value as number;
                {
                    const __customResult = isEven(__val);
                    if (__customResult === false) {
                        errors.push({
                            field: "evenNumber",
                            message: "Number must be even"
                        });
                    }
                }
                break;
            }
    }
    return errors;
}

    static validateFields(partial: Partial<CustomWithMessageValidator>): Array<{
    field: string;
    message: string;
}> {
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if ("evenNumber" in partial && partial.evenNumber !== undefined) {
        const __val = partial.evenNumber as number;
        {
            const __customResult = isEven(__val);
            if (__customResult === false) {
                errors.push({
                    field: "evenNumber",
                    message: "Number must be even"
                });
            }
        }
    }
    return errors;
}
}