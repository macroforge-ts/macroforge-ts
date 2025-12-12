import { Result } from "macroforge/utils";
import { DeserializeContext } from "macroforge/serde";
import { DeserializeError } from "macroforge/serde";
import type { DeserializeOptions } from "macroforge/serde";
import { PendingRef } from "macroforge/serde";
/**
 * Validator form model for E2E testing.
 * Tests string, number, array, and date validators with real form validation.
 */


export class UserRegistrationForm {
  
  email: string;

  
  password: string;

  
  username: string;

  
  age: number;

  
  website: string;

  constructor(props: {
    email: string;
    password: string;
    username: string;
    age: number;
    website: string;
}){
    this.email = props.email;
    this.password = props.password;
    this.username = props.username;
    this.age = props.age;
    this.website = props.website;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<UserRegistrationForm, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return UserRegistrationForm.fromObject(raw, opts);
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

  static fromObject(obj: unknown, opts?: DeserializeOptions): Result<UserRegistrationForm, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = UserRegistrationForm.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "UserRegistrationForm.fromObject: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): UserRegistrationForm | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "UserRegistrationForm.__deserialize: expected an object"
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
    if (!("password" in obj)) {
        errors.push({
            field: "password",
            message: "missing required field"
        });
    }
    if (!("username" in obj)) {
        errors.push({
            field: "username",
            message: "missing required field"
        });
    }
    if (!("age" in obj)) {
        errors.push({
            field: "age",
            message: "missing required field"
        });
    }
    if (!("website" in obj)) {
        errors.push({
            field: "website",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(UserRegistrationForm.prototype) as UserRegistrationForm;
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
        (instance as any).email = __raw_email;
    }
    {
        const __raw_password = obj["password"];
        if (__raw_password.length < 8) {
            errors.push({
                field: "password",
                message: "must have at least 8 characters"
            });
        }
        if (__raw_password.length > 50) {
            errors.push({
                field: "password",
                message: "must have at most 50 characters"
            });
        }
        (instance as any).password = __raw_password;
    }
    {
        const __raw_username = obj["username"];
        if (__raw_username.length < 3) {
            errors.push({
                field: "username",
                message: "must have at least 3 characters"
            });
        }
        if (__raw_username.length > 20) {
            errors.push({
                field: "username",
                message: "must have at most 20 characters"
            });
        }
        if (__raw_username !== __raw_username.toLowerCase()) {
            errors.push({
                field: "username",
                message: "must be lowercase"
            });
        }
        if (!/^[a-z][a-z0-9_]+$/.test(__raw_username)) {
            errors.push({
                field: "username",
                message: "must match the required pattern"
            });
        }
        (instance as any).username = __raw_username;
    }
    {
        const __raw_age = obj["age"];
        if (!Number.isInteger(__raw_age)) {
            errors.push({
                field: "age",
                message: "must be an integer"
            });
        }
        if (__raw_age < 18 || __raw_age > 120) {
            errors.push({
                field: "age",
                message: "must be between 18 and 120"
            });
        }
        (instance as any).age = __raw_age;
    }
    {
        const __raw_website = obj["website"];
        if ((()=>{
            try {
                new URL(__raw_website);
                return false;
            } catch  {
                return true;
            }
        })()) {
            errors.push({
                field: "website",
                message: "must be a valid URL"
            });
        }
        (instance as any).website = __raw_website;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}
}


export class ProductForm {
  
  name: string;

  
  price: number;

  
  quantity: number;

  
  tags: string[];

  
  sku: string;

  constructor(props: {
    name: string;
    price: number;
    quantity: number;
    tags: string[];
    sku: string;
}){
    this.name = props.name;
    this.price = props.price;
    this.quantity = props.quantity;
    this.tags = props.tags;
    this.sku = props.sku;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<ProductForm, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return ProductForm.fromObject(raw, opts);
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

  static fromObject(obj: unknown, opts?: DeserializeOptions): Result<ProductForm, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = ProductForm.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "ProductForm.fromObject: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): ProductForm | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "ProductForm.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("name" in obj)) {
        errors.push({
            field: "name",
            message: "missing required field"
        });
    }
    if (!("price" in obj)) {
        errors.push({
            field: "price",
            message: "missing required field"
        });
    }
    if (!("quantity" in obj)) {
        errors.push({
            field: "quantity",
            message: "missing required field"
        });
    }
    if (!("tags" in obj)) {
        errors.push({
            field: "tags",
            message: "missing required field"
        });
    }
    if (!("sku" in obj)) {
        errors.push({
            field: "sku",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(ProductForm.prototype) as ProductForm;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_name = obj["name"];
        if (__raw_name.length === 0) {
            errors.push({
                field: "name",
                message: "must not be empty"
            });
        }
        if (__raw_name.length > 100) {
            errors.push({
                field: "name",
                message: "must have at most 100 characters"
            });
        }
        (instance as any).name = __raw_name;
    }
    {
        const __raw_price = obj["price"];
        if (__raw_price <= 0) {
            errors.push({
                field: "price",
                message: "must be positive"
            });
        }
        if (__raw_price >= 1000000) {
            errors.push({
                field: "price",
                message: "must be less than 1000000"
            });
        }
        (instance as any).price = __raw_price;
    }
    {
        const __raw_quantity = obj["quantity"];
        if (!Number.isInteger(__raw_quantity)) {
            errors.push({
                field: "quantity",
                message: "must be an integer"
            });
        }
        if (__raw_quantity < 0) {
            errors.push({
                field: "quantity",
                message: "must be non-negative"
            });
        }
        (instance as any).quantity = __raw_quantity;
    }
    {
        const __raw_tags = obj["tags"];
        if (Array.isArray(__raw_tags)) {
            if (__raw_tags.length < 1) {
                errors.push({
                    field: "tags",
                    message: "must have at least 1 items"
                });
            }
            if (__raw_tags.length > 5) {
                errors.push({
                    field: "tags",
                    message: "must have at most 5 items"
                });
            }
            const __arr = (__raw_tags as any[]).map((item, idx)=>{
                if (item?.__ref !== undefined) {
                    const result = ctx.getOrDefer(item.__ref);
                    if (PendingRef.is(result)) {
                        return {
                            __pendingIdx: idx,
                            __refId: result.id
                        };
                    }
                    return result;
                }
                return item as string;
            });
            (instance as any).tags = __arr;
            __arr.forEach((item, idx)=>{
                if (item && typeof item === "object" && "__pendingIdx" in item) {
                    ctx.addPatch((instance as any).tags, idx, (item as any).__refId);
                }
            });
        }
    }
    {
        const __raw_sku = obj["sku"];
        if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(__raw_sku)) {
            errors.push({
                field: "sku",
                message: "must be a valid UUID"
            });
        }
        (instance as any).sku = __raw_sku;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}
}


export class EventForm {
  
  title: string;

  
  startDate: Date;

  
  endDate: Date;

  
  maxAttendees: number;

  constructor(props: {
    title: string;
    startDate: Date;
    endDate: Date;
    maxAttendees: number;
}){
    this.title = props.title;
    this.startDate = props.startDate;
    this.endDate = props.endDate;
    this.maxAttendees = props.maxAttendees;
}

  static fromStringifiedJSON(json: string, opts?: DeserializeOptions): Result<EventForm, Array<{
    field: string;
    message: string;
}>> {
    try {
        const raw = JSON.parse(json);
        return EventForm.fromObject(raw, opts);
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

  static fromObject(obj: unknown, opts?: DeserializeOptions): Result<EventForm, Array<{
    field: string;
    message: string;
}>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = EventForm.__deserialize(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: "_root",
                    message: "EventForm.fromObject: root cannot be a forward reference"
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

  static __deserialize(value: any, ctx: DeserializeContext): EventForm | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== "object" || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            {
                field: "_root",
                message: "EventForm.__deserialize: expected an object"
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{
        field: string;
        message: string;
    }> = [];
    if (!("title" in obj)) {
        errors.push({
            field: "title",
            message: "missing required field"
        });
    }
    if (!("startDate" in obj)) {
        errors.push({
            field: "startDate",
            message: "missing required field"
        });
    }
    if (!("endDate" in obj)) {
        errors.push({
            field: "endDate",
            message: "missing required field"
        });
    }
    if (!("maxAttendees" in obj)) {
        errors.push({
            field: "maxAttendees",
            message: "missing required field"
        });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance = Object.create(EventForm.prototype) as EventForm;
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_title = obj["title"];
        if (__raw_title.length === 0) {
            errors.push({
                field: "title",
                message: "must not be empty"
            });
        }
        if (__raw_title !== __raw_title.trim()) {
            errors.push({
                field: "title",
                message: "must be trimmed (no leading/trailing whitespace)"
            });
        }
        (instance as any).title = __raw_title;
    }
    {
        const __raw_startDate = obj["startDate"];
        {
            const __dateVal = typeof __raw_startDate === "string" ? new Date(__raw_startDate) : __raw_startDate as Date;
            if (__dateVal == null || isNaN(__dateVal.getTime())) {
                errors.push({
                    field: "startDate",
                    message: "must be a valid date"
                });
            }
            if (__dateVal == null || __dateVal.getTime() <= new Date("2020-01-01").getTime()) {
                errors.push({
                    field: "startDate",
                    message: "must be after 2020-01-01"
                });
            }
            (instance as any).startDate = __dateVal;
        }
    }
    {
        const __raw_endDate = obj["endDate"];
        {
            const __dateVal = typeof __raw_endDate === "string" ? new Date(__raw_endDate) : __raw_endDate as Date;
            if (__dateVal == null || isNaN(__dateVal.getTime())) {
                errors.push({
                    field: "endDate",
                    message: "must be a valid date"
                });
            }
            (instance as any).endDate = __dateVal;
        }
    }
    {
        const __raw_maxAttendees = obj["maxAttendees"];
        if (!Number.isInteger(__raw_maxAttendees)) {
            errors.push({
                field: "maxAttendees",
                message: "must be an integer"
            });
        }
        if (__raw_maxAttendees < 1 || __raw_maxAttendees > 1000) {
            errors.push({
                field: "maxAttendees",
                message: "must be between 1 and 1000"
            });
        }
        (instance as any).maxAttendees = __raw_maxAttendees;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance;
}
}

// Type for validation result
export type ValidationResult<T> = {
  success: boolean;
  data?: T;
  errors?: string[];
};

// Helper to convert Result to ValidationResult
// The Result type is provided by the macro expansion
export function toValidationResult<T>(result: any): ValidationResult<T> {
  if (result.isOk()) {
    return { success: true, data: result.unwrap() };
  } else {
    return { success: false, errors: result.unwrapErr() };
  }
}

// Form validation functions
export function validateUserRegistration(data: unknown): ValidationResult<UserRegistrationForm> {
  const result = (UserRegistrationForm as any).fromStringifiedJSON(JSON.stringify(data));
  return toValidationResult(result);
}

export function validateProduct(data: unknown): ValidationResult<ProductForm> {
  const result = (ProductForm as any).fromStringifiedJSON(JSON.stringify(data));
  return toValidationResult(result);
}

export function validateEvent(data: unknown): ValidationResult<EventForm> {
  const result = (EventForm as any).fromStringifiedJSON(JSON.stringify(data));
  return toValidationResult(result);
}