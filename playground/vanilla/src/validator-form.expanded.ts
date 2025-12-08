import { Result } from "macroforge/result";
/**
 * Validator form model for E2E testing.
 * Tests string, number, array, and date validators with real form validation.
 */

/**  */
export class UserRegistrationForm {
  
  email: string;

  
  password: string;

  
  username: string;

  
  age: number;

  
  website: string;

  constructor(init: {
    email: string;
    password: string;
    username: string;
    age: number;
    website: string;
}){
    this.email = init.email;
    this.password = init.password;
    this.username = init.username;
    this.age = init.age;
    this.website = init.website;
}

  static fromJSON(data: unknown): Result<UserRegistrationForm, string[]> {
    if (typeof data !== "object" || data === null || Array.isArray(data)) {
        return Result.err([
            "UserRegistrationForm.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data)
        ]);
    }
    const obj = data as Record<string, unknown>;
    const errors: string[] = [];
    if (!("email" in obj)) {
        errors.push("UserRegistrationForm.fromJSON: missing required field \"email\"");
    }
    if (!("password" in obj)) {
        errors.push("UserRegistrationForm.fromJSON: missing required field \"password\"");
    }
    if (!("username" in obj)) {
        errors.push("UserRegistrationForm.fromJSON: missing required field \"username\"");
    }
    if (!("age" in obj)) {
        errors.push("UserRegistrationForm.fromJSON: missing required field \"age\"");
    }
    if (!("website" in obj)) {
        errors.push("UserRegistrationForm.fromJSON: missing required field \"website\"");
    }
    const __raw_email = obj["email"];
    const __val_email = __raw_email as string;
    if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(__val_email)) {
        errors.push("UserRegistrationForm.fromJSON: field 'email' must be a valid email");
    }
    const __raw_password = obj["password"];
    const __val_password = __raw_password as string;
    if (__val_password.length < 8) {
        errors.push("UserRegistrationForm.fromJSON: field 'password' must have at least 8 characters");
    }
    if (__val_password.length > 50) {
        errors.push("UserRegistrationForm.fromJSON: field 'password' must have at most 50 characters");
    }
    const __raw_username = obj["username"];
    const __val_username = __raw_username as string;
    if (__val_username.length < 3) {
        errors.push("UserRegistrationForm.fromJSON: field 'username' must have at least 3 characters");
    }
    if (__val_username.length > 20) {
        errors.push("UserRegistrationForm.fromJSON: field 'username' must have at most 20 characters");
    }
    if (__val_username !== __val_username.toLowerCase()) {
        errors.push("UserRegistrationForm.fromJSON: field 'username' must be lowercase");
    }
    if (!/^[a-z][a-z0-9_]+$/.test(__val_username)) {
        errors.push("UserRegistrationForm.fromJSON: field 'username' must match the required pattern");
    }
    const __raw_age = obj["age"];
    const __val_age = __raw_age as number;
    if (!Number.isInteger(__val_age)) {
        errors.push("UserRegistrationForm.fromJSON: field 'age' must be an integer");
    }
    if (__val_age < 18 || __val_age > 120) {
        errors.push("UserRegistrationForm.fromJSON: field 'age' must be between 18 and 120");
    }
    const __raw_website = obj["website"];
    const __val_website = __raw_website as string;
    if ((()=>{
        try {
            new URL(__val_website);
            return false;
        } catch  {
            return true;
        }
    })()) {
        errors.push("UserRegistrationForm.fromJSON: field 'website' must be a valid URL");
    }
    if (errors.length > 0) {
        return Result.err(errors);
    }
    const init: Record<string, unknown> = {};
    init.email = __raw_email as string;
    init.password = __raw_password as string;
    init.username = __raw_username as string;
    init.age = __raw_age as number;
    init.website = __raw_website as string;
    return Result.ok(new UserRegistrationForm(init as any));
}
}

/**  */
export class ProductForm {
  
  name: string;

  
  price: number;

  
  quantity: number;

  
  tags: string[];

  
  sku: string;

  constructor(init: {
    name: string;
    price: number;
    quantity: number;
    tags: string[];
    sku: string;
}){
    this.name = init.name;
    this.price = init.price;
    this.quantity = init.quantity;
    this.tags = init.tags;
    this.sku = init.sku;
}

  static fromJSON(data: unknown): Result<ProductForm, string[]> {
    if (typeof data !== "object" || data === null || Array.isArray(data)) {
        return Result.err([
            "ProductForm.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data)
        ]);
    }
    const obj = data as Record<string, unknown>;
    const errors: string[] = [];
    if (!("name" in obj)) {
        errors.push("ProductForm.fromJSON: missing required field \"name\"");
    }
    if (!("price" in obj)) {
        errors.push("ProductForm.fromJSON: missing required field \"price\"");
    }
    if (!("quantity" in obj)) {
        errors.push("ProductForm.fromJSON: missing required field \"quantity\"");
    }
    if (!("tags" in obj)) {
        errors.push("ProductForm.fromJSON: missing required field \"tags\"");
    }
    if (!("sku" in obj)) {
        errors.push("ProductForm.fromJSON: missing required field \"sku\"");
    }
    const __raw_name = obj["name"];
    const __val_name = __raw_name as string;
    if (__val_name.length === 0) {
        errors.push("ProductForm.fromJSON: field 'name' must not be empty");
    }
    if (__val_name.length > 100) {
        errors.push("ProductForm.fromJSON: field 'name' must have at most 100 characters");
    }
    const __raw_price = obj["price"];
    const __val_price = __raw_price as number;
    if (__val_price <= 0) {
        errors.push("ProductForm.fromJSON: field 'price' must be positive");
    }
    if (__val_price >= 1000000) {
        errors.push("ProductForm.fromJSON: field 'price' must be less than 1000000");
    }
    const __raw_quantity = obj["quantity"];
    const __val_quantity = __raw_quantity as number;
    if (!Number.isInteger(__val_quantity)) {
        errors.push("ProductForm.fromJSON: field 'quantity' must be an integer");
    }
    if (__val_quantity < 0) {
        errors.push("ProductForm.fromJSON: field 'quantity' must be non-negative");
    }
    const __raw_tags = obj["tags"];
    if (!Array.isArray(__raw_tags)) {
        errors.push("ProductForm.fromJSON: field \"tags\" must be an array");
    }
    const __val_tags = __raw_tags as string[];
    if (__val_tags.length < 1) {
        errors.push("ProductForm.fromJSON: field 'tags' must have at least 1 items");
    }
    if (__val_tags.length > 5) {
        errors.push("ProductForm.fromJSON: field 'tags' must have at most 5 items");
    }
    const __raw_sku = obj["sku"];
    const __val_sku = __raw_sku as string;
    if (!/^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(__val_sku)) {
        errors.push("ProductForm.fromJSON: field 'sku' must be a valid UUID");
    }
    if (errors.length > 0) {
        return Result.err(errors);
    }
    const init: Record<string, unknown> = {};
    init.name = __raw_name as string;
    init.price = __raw_price as number;
    init.quantity = __raw_quantity as number;
    init.tags = (__raw_tags as unknown[]).map((item)=>typeof (item as any)?.constructor?.fromJSON === "function" ? (item as any).constructor.fromJSON(item) : item as string);
    init.sku = __raw_sku as string;
    return Result.ok(new ProductForm(init as any));
}
}

/**  */
export class EventForm {
  
  title: string;

  
  startDate: Date;

  
  endDate: Date;

  
  maxAttendees: number;

  constructor(init: {
    title: string;
    startDate: Date;
    endDate: Date;
    maxAttendees: number;
}){
    this.title = init.title;
    this.startDate = init.startDate;
    this.endDate = init.endDate;
    this.maxAttendees = init.maxAttendees;
}

  static fromJSON(data: unknown): Result<EventForm, string[]> {
    if (typeof data !== "object" || data === null || Array.isArray(data)) {
        return Result.err([
            "EventForm.fromJSON: expected an object, got " + (Array.isArray(data) ? "array" : typeof data)
        ]);
    }
    const obj = data as Record<string, unknown>;
    const errors: string[] = [];
    if (!("title" in obj)) {
        errors.push("EventForm.fromJSON: missing required field \"title\"");
    }
    if (!("startDate" in obj)) {
        errors.push("EventForm.fromJSON: missing required field \"startDate\"");
    }
    if (!("endDate" in obj)) {
        errors.push("EventForm.fromJSON: missing required field \"endDate\"");
    }
    if (!("maxAttendees" in obj)) {
        errors.push("EventForm.fromJSON: missing required field \"maxAttendees\"");
    }
    const __raw_title = obj["title"];
    const __val_title = __raw_title as string;
    if (__val_title.length === 0) {
        errors.push("EventForm.fromJSON: field 'title' must not be empty");
    }
    if (__val_title !== __val_title.trim()) {
        errors.push("EventForm.fromJSON: field 'title' must be trimmed (no leading/trailing whitespace)");
    }
    const __raw_startDate = obj["startDate"];
    let __val_startDate: Date;
    if (typeof __raw_startDate === "string") {
        __val_startDate = new Date(__raw_startDate);
    } else if (__raw_startDate instanceof Date) {
        __val_startDate = __raw_startDate;
    } else {
        errors.push("EventForm.fromJSON: field \"startDate\" must be a Date or ISO string");
        __val_startDate = new Date();
    }
    if (isNaN(__val_startDate.getTime())) {
        errors.push("EventForm.fromJSON: field 'startDate' must be a valid date");
    }
    if (__val_startDate.getTime() <= new Date("2020-01-01").getTime()) {
        errors.push("EventForm.fromJSON: field 'startDate' must be after 2020-01-01");
    }
    const __raw_endDate = obj["endDate"];
    let __val_endDate: Date;
    if (typeof __raw_endDate === "string") {
        __val_endDate = new Date(__raw_endDate);
    } else if (__raw_endDate instanceof Date) {
        __val_endDate = __raw_endDate;
    } else {
        errors.push("EventForm.fromJSON: field \"endDate\" must be a Date or ISO string");
        __val_endDate = new Date();
    }
    if (isNaN(__val_endDate.getTime())) {
        errors.push("EventForm.fromJSON: field 'endDate' must be a valid date");
    }
    const __raw_maxAttendees = obj["maxAttendees"];
    const __val_maxAttendees = __raw_maxAttendees as number;
    if (!Number.isInteger(__val_maxAttendees)) {
        errors.push("EventForm.fromJSON: field 'maxAttendees' must be an integer");
    }
    if (__val_maxAttendees < 1 || __val_maxAttendees > 1000) {
        errors.push("EventForm.fromJSON: field 'maxAttendees' must be between 1 and 1000");
    }
    if (errors.length > 0) {
        return Result.err(errors);
    }
    const init: Record<string, unknown> = {};
    init.title = __raw_title as string;
    if (typeof __raw_startDate === "string") {
        init.startDate = new Date(__raw_startDate);
    } else {
        init.startDate = __raw_startDate as Date;
    }
    if (typeof __raw_endDate === "string") {
        init.endDate = new Date(__raw_endDate);
    } else {
        init.endDate = __raw_endDate as Date;
    }
    init.maxAttendees = __raw_maxAttendees as number;
    return Result.ok(new EventForm(init as any));
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
  const result = (UserRegistrationForm as any).fromJSON(data);
  return toValidationResult(result);
}

export function validateProduct(data: unknown): ValidationResult<ProductForm> {
  const result = (ProductForm as any).fromJSON(data);
  return toValidationResult(result);
}

export function validateEvent(data: unknown): ValidationResult<EventForm> {
  const result = (EventForm as any).fromJSON(data);
  return toValidationResult(result);
}