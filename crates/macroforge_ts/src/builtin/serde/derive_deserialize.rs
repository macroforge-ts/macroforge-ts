//! # Deserialize Macro Implementation
//!
//! The `Deserialize` macro generates JSON deserialization methods with **cycle and
//! forward-reference support**, plus comprehensive runtime validation. This enables
//! safe parsing of complex JSON structures including circular references.
//!
//! ## Generated Output
//!
//! | Type | Generated Code | Description |
//! |------|----------------|-------------|
//! | Class | `classNameDeserialize(input)` + `static deserialize(input)` | Standalone function + static factory method |
//! | Enum | `enumNameDeserialize(input)`, `enumNameDeserializeWithContext(data)`, `enumNameIs(value)` | Standalone functions |
//! | Interface | `interfaceNameDeserialize(input)`, etc. | Standalone functions |
//! | Type Alias | `typeNameDeserialize(input)`, etc. | Standalone functions |
//!
//! ## Return Type
//!
//! All public deserialization methods return `Result<T, Array<{ field: string; message: string }>>`:
//!
//! - `Result.ok(value)` - Successfully deserialized value
//! - `Result.err(errors)` - Array of validation errors with field names and messages
//!
//! ## Cycle/Forward-Reference Support
//!
//! Uses deferred patching to handle references:
//!
//! 1. When encountering `{ "__ref": id }`, returns a `PendingRef` marker
//! 2. Continues deserializing other fields
//! 3. After all objects are created, `ctx.applyPatches()` resolves all pending references
//!
//! References only apply to object-shaped, serializable values. The generator avoids probing for
//! `__ref` on primitive-like fields (including literal unions and `T | null` where `T` is primitive-like),
//! and it parses `Date` / `Date | null` from ISO strings without treating them as references.
//!
//! ## Validation
//!
//! The macro supports 30+ validators via `@serde(validate(...))`:
//!
//! ### String Validators
//! - `email`, `url`, `uuid` - Format validation
//! - `minLength(n)`, `maxLength(n)`, `length(n)` - Length constraints
//! - `pattern("regex")` - Regular expression matching
//! - `nonEmpty`, `trimmed`, `lowercase`, `uppercase` - String properties
//!
//! ### Number Validators
//! - `gt(n)`, `gte(n)`, `lt(n)`, `lte(n)`, `between(min, max)` - Range checks
//! - `int`, `positive`, `nonNegative`, `finite` - Number properties
//!
//! ### Array Validators
//! - `minItems(n)`, `maxItems(n)`, `itemsCount(n)` - Collection size
//!
//! ### Date Validators
//! - `validDate`, `afterDate("ISO")`, `beforeDate("ISO")` - Date validation
//!
//! ## Field-Level Options
//!
//! The `@serde` decorator supports:
//!
//! - `skip` / `skipDeserializing` - Exclude field from deserialization
//! - `rename = "jsonKey"` - Read from different JSON property
//! - `default` / `default = expr` - Use default value if missing
//! - `flatten` - Read fields from parent object level
//! - `validate(...)` - Apply validators
//!
//! ## Container-Level Options
//!
//! - `denyUnknownFields` - Error on unrecognized JSON properties
//! - `renameAll = "camelCase"` - Apply naming convention to all fields
//!
//! ## Union Type Deserialization
//!
//! Union types are deserialized based on their member types:
//!
//! ### Literal Unions
//! For unions of literal values (`"A" | "B" | 123`), the value is validated against
//! the allowed literals directly.
//!
//! ### Primitive Unions
//! For unions containing primitive types (`string | number`), the deserializer uses
//! `typeof` checks to validate the value type. No `__type` discriminator is needed.
//!
//! ### Class/Interface Unions
//! For unions of serializable types (`User | Admin`), the deserializer requires a
//! `__type` field in the JSON to dispatch to the correct type's `deserializeWithContext` method.
//!
//! ### Generic Type Parameters
//! For generic unions like `type Result<T> = T | Error`, the generic type parameter `T`
//! is passed through as-is since its concrete type is only known at the call site.
//!
//! ### Mixed Unions
//! Mixed unions (e.g., `string | Date | User`) check in order:
//! 1. Literal values
//! 2. Primitives (via `typeof`)
//! 3. Date (via `instanceof` or ISO string parsing)
//! 4. Serializable types (via `__type` dispatch)
//! 5. Generic type parameters (pass-through)
//!
//! ## Example
//!
//! ```typescript
//! /** @derive(Deserialize) @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//!
//!     /** @serde({ validate: { email: true, maxLength: 255 } }) */
//!     email: string;
//!
//!     /** @serde({ default: "guest" }) */
//!     name: string;
//!
//!     /** @serde({ validate: { positive: true } }) */
//!     age?: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! class User {
//!     id: number;
//! 
//!     email: string;
//! 
//!     name: string;
//! 
//!     age?: number;
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//!
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//!
//!     email: string;
//!
//!     name: string;
//!
//!     age?: number;
//!
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//!
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: DeserializeOptions
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//!
//!             const ctx = DeserializeContext.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (PendingRef.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof DeserializeError) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//!
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new DeserializeError([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = "guest";
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         return instance;
//!     }
//!
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//!
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//!
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//! 
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//! 
//!     email: string;
//! 
//!     name: string;
//! 
//!     age?: number;
//! 
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//! 
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: DeserializeOptions
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//! 
//!             const ctx = DeserializeContext.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (PendingRef.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof DeserializeError) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//! 
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new DeserializeError([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = 'guest';
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         return instance;
//!     }
//! 
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//! 
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//! 
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//! 
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//! 
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//!
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//!
//!     email: string;
//!
//!     name: string;
//!
//!     age?: number;
//!
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//!
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: DeserializeOptions
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//!
//!             const ctx = DeserializeContext.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (PendingRef.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof DeserializeError) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//!
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new DeserializeError([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = "guest";
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         return instance;
//!     }
//!
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//!
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//!
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//! 
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//! 
//!     email: string;
//! 
//!     name: string;
//! 
//!     age?: number;
//! 
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//! 
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: DeserializeOptions
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//! 
//!             const ctx = DeserializeContext.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (PendingRef.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof DeserializeError) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//! 
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new DeserializeError([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = 'guest';
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         return instance;
//!     }
//! 
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//! 
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//! 
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//! 
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//! 
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//!
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//!
//!     email: string;
//!
//!     name: string;
//!
//!     age?: number;
//!
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//!
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: DeserializeOptions
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//!
//!             const ctx = DeserializeContext.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (PendingRef.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof DeserializeError) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//!
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new DeserializeError([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = "guest";
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         return instance;
//!     }
//!
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//!
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//!
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//! 
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//! 
//!     email: string;
//! 
//!     name: string;
//! 
//!     age?: number;
//! 
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//! 
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: DeserializeOptions
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//! 
//!             const ctx = DeserializeContext.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (PendingRef.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof DeserializeError) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//! 
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new DeserializeError([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = 'guest';
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         return instance;
//!     }
//! 
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//! 
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//! 
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//! 
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//! 
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//!
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//!
//!     email: string;
//!
//!     name: string;
//!
//!     age?: number;
//!
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//!
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: DeserializeOptions
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//!
//!             const ctx = DeserializeContext.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (PendingRef.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof DeserializeError) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//!
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new DeserializeError([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = 'guest';
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         return instance;
//!     }
//!
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//!
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//!
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//!
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! Generated output:
//!
//! ```typescript
//! import { DeserializeContext } from 'macroforge/serde';
//! import { DeserializeError } from 'macroforge/serde';
//! import type { DeserializeOptions } from 'macroforge/serde';
//! import { PendingRef } from 'macroforge/serde';
//! 
//! /** @serde({ denyUnknownFields: true }) */
//! class User {
//!     id: number;
//! 
//!     email: string;
//! 
//!     name: string;
//! 
//!     age?: number;
//! 
//!     constructor(props: {
//!         id: number;
//!         email: string;
//!         name?: string;
//!         age?: number;
//!     }) {
//!         this.id = props.id;
//!         this.email = props.email;
//!         this.name = props.name as string;
//!         this.age = props.age as number;
//!     }
//! 
//!     /**
//!      * Deserializes input to an instance of this class.
//!      * Automatically detects whether input is a JSON string or object.
//!      * @param input - JSON string or object to deserialize
//!      * @param opts - Optional deserialization options
//!      * @returns Result containing the deserialized instance or validation errors
//!      */
//!     static deserialize(
//!         input: unknown,
//!         opts?: DeserializeOptions
//!     ): Result<
//!         User,
//!         Array<{
//!             field: string;
//!             message: string;
//!         }>
//!     > {
//!         try {
//!             // Auto-detect: if string, parse as JSON first
//!             const data = typeof input === 'string' ? JSON.parse(input) : input;
//! 
//!             const ctx = DeserializeContext.create();
//!             const resultOrRef = User.deserializeWithContext(data, ctx);
//!             if (PendingRef.is(resultOrRef)) {
//!                 return Result.err([
//!                     {
//!                         field: '_root',
//!                         message: 'User.deserialize: root cannot be a forward reference'
//!                     }
//!                 ]);
//!             }
//!             ctx.applyPatches();
//!             if (opts?.freeze) {
//!                 ctx.freezeAll();
//!             }
//!             return Result.ok(resultOrRef);
//!         } catch (e) {
//!             if (e instanceof DeserializeError) {
//!                 return Result.err(e.errors);
//!             }
//!             const message = e instanceof Error ? e.message : String(e);
//!             return Result.err([
//!                 {
//!                     field: '_root',
//!                     message
//!                 }
//!             ]);
//!         }
//!     }
//! 
//!     /** @internal */
//!     static deserializeWithContext(value: any, ctx: DeserializeContext): User | PendingRef {
//!         if (value?.__ref !== undefined) {
//!             return ctx.getOrDefer(value.__ref);
//!         }
//!         if (typeof value !== 'object' || value === null || Array.isArray(value)) {
//!             throw new DeserializeError([
//!                 {
//!                     field: '_root',
//!                     message: 'User.deserializeWithContext: expected an object'
//!                 }
//!             ]);
//!         }
//!         const obj = value as Record<string, unknown>;
//!         const errors: Array<{
//!             field: string;
//!             message: string;
//!         }> = [];
//!         const knownKeys = new Set(['__type', '__id', '__ref', 'id', 'email', 'name', 'age']);
//!         for (const key of Object.keys(obj)) {
//!             if (!knownKeys.has(key)) {
//!                 errors.push({
//!                     field: key,
//!                     message: 'unknown field'
//!                 });
//!             }
//!         }
//!         if (!('id' in obj)) {
//!             errors.push({
//!                 field: 'id',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (!('email' in obj)) {
//!             errors.push({
//!                 field: 'email',
//!                 message: 'missing required field'
//!             });
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         const instance = Object.create(User.prototype) as User;
//!         if (obj.__id !== undefined) {
//!             ctx.register(obj.__id as number, instance);
//!         }
//!         ctx.trackForFreeze(instance);
//!         {
//!             const __raw_id = obj['id'] as number;
//!             instance.id = __raw_id;
//!         }
//!         {
//!             const __raw_email = obj['email'] as string;
//!             instance.email = __raw_email;
//!         }
//!         if ('name' in obj && obj['name'] !== undefined) {
//!             const __raw_name = obj['name'] as string;
//!             instance.name = __raw_name;
//!         } else {
//!             instance.name = 'guest';
//!         }
//!         if ('age' in obj && obj['age'] !== undefined) {
//!             const __raw_age = obj['age'] as number;
//!             instance.age = __raw_age;
//!         }
//!         if (errors.length > 0) {
//!             throw new DeserializeError(errors);
//!         }
//!         return instance;
//!     }
//! 
//!     static validateField<K extends keyof User>(
//!         field: K,
//!         value: User[K]
//!     ): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//! 
//!     static validateFields(partial: Partial<User>): Array<{
//!         field: string;
//!         message: string;
//!     }> {
//!         return [];
//!     }
//! 
//!     static hasShape(obj: unknown): boolean {
//!         if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
//!             return false;
//!         }
//!         const o = obj as Record<string, unknown>;
//!         return 'id' in o && 'email' in o;
//!     }
//! 
//!     static is(obj: unknown): obj is User {
//!         if (obj instanceof User) {
//!             return true;
//!         }
//!         if (!User.hasShape(obj)) {
//!             return false;
//!         }
//!         const result = User.deserialize(obj);
//!         return Result.isOk(result);
//!     }
//! }
//! 
//! // Usage:
//! const result = User.deserialize('{"id":1,"email":"test@example.com"}');
//! if (Result.isOk(result)) {
//!     const user = result.value;
//! } else {
//!     console.error(result.error); // [{ field: "email", message: "must be a valid email" }]
//! }
//! ```
//!
//! ## Required Imports
//!
//! The generated code automatically imports:
//! - `Result` from `macroforge/utils`
//! - `DeserializeContext`, `DeserializeError`, `PendingRef` from `macroforge/serde`

use crate::macros::{body, ts_macro_derive, ts_template};
use crate::ts_syn::abi::DiagnosticCollector;
use crate::ts_syn::{
    Data, DeriveInput, MacroforgeError, MacroforgeErrors, TsStream, parse_ts_macro_input,
};

use super::{SerdeContainerOptions, SerdeFieldOptions, TypeCategory, Validator, ValidatorSpec, get_foreign_types};

/// Convert a PascalCase name to camelCase (for prefix naming style)
fn to_camel_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn nested_deserialize_fn_name(type_name: &str) -> String {
    format!("{}DeserializeWithContext", to_camel_case(type_name))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SerdeValueKind {
    PrimitiveLike,
    Date,
    NullableDate,
    Other,
}

fn is_ts_primitive_keyword(s: &str) -> bool {
    matches!(
        s.trim(),
        "string" | "number" | "boolean" | "bigint" | "null" | "undefined"
    )
}

fn is_ts_literal(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }

    if matches!(s, "true" | "false") {
        return true;
    }

    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        return true;
    }

    // Very small heuristic: numeric / bigint literals
    if let Some(digits) = s.strip_suffix('n') {
        return !digits.is_empty()
            && digits
                .chars()
                .all(|c| c.is_ascii_digit() || c == '_' || c == '-' || c == '+');
    }

    s.chars()
        .all(|c| c.is_ascii_digit() || c == '_' || c == '-' || c == '+' || c == '.')
}

fn is_union_of_primitive_like(s: &str) -> bool {
    if !s.contains('|') {
        return false;
    }
    s.split('|').all(|part| {
        let part = part.trim();
        is_ts_primitive_keyword(part) || is_ts_literal(part)
    })
}

fn classify_serde_value_kind(ts_type: &str) -> SerdeValueKind {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Primitive => SerdeValueKind::PrimitiveLike,
        TypeCategory::Date => SerdeValueKind::Date,
        TypeCategory::Nullable(inner) => match classify_serde_value_kind(&inner) {
            SerdeValueKind::Date => SerdeValueKind::NullableDate,
            SerdeValueKind::PrimitiveLike => SerdeValueKind::PrimitiveLike,
            _ => SerdeValueKind::Other,
        },
        TypeCategory::Optional(inner) => classify_serde_value_kind(&inner),
        _ => {
            if is_union_of_primitive_like(ts_type) {
                SerdeValueKind::PrimitiveLike
            } else {
                SerdeValueKind::Other
            }
        }
    }
}

/// If the given type string is a Serializable type, return its name.
/// Returns None for primitives, Date, and other non-serializable types.
fn get_serializable_type_name(ts_type: &str) -> Option<String> {
    match TypeCategory::from_ts_type(ts_type) {
        TypeCategory::Serializable(name) => Some(name),
        _ => None,
    }
}

/// Contains field information needed for JSON deserialization code generation.
///
/// Each field that should be deserialized is represented by this struct,
/// capturing all the information needed to generate parsing, validation,
/// and assignment code.
#[derive(Clone)]
struct DeserializeField {
    /// The JSON property name to read from the input object.
    /// This may differ from `field_name` if `@serde(rename = "...")` is used.
    json_key: String,

    /// The TypeScript field name as it appears in the source class.
    /// Used for generating property assignments like `instance.fieldName = value`.
    field_name: String,

    /// The TypeScript type annotation string (e.g., "string", "number[]").
    /// Used for type casting in generated code.
    #[allow(dead_code)]
    ts_type: String,

    /// The category of the field's type, used to select the appropriate
    /// deserialization strategy (primitive, Date, Array, Map, Set, etc.).
    type_cat: TypeCategory,

    /// Whether the field is optional (has `?` modifier or `@serde(default)`).
    /// Optional fields don't require the JSON property to be present.
    optional: bool,

    /// Whether the field has a default value specified.
    #[allow(dead_code)]
    has_default: bool,

    /// The default value expression to use if the field is missing.
    /// Example: `Some("\"guest\"".to_string())` for `@serde(default = "guest")`.
    default_expr: Option<String>,

    /// Whether the field should be read from the parent object level.
    /// Flattened fields look for their properties directly in the parent JSON.
    flatten: bool,

    /// List of validators to apply after parsing the field value.
    /// Each validator generates a condition check and error message.
    validators: Vec<ValidatorSpec>,

    /// For `T | null` unions: classification of `T`.
    nullable_inner_kind: Option<SerdeValueKind>,
    /// For `Array<T>` and `T[]`: classification of `T`.
    array_elem_kind: Option<SerdeValueKind>,

    // --- Serializable type tracking for direct function calls ---
    /// For `T | null` where T is Serializable: the type name.
    nullable_serializable_type: Option<String>,

    /// Custom deserialization function name (from `@serde({deserializeWith: "fn"})`)
    /// When set, this function is called instead of type-based deserialization.
    deserialize_with: Option<String>,
}

impl DeserializeField {
    /// Returns true if this field has any validators that need to be applied.
    fn has_validators(&self) -> bool {
        !self.validators.is_empty()
    }
}

/// Holds information about a serializable type reference in a union.
///
/// For parameterized types like `RecordLink<Product>`, we need both:
/// - The full type string for `__type` comparison and type casting
/// - The base type name for runtime namespace access
#[derive(Clone)]
struct SerializableTypeRef {
    /// The full type reference string (e.g., "RecordLink<Product>")
    full_type: String,
    /// The base type name without generic parameters (e.g., "RecordLink")
    base_type: String,
}

/// Extracts the base type name from a potentially parameterized type reference.
///
/// # Examples
///
/// - `"RecordLink<Product>"` → `"RecordLink"`
/// - `"User"` → `"User"`
/// - `"Map<string, number>"` → `"Map"`
fn extract_base_type(type_ref: &str) -> String {
    if let Some(pos) = type_ref.find('<') {
        type_ref[..pos].to_string()
    } else {
        type_ref.to_string()
    }
}

/// Generates a JavaScript boolean expression that evaluates to `true` when validation fails.
///
/// This function produces the *failure condition* - the expression should be used in
/// `if (condition) { errors.push(...) }` to detect invalid values.
///
/// # Arguments
///
/// * `validator` - The validator type to generate a condition for
/// * `value_var` - The variable name containing the value to validate
///
/// # Returns
///
/// A string containing a JavaScript boolean expression. The expression evaluates to
/// `true` when the value is **invalid** (fails validation).
///
/// # Example
///
/// ```rust
/// use macroforge_ts::builtin::serde::Validator;
/// use macroforge_ts::builtin::serde::derive_deserialize::generate_validation_condition;
///
/// let condition = generate_validation_condition(&Validator::Email, "email");
/// assert!(condition.contains("test(email)"));
///
/// let condition = generate_validation_condition(&Validator::MaxLength(100), "name");
/// assert_eq!(condition, "name.length > 100");
/// ```
pub fn generate_validation_condition(validator: &Validator, value_var: &str) -> String {
    match validator {
        // String validators
        Validator::Email => {
            format!(r#"!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test({value_var})"#)
        }
        Validator::Url => {
            format!(
                r#"(() => {{ try {{ new URL({value_var}); return false; }} catch {{ return true; }} }})()"#
            )
        }
        Validator::Uuid => {
            format!(
                r#"!/^[0-9a-f]{{8}}-[0-9a-f]{{4}}-[1-5][0-9a-f]{{3}}-[89ab][0-9a-f]{{3}}-[0-9a-f]{{12}}$/i.test({value_var})"#
            )
        }
        Validator::MaxLength(n) => format!("{value_var}.length > {n}"),
        Validator::MinLength(n) => format!("{value_var}.length < {n}"),
        Validator::Length(n) => format!("{value_var}.length !== {n}"),
        Validator::LengthRange(min, max) => {
            format!("{value_var}.length < {min} || {value_var}.length > {max}")
        }
        Validator::Pattern(regex) => {
            let escaped = regex.replace('\\', "\\\\");
            format!("!/{escaped}/.test({value_var})")
        }
        Validator::NonEmpty => format!("{value_var}.length === 0"),
        Validator::Trimmed => format!("{value_var} !== {value_var}.trim()"),
        Validator::Lowercase => format!("{value_var} !== {value_var}.toLowerCase()"),
        Validator::Uppercase => format!("{value_var} !== {value_var}.toUpperCase()"),
        Validator::Capitalized => {
            format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toUpperCase()")
        }
        Validator::Uncapitalized => {
            format!("{value_var}.length > 0 && {value_var}[0] !== {value_var}[0].toLowerCase()")
        }
        Validator::StartsWith(prefix) => format!(r#"!{value_var}.startsWith("{prefix}")"#),
        Validator::EndsWith(suffix) => format!(r#"!{value_var}.endsWith("{suffix}")"#),
        Validator::Includes(substr) => format!(r#"!{value_var}.includes("{substr}")"#),

        // Number validators
        Validator::GreaterThan(n) => format!("{value_var} <= {n}"),
        Validator::GreaterThanOrEqualTo(n) => format!("{value_var} < {n}"),
        Validator::LessThan(n) => format!("{value_var} >= {n}"),
        Validator::LessThanOrEqualTo(n) => format!("{value_var} > {n}"),
        Validator::Between(min, max) => format!("{value_var} < {min} || {value_var} > {max}"),
        Validator::Int => format!("!Number.isInteger({value_var})"),
        Validator::NonNaN => format!("Number.isNaN({value_var})"),
        Validator::Finite => format!("!Number.isFinite({value_var})"),
        Validator::Positive => format!("{value_var} <= 0"),
        Validator::NonNegative => format!("{value_var} < 0"),
        Validator::Negative => format!("{value_var} >= 0"),
        Validator::NonPositive => format!("{value_var} > 0"),
        Validator::MultipleOf(n) => format!("{value_var} % {n} !== 0"),
        Validator::Uint8 => {
            format!("!Number.isInteger({value_var}) || {value_var} < 0 || {value_var} > 255")
        }

        // Array validators
        Validator::MaxItems(n) => format!("{value_var}.length > {n}"),
        Validator::MinItems(n) => format!("{value_var}.length < {n}"),
        Validator::ItemsCount(n) => format!("{value_var}.length !== {n}"),

        // Date validators (null-safe: JSON.stringify converts Invalid Date to null)
        Validator::ValidDate => format!("{value_var} == null || isNaN({value_var}.getTime())"),
        Validator::GreaterThanDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() <= new Date("{date}").getTime()"#
            )
        }
        Validator::GreaterThanOrEqualToDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() < new Date("{date}").getTime()"#
            )
        }
        Validator::LessThanDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() >= new Date("{date}").getTime()"#
            )
        }
        Validator::LessThanOrEqualToDate(date) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() > new Date("{date}").getTime()"#
            )
        }
        Validator::BetweenDate(min, max) => {
            format!(
                r#"{value_var} == null || {value_var}.getTime() < new Date("{min}").getTime() || {value_var}.getTime() > new Date("{max}").getTime()"#
            )
        }

        // BigInt validators
        Validator::GreaterThanBigInt(n) => format!("{value_var} <= BigInt({n})"),
        Validator::GreaterThanOrEqualToBigInt(n) => format!("{value_var} < BigInt({n})"),
        Validator::LessThanBigInt(n) => format!("{value_var} >= BigInt({n})"),
        Validator::LessThanOrEqualToBigInt(n) => format!("{value_var} > BigInt({n})"),
        Validator::BetweenBigInt(min, max) => {
            format!("{value_var} < BigInt({min}) || {value_var} > BigInt({max})")
        }
        Validator::PositiveBigInt => format!("{value_var} <= 0n"),
        Validator::NonNegativeBigInt => format!("{value_var} < 0n"),
        Validator::NegativeBigInt => format!("{value_var} >= 0n"),
        Validator::NonPositiveBigInt => format!("{value_var} > 0n"),

        // Custom validator - handled specially
        Validator::Custom(_) => String::new(),
    }
}

/// Returns the default human-readable error message for a validator.
///
/// These messages are used when no custom message is provided via
/// `@serde(validate(..., message = "custom"))`.
///
/// # Arguments
///
/// * `validator` - The validator to get a message for
///
/// # Returns
///
/// A user-friendly error message describing the validation requirement.
///
/// # Example Messages
///
/// - `Validator::Email` → "must be a valid email"
/// - `Validator::MaxLength(100)` → "must have at most 100 characters"
/// - `Validator::Between(1, 10)` → "must be between 1 and 10"
fn get_validator_message(validator: &Validator) -> String {
    match validator {
        Validator::Email => "must be a valid email".to_string(),
        Validator::Url => "must be a valid URL".to_string(),
        Validator::Uuid => "must be a valid UUID".to_string(),
        Validator::MaxLength(n) => format!("must have at most {n} characters"),
        Validator::MinLength(n) => format!("must have at least {n} characters"),
        Validator::Length(n) => format!("must have exactly {n} characters"),
        Validator::LengthRange(min, max) => {
            format!("must have between {min} and {max} characters")
        }
        Validator::Pattern(_) => "must match the required pattern".to_string(),
        Validator::NonEmpty => "must not be empty".to_string(),
        Validator::Trimmed => "must be trimmed (no leading/trailing whitespace)".to_string(),
        Validator::Lowercase => "must be lowercase".to_string(),
        Validator::Uppercase => "must be uppercase".to_string(),
        Validator::Capitalized => "must be capitalized".to_string(),
        Validator::Uncapitalized => "must not be capitalized".to_string(),
        Validator::StartsWith(s) => format!("must start with '{s}'"),
        Validator::EndsWith(s) => format!("must end with '{s}'"),
        Validator::Includes(s) => format!("must include '{s}'"),
        Validator::GreaterThan(n) => format!("must be greater than {n}"),
        Validator::GreaterThanOrEqualTo(n) => format!("must be greater than or equal to {n}"),
        Validator::LessThan(n) => format!("must be less than {n}"),
        Validator::LessThanOrEqualTo(n) => format!("must be less than or equal to {n}"),
        Validator::Between(min, max) => format!("must be between {min} and {max}"),
        Validator::Int => "must be an integer".to_string(),
        Validator::NonNaN => "must not be NaN".to_string(),
        Validator::Finite => "must be finite".to_string(),
        Validator::Positive => "must be positive".to_string(),
        Validator::NonNegative => "must be non-negative".to_string(),
        Validator::Negative => "must be negative".to_string(),
        Validator::NonPositive => "must be non-positive".to_string(),
        Validator::MultipleOf(n) => format!("must be a multiple of {n}"),
        Validator::Uint8 => "must be a uint8 (0-255)".to_string(),
        Validator::MaxItems(n) => format!("must have at most {n} items"),
        Validator::MinItems(n) => format!("must have at least {n} items"),
        Validator::ItemsCount(n) => format!("must have exactly {n} items"),
        Validator::ValidDate => "must be a valid date".to_string(),
        Validator::GreaterThanDate(d) => format!("must be after {d}"),
        Validator::GreaterThanOrEqualToDate(d) => format!("must be on or after {d}"),
        Validator::LessThanDate(d) => format!("must be before {d}"),
        Validator::LessThanOrEqualToDate(d) => format!("must be on or before {d}"),
        Validator::BetweenDate(min, max) => format!("must be between {min} and {max}"),
        Validator::GreaterThanBigInt(n) => format!("must be greater than {n}"),
        Validator::GreaterThanOrEqualToBigInt(n) => format!("must be greater than or equal to {n}"),
        Validator::LessThanBigInt(n) => format!("must be less than {n}"),
        Validator::LessThanOrEqualToBigInt(n) => format!("must be less than or equal to {n}"),
        Validator::BetweenBigInt(min, max) => format!("must be between {min} and {max}"),
        Validator::PositiveBigInt => "must be positive".to_string(),
        Validator::NonNegativeBigInt => "must be non-negative".to_string(),
        Validator::NegativeBigInt => "must be negative".to_string(),
        Validator::NonPositiveBigInt => "must be non-positive".to_string(),
        Validator::Custom(_) => "failed custom validation".to_string(),
    }
}

/// Generates JavaScript code that validates a field and collects errors.
///
/// This function produces a series of `if` statements that check each validator
/// and push error objects to the `errors` array when validation fails.
///
/// # Arguments
///
/// * `validators` - List of validators to apply, each with optional custom message
/// * `value_var` - The variable name containing the value to validate
/// * `json_key` - The JSON property name (used in error messages)
/// * `_class_name` - The class name (reserved for future use)
///
/// # Returns
///
/// A string containing JavaScript code that performs validation checks.
/// The generated code assumes an `errors` array is in scope.
///
/// # Example Output
///
/// ```javascript
/// if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
///     errors.push({ field: "email", message: "must be a valid email" });
/// }
/// if (email.length > 255) {
///     errors.push({ field: "email", message: "must have at most 255 characters" });
/// }
/// ```
fn generate_field_validations(
    validators: &[ValidatorSpec],
    value_var: &str,
    json_key: &str,
    _class_name: &str,
) -> String {
    let mut code = String::new();

    for spec in validators {
        let message = spec
            .custom_message
            .clone()
            .unwrap_or_else(|| get_validator_message(&spec.validator));

        if let Validator::Custom(fn_name) = &spec.validator {
            code.push_str(&format!(
                r#"
                {{
                    const __customResult = {fn_name}({value_var});
                    if (__customResult === false) {{
                        errors.push({{ field: "{json_key}", message: "{message}" }});
                    }}
                }}
"#
            ));
        } else {
            let condition = generate_validation_condition(&spec.validator, value_var);
            code.push_str(&format!(
                r#"
                if ({condition}) {{
                    errors.push({{ field: "{json_key}", message: "{message}" }});
                }}
"#
            ));
        }
    }

    code
}

#[ts_macro_derive(
    Deserialize,
    description = "Generates deserialization methods with cycle/forward-reference support (fromStringifiedJSON, deserializeWithContext)",
    attributes((serde, "Configure deserialization for this field. Options: skip, rename, flatten, default, validate"))
)]
pub fn derive_deserialize_macro(mut input: TsStream) -> Result<TsStream, MacroforgeError> {
    let input = parse_ts_macro_input!(input as DeriveInput);

    match &input.data {
        Data::Class(class) => {
            let class_name = input.name();
            let container_opts = SerdeContainerOptions::from_decorators(&class.inner.decorators);

            // Generate function names (always prefix style)
            let fn_deserialize = format!("{}Deserialize", to_camel_case(class_name));
            let fn_deserialize_internal = format!("{}DeserializeWithContext", to_camel_case(class_name));
            let fn_is = format!("{}Is", to_camel_case(class_name));

            // Check for user-defined constructor with parameters
            if let Some(ctor) = class.method("constructor")
                && !ctor.params_src.trim().is_empty()
            {
                return Err(MacroforgeError::new(
                    ctor.span,
                    format!(
                        "@Derive(Deserialize) cannot be used on class '{}' with a custom constructor. \
                            Remove the constructor or use @Derive(Deserialize) on a class without a constructor.",
                        class_name
                    ),
                ));
            }

            // Collect deserializable fields with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<DeserializeField> = class
                .fields()
                .iter()
                .filter_map(|field| {
                    let parse_result =
                        SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                    all_diagnostics.extend(parse_result.diagnostics);
                    let opts = parse_result.options;

                    if !opts.should_deserialize() {
                        return None;
                    }

                    let json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    let nullable_inner_kind = match &type_cat {
                        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let array_elem_kind = match &type_cat {
                        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };

                    // Extract serializable type names for direct function calls
                    let nullable_serializable_type = match &type_cat {
                        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };

                    // Check for foreign type deserializer if no explicit deserialize_with
                    let deserialize_with = if opts.deserialize_with.is_some() {
                        opts.deserialize_with.clone()
                    } else {
                        // Check if the field's type matches a configured foreign type
                        let foreign_types = get_foreign_types();
                        TypeCategory::match_foreign_type(&field.ts_type, &foreign_types)
                            .and_then(|ft| ft.deserialize_expr.clone())
                    };

                    Some(DeserializeField {
                        json_key,
                        field_name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                        type_cat,
                        optional: field.optional || opts.default || opts.default_expr.is_some(),
                        has_default: opts.default || opts.default_expr.is_some(),
                        default_expr: opts.default_expr.clone(),
                        flatten: opts.flatten,
                        validators: opts.validators.clone(),
                        nullable_inner_kind,
                        array_elem_kind,
                        nullable_serializable_type,
                        deserialize_with,
                    })
                })
                .collect();

            // Check for errors in field parsing before continuing
            if all_diagnostics.has_errors() {
                return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
            }

            // Separate required vs optional fields
            let required_fields: Vec<_> = fields
                .iter()
                .filter(|f| !f.optional && !f.flatten)
                .cloned()
                .collect();
            let optional_fields: Vec<_> = fields
                .iter()
                .filter(|f| f.optional && !f.flatten)
                .cloned()
                .collect();
            let flatten_fields: Vec<_> = fields.iter().filter(|f| f.flatten).cloned().collect();

            // Build known keys for deny_unknown_fields
            let known_keys: Vec<String> = fields
                .iter()
                .filter(|f| !f.flatten)
                .map(|f| f.json_key.clone())
                .collect();

            let has_required = !required_fields.is_empty();
            let _has_optional = !optional_fields.is_empty();
            let has_flatten = !flatten_fields.is_empty();
            let deny_unknown = container_opts.deny_unknown_fields;

            // All non-flatten fields for assignments
            let all_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let has_fields = !all_fields.is_empty();

            // Fields with validators for per-field validation
            let fields_with_validators: Vec<_> = all_fields
                .iter()
                .filter(|f| f.has_validators())
                .cloned()
                .collect();
            let has_validators = !fields_with_validators.is_empty();

            // Generate shape check condition for hasShape method
            let shape_check_condition: String = if required_fields.is_empty() {
                "true".to_string()
            } else {
                required_fields
                    .iter()
                    .map(|f| format!("\"{}\" in o", f.json_key))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            let mut result = body! {
                constructor(props: { {#for field in &all_fields} @{field.field_name}{#if field.optional}?{/if}: @{field.ts_type}; {/for} }) {
                    {#for field in &all_fields}
                        this.@{field.field_name} = props.@{field.field_name}{#if field.optional} as @{field.ts_type}{/if};
                    {/for}
                }

                {>> "Deserializes input to an instance of this class.\nAutomatically detects whether input is a JSON string or object.\n@param input - JSON string or object to deserialize\n@param opts - Optional deserialization options\n@returns Result containing the deserialized instance or validation errors" <<}
                static deserialize(input: unknown, opts?: DeserializeOptions): Result<@{class_name}, Array<{ field: string; message: string }>> {
                    try {
                        // Auto-detect: if string, parse as JSON first
                        const data = typeof input === "string" ? JSON.parse(input) : input;

                        const ctx = DeserializeContext.create();
                        const resultOrRef = @{class_name}.deserializeWithContext(data, ctx);

                        if (PendingRef.is(resultOrRef)) {
                            return Result.err([{ field: "_root", message: "@{class_name}.deserialize: root cannot be a forward reference" }]);
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
                        return Result.err([{ field: "_root", message }]);
                    }
                }

                {>> "Deserializes with an existing context for nested/cyclic object graphs.\n@param value - The raw value to deserialize\n@param ctx - The deserialization context" <<}
                static deserializeWithContext(value: any, ctx: DeserializeContext): @{class_name} | PendingRef {
                    // Handle reference to already-deserialized object
                    if (value?.__ref !== undefined) {
                        return ctx.getOrDefer(value.__ref);
                    }

                    if (typeof value !== "object" || value === null || Array.isArray(value)) {
                        throw new DeserializeError([{ field: "_root", message: "@{class_name}.deserializeWithContext: expected an object" }]);
                    }

                    const obj = value as Record<string, unknown>;
                    const errors: Array<{ field: string; message: string }> = [];

                    {#if deny_unknown}
                        const knownKeys = new Set(["__type", "__id", "__ref", {#for key in known_keys}"@{key}", {/for}]);
                        for (const key of Object.keys(obj)) {
                            if (!knownKeys.has(key)) {
                                errors.push({ field: key, message: "unknown field" });
                            }
                        }
                    {/if}

                    {#if has_required}
                        {#for field in &required_fields}
                            if (!("@{field.json_key}" in obj)) {
                                errors.push({ field: "@{field.json_key}", message: "missing required field" });
                            }
                        {/for}
                    {/if}

                    if (errors.length > 0) {
                        throw new DeserializeError(errors);
                    }

                    // Create instance using Object.create to avoid constructor
                    const instance = Object.create(@{class_name}.prototype) as @{class_name};

                    // Register with context if __id is present
                    if (obj.__id !== undefined) {
                        ctx.register(obj.__id as number, instance);
                    }

                    // Track for optional freezing
                    ctx.trackForFreeze(instance);

                    // Assign fields
                    {#if has_fields}
                        {#for field in all_fields}
                            {$let raw_var = format!("__raw_{}", field.field_name)}
                            {$let has_validators = field.has_validators()}
                            {#if let Some(fn_name) = &field.deserialize_with}
                                // Custom deserialization function (deserializeWith)
                                {#if field.optional}
                                    if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                        instance.@{field.field_name} = @{fn_name}(obj["@{field.json_key}"]);
                                    }
                                {:else}
                                    instance.@{field.field_name} = @{fn_name}(obj["@{field.json_key}"]);
                                {/if}
                            {:else}
                            {#if field.optional}
                                if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                    const @{raw_var} = obj["@{field.json_key}"] as @{field.ts_type};
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Primitive}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                @{validation_code}
                                            {/if}
                                            instance.@{field.field_name} = @{raw_var};

                                        {:case TypeCategory::Date}
                                            {
                                                const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}
                                                instance.@{field.field_name} = __dateVal;
                                            }

                                        {:case TypeCategory::Array(inner)}
                                            if (Array.isArray(@{raw_var})) {
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}

                                                {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                    {:case SerdeValueKind::Date}
                                                        instance.@{field.field_name} = (@{raw_var} as any[]).map(
                                                            (item) => typeof item === "string" ? new Date(item) : item as Date
                                                        ) as any;
                                                    {:case SerdeValueKind::NullableDate}
                                                        instance.@{field.field_name} = (@{raw_var} as any[]).map(
                                                            (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                                                        ) as any;
                                                    {:case _}
                                                        const __arr = (@{raw_var} as any[]).map((item, idx) => {
                                                            if (typeof item?.deserializeWithContext === "function") {
                                                                const result = item.deserializeWithContext(item, ctx);
                                                                if (PendingRef.is(result)) {
                                                                    return { __pendingIdx: idx, __refId: result.id };
                                                                }
                                                                return result;
                                                            }
                                                            // Check for __ref in array items
                                                            if (item?.__ref !== undefined) {
                                                                const result = ctx.getOrDefer(item.__ref);
                                                                if (PendingRef.is(result)) {
                                                                    return { __pendingIdx: idx, __refId: result.id };
                                                                }
                                                                return result;
                                                            }
                                                            return item as @{inner};
                                                        });
                                                        instance.@{field.field_name} = __arr;
                                                        // Patch array items that were pending
                                                        __arr.forEach((item, idx) => {
                                                            if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                                ctx.addPatch(instance.@{field.field_name}, idx, (item as any).__refId);
                                                            }
                                                        });
                                                {/match}
                                            }

                                        {:case TypeCategory::Map(key_type, value_type)}
                                            if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                instance.@{field.field_name} = new Map(
                                                    Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                );
                                            }

                                        {:case TypeCategory::Set(inner)}
                                            if (Array.isArray(@{raw_var})) {
                                                instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                            }

                                        {:case TypeCategory::Serializable(type_name)}
                                            {
                                                const __result = @{type_name}.deserializeWithContext(@{raw_var}, ctx);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field.field_name} = @{raw_var};
                                                {:case SerdeValueKind::Date}
                                                    if (@{raw_var} === null) {
                                                        instance.@{field.field_name} = null;
                                                    } else {
                                                        instance.@{field.field_name} = typeof @{raw_var} === "string"
                                                            ? new Date(@{raw_var} as any)
                                                            : @{raw_var} as any;
                                                    }
                                                {:case _}
                                                    if (@{raw_var} === null) {
                                                        instance.@{field.field_name} = null;
                                                    } else {
                                                        {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                            const __result = @{inner_type}.deserializeWithContext(@{raw_var}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                        {:else}
                                                            instance.@{field.field_name} = @{raw_var};
                                                        {/if}
                                                    }
                                            {/match}

                                        {:case _}
                                            instance.@{field.field_name} = @{raw_var};
                                    {/match}
                                }
                                {#if let Some(default_expr) = &field.default_expr}
                                    else {
                                        instance.@{field.field_name} = @{default_expr};
                                    }
                                {/if}
                            {:else}
                                {
                                    const @{raw_var} = obj["@{field.json_key}"] as @{field.ts_type};
                                    {#match &field.type_cat}
                                        {:case TypeCategory::Primitive}
                                            {#if has_validators}
                                                {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                @{validation_code}
                                            {/if}
                                            instance.@{field.field_name} = @{raw_var};

                                        {:case TypeCategory::Date}
                                            {
                                                const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}
                                                instance.@{field.field_name} = __dateVal;
                                            }

                                        {:case TypeCategory::Array(inner)}
                                            if (Array.isArray(@{raw_var})) {
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, class_name)}
                                                    @{validation_code}
                                                {/if}

                                                {#match field.array_elem_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                    {:case SerdeValueKind::Date}
                                                        instance.@{field.field_name} = (@{raw_var} as any[]).map(
                                                            (item) => typeof item === "string" ? new Date(item) : item as Date
                                                        ) as any;
                                                    {:case SerdeValueKind::NullableDate}
                                                        instance.@{field.field_name} = (@{raw_var} as any[]).map(
                                                            (item) => item === null ? null : (typeof item === "string" ? new Date(item) : item as Date)
                                                        ) as any;
                                                    {:case _}
                                                        const __arr = (@{raw_var} as any[]).map((item, idx) => {
                                                            if (item?.__ref !== undefined) {
                                                                const result = ctx.getOrDefer(item.__ref);
                                                                if (PendingRef.is(result)) {
                                                                    return { __pendingIdx: idx, __refId: result.id };
                                                                }
                                                                return result;
                                                            }
                                                            return item as @{inner};
                                                        });
                                                        instance.@{field.field_name} = __arr;
                                                        __arr.forEach((item, idx) => {
                                                            if (item && typeof item === "object" && "__pendingIdx" in item) {
                                                                ctx.addPatch(instance.@{field.field_name}, idx, (item as any).__refId);
                                                            }
                                                        });
                                                {/match}
                                            }

                                        {:case TypeCategory::Map(key_type, value_type)}
                                            instance.@{field.field_name} = new Map(
                                                Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                            );

                                        {:case TypeCategory::Set(inner)}
                                            instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);

                                        {:case TypeCategory::Serializable(type_name)}
                                            {
                                                const __result = @{type_name}.deserializeWithContext(@{raw_var}, ctx);
                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                            }

                                        {:case TypeCategory::Nullable(_)}
                                            {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                {:case SerdeValueKind::PrimitiveLike}
                                                    instance.@{field.field_name} = @{raw_var};
                                                {:case SerdeValueKind::Date}
                                                    if (@{raw_var} === null) {
                                                        instance.@{field.field_name} = null;
                                                    } else {
                                                        instance.@{field.field_name} = typeof @{raw_var} === "string"
                                                            ? new Date(@{raw_var} as any)
                                                            : @{raw_var} as any;
                                                    }
                                                {:case _}
                                                    if (@{raw_var} === null) {
                                                        instance.@{field.field_name} = null;
                                                    } else {
                                                        {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                            const __result = @{inner_type}.deserializeWithContext(@{raw_var}, ctx);
                                                            ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                        {:else}
                                                            instance.@{field.field_name} = @{raw_var};
                                                        {/if}
                                                    }
                                            {/match}

                                        {:case _}
                                            instance.@{field.field_name} = @{raw_var};
                                    {/match}
                                }
                            {/if}
                            {/if}
                        {/for}
                    {/if}

                    {#if has_flatten}
                        {#for field in flatten_fields}
                            {#match &field.type_cat}
                                {:case TypeCategory::Serializable(type_name)}
                                    {
                                        const __result = @{type_name}.deserializeWithContext(obj, ctx);
                                        ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                    }
                                {:case _}
                                    instance.@{field.field_name} = obj as any;
                            {/match}
                        {/for}
                    {/if}

                    if (errors.length > 0) {
                        throw new DeserializeError(errors);
                    }

                    return instance;
                }

                static validateField<K extends keyof @{class_name}>(
                    field: K,
                    value: @{class_name}[K]
                ): Array<{ field: string; message: string }> {
                    {#if has_validators}
                    const errors: Array<{ field: string; message: string }> = [];
                    switch (field) {
                        {#for field in &fields_with_validators}
                        case "@{field.field_name}": {
                            const __val = value as @{field.ts_type};
                            {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, class_name)}
                            @{validation_code}
                            break;
                        }
                        {/for}
                    }
                    return errors;
                    {:else}
                    return [];
                    {/if}
                }

                static validateFields(
                    partial: Partial<@{class_name}>
                ): Array<{ field: string; message: string }> {
                    {#if has_validators}
                    const errors: Array<{ field: string; message: string }> = [];
                    {#for field in &fields_with_validators}
                    if ("@{field.field_name}" in partial && partial.@{field.field_name} !== undefined) {
                        const __val = partial.@{field.field_name} as @{field.ts_type};
                        {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, class_name)}
                        @{validation_code}
                    }
                    {/for}
                    return errors;
                    {:else}
                    return [];
                    {/if}
                }

                static hasShape(obj: unknown): boolean {
                    if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                        return false;
                    }
                    const o = obj as Record<string, unknown>;
                    return @{shape_check_condition};
                }

                static is(obj: unknown): obj is @{class_name} {
                    if (obj instanceof @{class_name}) {
                        return true;
                    }
                    if (!@{class_name}.hasShape(obj)) {
                        return false;
                    }
                    const result = @{class_name}.deserialize(obj);
                    return Result.isOk(result);
                }
            };
            result.add_import("Result", "macroforge/utils");
            result.add_import("DeserializeContext", "macroforge/serde");
            result.add_import("DeserializeError", "macroforge/serde");
            result.add_type_import("DeserializeOptions", "macroforge/serde");
            result.add_import("PendingRef", "macroforge/serde");

            // Generate standalone functions that delegate to static methods
            let mut standalone = ts_template! {
                {>> "Deserializes input to an instance.\nAutomatically detects whether input is a JSON string or object.\n@param input - JSON string or object to deserialize\n@param opts - Optional deserialization options\n@returns Result containing the deserialized instance or validation errors" <<}
                export function @{fn_deserialize}(input: unknown, opts?: DeserializeOptions): Result<@{class_name}, Array<{ field: string; message: string }>> {
                    return @{class_name}.deserialize(input, opts);
                }

                {>> "Deserializes with an existing context for nested/cyclic object graphs.\n@param value - The raw value to deserialize\n@param ctx - The deserialization context" <<}
                export function @{fn_deserialize_internal}(value: any, ctx: DeserializeContext): @{class_name} | PendingRef {
                    return @{class_name}.deserializeWithContext(value, ctx);
                }

                {>> "Type guard: checks if a value can be successfully deserialized.\n@param value - The value to check\n@returns True if the value can be deserialized to this type" <<}
                export function @{fn_is}(value: unknown): value is @{class_name} {
                    return @{class_name}.is(value);
                }
            };
            standalone.add_import("Result", "macroforge/utils");
            standalone.add_import("DeserializeContext", "macroforge/serde");
            standalone.add_type_import("DeserializeOptions", "macroforge/serde");
            standalone.add_import("PendingRef", "macroforge/serde");

            // Combine standalone functions with class body by concatenating sources
            // The standalone output (no marker) must come FIRST so it defaults to "below" (after class)
            let combined_source = format!("{}\n{}", standalone.source(), result.source());
            let mut combined = TsStream::from_string(combined_source);
            combined.runtime_patches = standalone.runtime_patches;
            combined.runtime_patches.extend(result.runtime_patches);
            Ok(combined)
        }
        Data::Enum(_) => {
            let enum_name = input.name();
            let fn_deserialize = format!("{}Deserialize", to_camel_case(enum_name));
            let fn_deserialize_internal =
                format!("{}DeserializeWithContext", to_camel_case(enum_name));
            let fn_is = format!("{}Is", to_camel_case(enum_name));
            let mut result = ts_template! {
                {>> "Deserializes input to an enum value.\nAutomatically detects whether input is a JSON string or value.\n@param input - JSON string or value to deserialize\n@returns The enum value\n@throws Error if the value is not a valid enum member" <<}
                export function @{fn_deserialize}(input: unknown): @{enum_name} {
                    const data = typeof input === "string" ? JSON.parse(input) : input;
                    return @{fn_deserialize_internal}(data);
                }

                {>> "Deserializes with an existing context (for consistency with other types)." <<}
                export function @{fn_deserialize_internal}(data: unknown): @{enum_name} {
                    for (const key of Object.keys(@{enum_name})) {
                        const enumValue = @{enum_name}[key as keyof typeof @{enum_name}];
                        if (enumValue === data) {
                            return data as @{enum_name};
                        }
                    }
                    throw new Error("Invalid @{enum_name} value: " + JSON.stringify(data));
                }

                export function @{fn_is}(value: unknown): value is @{enum_name} {
                    for (const key of Object.keys(@{enum_name})) {
                        const enumValue = @{enum_name}[key as keyof typeof @{enum_name}];
                        if (enumValue === value) {
                            return true;
                        }
                    }
                    return false;
                }
            };

            result.add_import("DeserializeContext", "macroforge/serde");
            Ok(result)
        }
        Data::Interface(interface) => {
            let interface_name = input.name();
            let container_opts =
                SerdeContainerOptions::from_decorators(&interface.inner.decorators);

            // Collect deserializable fields with diagnostic collection
            let mut all_diagnostics = DiagnosticCollector::new();
            let fields: Vec<DeserializeField> = interface
                .fields()
                .iter()
                .filter_map(|field| {
                    let parse_result =
                        SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                    all_diagnostics.extend(parse_result.diagnostics);
                    let opts = parse_result.options;

                    if !opts.should_deserialize() {
                        return None;
                    }

                    let json_key = opts
                        .rename
                        .clone()
                        .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                    let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                    let nullable_inner_kind = match &type_cat {
                        TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };
                    let array_elem_kind = match &type_cat {
                        TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                        _ => None,
                    };

                    // Extract serializable type names for direct function calls
                    let nullable_serializable_type = match &type_cat {
                        TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                        _ => None,
                    };

                    // Check for foreign type deserializer if no explicit deserialize_with
                    let deserialize_with = if opts.deserialize_with.is_some() {
                        opts.deserialize_with.clone()
                    } else {
                        // Check if the field's type matches a configured foreign type
                        let foreign_types = get_foreign_types();
                        TypeCategory::match_foreign_type(&field.ts_type, &foreign_types)
                            .and_then(|ft| ft.deserialize_expr.clone())
                    };

                    Some(DeserializeField {
                        json_key,
                        field_name: field.name.clone(),
                        ts_type: field.ts_type.clone(),
                        type_cat,
                        optional: field.optional || opts.default || opts.default_expr.is_some(),
                        has_default: opts.default || opts.default_expr.is_some(),
                        default_expr: opts.default_expr.clone(),
                        flatten: opts.flatten,
                        validators: opts.validators.clone(),
                        nullable_inner_kind,
                        array_elem_kind,
                        nullable_serializable_type,
                        deserialize_with,
                    })
                })
                .collect();

            // Check for errors in field parsing before continuing
            if all_diagnostics.has_errors() {
                return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
            }

            let all_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
            let required_fields: Vec<_> = fields
                .iter()
                .filter(|f| !f.optional && !f.flatten)
                .cloned()
                .collect();

            let known_keys: Vec<String> = fields
                .iter()
                .filter(|f| !f.flatten)
                .map(|f| f.json_key.clone())
                .collect();

            let has_required = !required_fields.is_empty();
            let has_fields = !all_fields.is_empty();
            let deny_unknown = container_opts.deny_unknown_fields;

            // Fields with validators for per-field validation
            let fields_with_validators: Vec<_> = all_fields
                .iter()
                .filter(|f| f.has_validators())
                .cloned()
                .collect();
            let has_validators = !fields_with_validators.is_empty();

            // Generate shape check condition for hasShape method
            let shape_check_condition: String = if required_fields.is_empty() {
                "true".to_string()
            } else {
                required_fields
                    .iter()
                    .map(|f| format!("\"{}\" in o", f.json_key))
                    .collect::<Vec<_>>()
                    .join(" && ")
            };

            let (
                fn_deserialize,
                fn_deserialize_internal,
                fn_validate_field,
                fn_validate_fields,
                fn_is,
                fn_has_shape,
            ) = (
                format!("{}Deserialize", to_camel_case(interface_name)),
                format!("{}DeserializeWithContext", to_camel_case(interface_name)),
                format!("{}ValidateField", to_camel_case(interface_name)),
                format!("{}ValidateFields", to_camel_case(interface_name)),
                format!("{}Is", to_camel_case(interface_name)),
                format!("{}HasShape", to_camel_case(interface_name)),
            );

            let mut result = {
                ts_template! {
                    {>> "Deserializes input to this interface type.\nAutomatically detects whether input is a JSON string or object.\n@param input - JSON string or object to deserialize\n@param opts - Optional deserialization options\n@returns Result containing the deserialized value or validation errors" <<}
                    export function @{fn_deserialize}(input: unknown, opts?: DeserializeOptions): Result<@{interface_name}, Array<{ field: string; message: string }>> {
                        try {
                            // Auto-detect: if string, parse as JSON first
                            const data = typeof input === "string" ? JSON.parse(input) : input;

                            const ctx = DeserializeContext.create();
                            const resultOrRef = @{fn_deserialize_internal}(data, ctx);

                            if (PendingRef.is(resultOrRef)) {
                                return Result.err([{ field: "_root", message: "@{interface_name}.deserialize: root cannot be a forward reference" }]);
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
                            return Result.err([{ field: "_root", message }]);
                        }
                    }

                    {>> "Deserializes with an existing context for nested/cyclic object graphs.\n@param value - The raw value to deserialize\n@param ctx - The deserialization context" <<}
                    export function @{fn_deserialize_internal}(value: any, ctx: DeserializeContext): @{interface_name} | PendingRef {
                        if (value?.__ref !== undefined) {
                            return ctx.getOrDefer(value.__ref);
                        }

                        if (typeof value !== "object" || value === null || Array.isArray(value)) {
                            throw new DeserializeError([{ field: "_root", message: "@{interface_name}.deserializeWithContext: expected an object" }]);
                        }

                        const obj = value as Record<string, unknown>;
                        const errors: Array<{ field: string; message: string }> = [];

                        {#if deny_unknown}
                            const knownKeys = new Set(["__type", "__id", "__ref", {#for key in known_keys}"@{key}", {/for}]);
                            for (const key of Object.keys(obj)) {
                                if (!knownKeys.has(key)) {
                                    errors.push({ field: key, message: "unknown field" });
                                }
                            }
                        {/if}

                        {#if has_required}
                            {#for field in &required_fields}
                                if (!("@{field.json_key}" in obj)) {
                                    errors.push({ field: "@{field.json_key}", message: "missing required field" });
                                }
                            {/for}
                        {/if}

                        if (errors.length > 0) {
                            throw new DeserializeError(errors);
                        }

                        const instance: any = {};

                        if (obj.__id !== undefined) {
                            ctx.register(obj.__id as number, instance);
                        }

                        ctx.trackForFreeze(instance);

                        {#if has_fields}
                            {#for field in all_fields}
                                {$let raw_var = format!("__raw_{}", field.field_name)}
                                {$let has_validators = field.has_validators()}
                                {#if let Some(fn_name) = &field.deserialize_with}
                                    // Custom deserialization function (deserializeWith)
                                    {#if field.optional}
                                        if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                            instance.@{field.field_name} = @{fn_name}(obj["@{field.json_key}"]);
                                        }
                                    {:else}
                                        instance.@{field.field_name} = @{fn_name}(obj["@{field.json_key}"]);
                                    {/if}
                                {:else}
                                {#if field.optional}
                                    if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                        const @{raw_var} = obj["@{field.json_key}"] as @{field.ts_type};
                                        {#match &field.type_cat}
                                            {:case TypeCategory::Primitive}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, interface_name)}
                                                    @{validation_code}
                                                {/if}
                                                instance.@{field.field_name} = @{raw_var};

                                            {:case TypeCategory::Date}
                                                {
                                                    const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = __dateVal;
                                                }

                                            {:case TypeCategory::Array(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, interface_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                }

                                            {:case TypeCategory::Map(key_type, value_type)}
                                                if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                    instance.@{field.field_name} = new Map(
                                                        Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                }

                                            {:case TypeCategory::Set(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                                }

                                            {:case TypeCategory::Serializable(type_name)}
                                                {
                                                    {$let deserialize_with_context_fn = nested_deserialize_fn_name(type_name)}
                                                    const __result = @{deserialize_with_context_fn}(@{raw_var}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                }

                                            {:case TypeCategory::Nullable(_)}
                                                {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        instance.@{field.field_name} = @{raw_var};
                                                    {:case SerdeValueKind::Date}
                                                        if (@{raw_var} === null) {
                                                            instance.@{field.field_name} = null;
                                                        } else {
                                                            instance.@{field.field_name} = typeof @{raw_var} === "string"
                                                                ? new Date(@{raw_var} as any)
                                                                : @{raw_var} as any;
                                                        }
                                                    {:case _}
                                                        if (@{raw_var} === null) {
                                                            instance.@{field.field_name} = null;
                                                        } else {
                                                            {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                                {$let deserialize_with_context_fn = nested_deserialize_fn_name(inner_type)}
                                                                const __result = @{deserialize_with_context_fn}(@{raw_var}, ctx);
                                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                            {:else}
                                                                instance.@{field.field_name} = @{raw_var};
                                                            {/if}
                                                        }
                                                {/match}

                                            {:case _}
                                                instance.@{field.field_name} = @{raw_var};
                                        {/match}
                                    }
                                    {#if let Some(default_expr) = &field.default_expr}
                                        else {
                                            instance.@{field.field_name} = @{default_expr};
                                        }
                                    {/if}
                                {:else}
                                    {
                                        const @{raw_var} = obj["@{field.json_key}"] as @{field.ts_type};
                                        {#match &field.type_cat}
                                            {:case TypeCategory::Primitive}
                                                {#if has_validators}
                                                    {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, interface_name)}
                                                    @{validation_code}
                                                {/if}
                                                instance.@{field.field_name} = @{raw_var};

                                            {:case TypeCategory::Date}
                                                {
                                                    const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, interface_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = __dateVal;
                                                }

                                            {:case TypeCategory::Array(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, interface_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                }

                                            {:case TypeCategory::Map(key_type, value_type)}
                                                if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                    instance.@{field.field_name} = new Map(
                                                        Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                    );
                                                }

                                            {:case TypeCategory::Set(inner)}
                                                if (Array.isArray(@{raw_var})) {
                                                    instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                                }

                                            {:case TypeCategory::Serializable(type_name)}
                                                {
                                                    {$let deserialize_with_context_fn = nested_deserialize_fn_name(type_name)}
                                                    const __result = @{deserialize_with_context_fn}(@{raw_var}, ctx);
                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                }

                                            {:case TypeCategory::Nullable(_)}
                                                {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                    {:case SerdeValueKind::PrimitiveLike}
                                                        instance.@{field.field_name} = @{raw_var};
                                                    {:case SerdeValueKind::Date}
                                                        if (@{raw_var} === null) {
                                                            instance.@{field.field_name} = null;
                                                        } else {
                                                            instance.@{field.field_name} = typeof @{raw_var} === "string"
                                                                ? new Date(@{raw_var} as any)
                                                                : @{raw_var} as any;
                                                        }
                                                    {:case _}
                                                        if (@{raw_var} === null) {
                                                            instance.@{field.field_name} = null;
                                                        } else {
                                                            {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                                {$let deserialize_with_context_fn = nested_deserialize_fn_name(inner_type)}
                                                                const __result = @{deserialize_with_context_fn}(@{raw_var}, ctx);
                                                                ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                            {:else}
                                                                instance.@{field.field_name} = @{raw_var};
                                                            {/if}
                                                        }
                                                {/match}

                                            {:case _}
                                                instance.@{field.field_name} = @{raw_var};
                                        {/match}
                                    }
                                {/if}
                                {/if}
                            {/for}
                        {/if}

                        if (errors.length > 0) {
                            throw new DeserializeError(errors);
                        }

                        return instance as @{interface_name};
                    }

                    export function @{fn_validate_field}<K extends keyof @{interface_name}>(
                        field: K,
                        value: @{interface_name}[K]
                    ): Array<{ field: string; message: string }> {
                        {#if has_validators}
                        const errors: Array<{ field: string; message: string }> = [];
                        switch (field) {
                            {#for field in &fields_with_validators}
                            case "@{field.field_name}": {
                                const __val = value as @{field.ts_type};
                                {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name)}
                                @{validation_code}
                                break;
                            }
                            {/for}
                        }
                        return errors;
                        {:else}
                        return [];
                        {/if}
                    }

                    export function @{fn_validate_fields}(
                        partial: Partial<@{interface_name}>
                    ): Array<{ field: string; message: string }> {
                        {#if has_validators}
                        const errors: Array<{ field: string; message: string }> = [];
                        {#for field in &fields_with_validators}
                        if ("@{field.field_name}" in partial && partial.@{field.field_name} !== undefined) {
                            const __val = partial.@{field.field_name} as @{field.ts_type};
                            {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, interface_name)}
                            @{validation_code}
                        }
                        {/for}
                        return errors;
                        {:else}
                        return [];
                        {/if}
                    }

                    export function @{fn_has_shape}(obj: unknown): boolean {
                        if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                            return false;
                        }
                        const o = obj as Record<string, unknown>;
                        return @{shape_check_condition};
                    }

                    export function @{fn_is}(obj: unknown): obj is @{interface_name} {
                        if (!@{fn_has_shape}(obj)) {
                            return false;
                        }
                        const result = @{fn_deserialize}(obj);
                        return Result.isOk(result);
                    }
                }
            };

            result.add_import("Result", "macroforge/utils");
            result.add_import("DeserializeContext", "macroforge/serde");
            result.add_import("DeserializeError", "macroforge/serde");
            result.add_type_import("DeserializeOptions", "macroforge/serde");
            result.add_import("PendingRef", "macroforge/serde");
            Ok(result)
        }
        Data::TypeAlias(type_alias) => {
            let type_name = input.name();

            // Build generic type signature if type has type params
            let type_params = type_alias.type_params();
            let (generic_decl, generic_args) = if type_params.is_empty() {
                (String::new(), String::new())
            } else {
                let params = type_params.join(", ");
                (format!("<{}>", params), format!("<{}>", params))
            };
            let full_type_name = format!("{}{}", type_name, generic_args);

            // Create combined generic declarations for validateField that include K
            let validate_field_generic_decl = if type_params.is_empty() {
                format!("<K extends keyof {}>", type_name)
            } else {
                let params = type_params.join(", ");
                format!("<{}, K extends keyof {}>", params, full_type_name)
            };

            if type_alias.is_object() {
                let container_opts =
                    SerdeContainerOptions::from_decorators(&type_alias.inner.decorators);

                // Collect deserializable fields with diagnostic collection
                let mut all_diagnostics = DiagnosticCollector::new();
                let fields: Vec<DeserializeField> = type_alias
                    .as_object()
                    .unwrap()
                    .iter()
                    .filter_map(|field| {
                        let parse_result =
                            SerdeFieldOptions::from_decorators(&field.decorators, &field.name);
                        all_diagnostics.extend(parse_result.diagnostics);
                        let opts = parse_result.options;

                        if !opts.should_deserialize() {
                            return None;
                        }

                        let json_key = opts
                            .rename
                            .clone()
                            .unwrap_or_else(|| container_opts.rename_all.apply(&field.name));

                        let type_cat = TypeCategory::from_ts_type(&field.ts_type);

                        let nullable_inner_kind = match &type_cat {
                            TypeCategory::Nullable(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };
                        let array_elem_kind = match &type_cat {
                            TypeCategory::Array(inner) => Some(classify_serde_value_kind(inner)),
                            _ => None,
                        };

                        // Extract serializable type names for direct function calls
                        let nullable_serializable_type = match &type_cat {
                            TypeCategory::Nullable(inner) => get_serializable_type_name(inner),
                            _ => None,
                        };

                        Some(DeserializeField {
                            json_key,
                            field_name: field.name.clone(),
                            ts_type: field.ts_type.clone(),
                            type_cat,
                            optional: field.optional || opts.default || opts.default_expr.is_some(),
                            has_default: opts.default || opts.default_expr.is_some(),
                            default_expr: opts.default_expr.clone(),
                            flatten: opts.flatten,
                            validators: opts.validators.clone(),
                            nullable_inner_kind,
                            array_elem_kind,
                            nullable_serializable_type,
                            deserialize_with: opts.deserialize_with.clone(),
                        })
                    })
                    .collect();

                // Check for errors in field parsing before continuing
                if all_diagnostics.has_errors() {
                    return Err(MacroforgeErrors::new(all_diagnostics.into_vec()).into());
                }

                let all_fields: Vec<_> = fields.iter().filter(|f| !f.flatten).cloned().collect();
                let required_fields: Vec<_> = fields
                    .iter()
                    .filter(|f| !f.optional && !f.flatten)
                    .cloned()
                    .collect();

                let known_keys: Vec<String> = fields
                    .iter()
                    .filter(|f| !f.flatten)
                    .map(|f| f.json_key.clone())
                    .collect();

                let has_required = !required_fields.is_empty();
                let has_fields = !all_fields.is_empty();
                let deny_unknown = container_opts.deny_unknown_fields;

                // Fields with validators for per-field validation
                let fields_with_validators: Vec<_> = all_fields
                    .iter()
                    .filter(|f| f.has_validators())
                    .cloned()
                    .collect();
                let has_validators = !fields_with_validators.is_empty();

                let (
                    fn_deserialize,
                    fn_deserialize_internal,
                    fn_validate_field,
                    fn_validate_fields,
                    fn_is,
                ) = (
                    format!("{}Deserialize{}", to_camel_case(type_name), generic_decl),
                    format!("{}DeserializeWithContext", to_camel_case(type_name)),
                    format!(
                        "{}ValidateField{}",
                        to_camel_case(type_name),
                        validate_field_generic_decl
                    ),
                    format!("{}ValidateFields", to_camel_case(type_name)),
                    format!("{}Is{}", to_camel_case(type_name), generic_decl),
                );

                let mut result = {
                    ts_template! {
                        {>> "Deserializes input to this type.\nAutomatically detects whether input is a JSON string or object.\n@param input - JSON string or object to deserialize\n@param opts - Optional deserialization options\n@returns Result containing the deserialized value or validation errors" <<}
                        export function @{fn_deserialize}(input: unknown, opts?: DeserializeOptions): Result<@{full_type_name}, Array<{ field: string; message: string }>> {
                            try {
                                // Auto-detect: if string, parse as JSON first
                                const data = typeof input === "string" ? JSON.parse(input) : input;

                                const ctx = DeserializeContext.create();
                                const resultOrRef = @{fn_deserialize_internal}(data, ctx);

                                if (PendingRef.is(resultOrRef)) {
                                    return Result.err([{ field: "_root", message: "@{type_name}.deserialize: root cannot be a forward reference" }]);
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
                                return Result.err([{ field: "_root", message }]);
                            }
                        }

                        {>> "Deserializes with an existing context for nested/cyclic object graphs.\n@param value - The raw value to deserialize\n@param ctx - The deserialization context" <<}
                        export function @{fn_deserialize_internal}(value: any, ctx: DeserializeContext): @{type_name} | PendingRef {
                            if (value?.__ref !== undefined) {
                                return ctx.getOrDefer(value.__ref) as @{type_name} | PendingRef;
                            }

                            if (typeof value !== "object" || value === null || Array.isArray(value)) {
                                throw new DeserializeError([{ field: "_root", message: "@{type_name}.deserializeWithContext: expected an object" }]);
                            }

                            const obj = value as Record<string, unknown>;
                            const errors: Array<{ field: string; message: string }> = [];

                            {#if deny_unknown}
                                const knownKeys = new Set(["__type", "__id", "__ref", {#for key in known_keys}"@{key}", {/for}]);
                                for (const key of Object.keys(obj)) {
                                    if (!knownKeys.has(key)) {
                                        errors.push({ field: key, message: "unknown field" });
                                    }
                                }
                            {/if}

                            {#if has_required}
                                {#for field in &required_fields}
                                    if (!("@{field.json_key}" in obj)) {
                                        errors.push({ field: "@{field.json_key}", message: "missing required field" });
                                    }
                                {/for}
                            {/if}

                            if (errors.length > 0) {
                                throw new DeserializeError(errors);
                            }

                            const instance: any = {};

                            if (obj.__id !== undefined) {
                                ctx.register(obj.__id as number, instance);
                            }

                            ctx.trackForFreeze(instance);

                            {#if has_fields}
                                {#for field in all_fields}
                                    {$let raw_var = format!("__raw_{}", field.field_name)}
                                    {$let has_validators = field.has_validators()}
                                    {#if let Some(fn_name) = &field.deserialize_with}
                                        // Custom deserialization function (deserializeWith)
                                        {#if field.optional}
                                            if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                                instance.@{field.field_name} = @{fn_name}(obj["@{field.json_key}"]);
                                            }
                                        {:else}
                                            instance.@{field.field_name} = @{fn_name}(obj["@{field.json_key}"]);
                                        {/if}
                                    {:else}
                                    {#if field.optional}
                                        if ("@{field.json_key}" in obj && obj["@{field.json_key}"] !== undefined) {
                                            const @{raw_var} = obj["@{field.json_key}"] as @{field.ts_type};
                                            {#match &field.type_cat}
                                                {:case TypeCategory::Primitive}
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, type_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = @{raw_var};

                                                {:case TypeCategory::Date}
                                                    {
                                                        const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name)}
                                                            @{validation_code}
                                                        {/if}
                                                        instance.@{field.field_name} = __dateVal;
                                                    }

                                                {:case TypeCategory::Array(inner)}
                                                    if (Array.isArray(@{raw_var})) {
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, type_name)}
                                                            @{validation_code}
                                                        {/if}
                                                        instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                    }

                                                {:case TypeCategory::Map(key_type, value_type)}
                                                    if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                        instance.@{field.field_name} = new Map(
                                                            Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                        );
                                                    }

                                                {:case TypeCategory::Set(inner)}
                                                    if (Array.isArray(@{raw_var})) {
                                                        instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                                    }

                                                {:case TypeCategory::Serializable(inner_type_name)}
                                                    {
                                                        const __result = @{inner_type_name}.deserializeWithContext(@{raw_var}, ctx);
                                                        ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                    }

                                                {:case TypeCategory::Nullable(_)}
                                                    {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            instance.@{field.field_name} = @{raw_var};
                                                        {:case SerdeValueKind::Date}
                                                            if (@{raw_var} === null) {
                                                                instance.@{field.field_name} = null;
                                                            } else {
                                                                instance.@{field.field_name} = typeof @{raw_var} === "string"
                                                                    ? new Date(@{raw_var} as any)
                                                                    : @{raw_var} as any;
                                                            }
                                                        {:case _}
                                                            if (@{raw_var} === null) {
                                                                instance.@{field.field_name} = null;
                                                            } else {
                                                                {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                                    const __result = @{inner_type}.deserializeWithContext(@{raw_var}, ctx);
                                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                                {:else}
                                                                    instance.@{field.field_name} = @{raw_var};
                                                                {/if}
                                                            }
                                                    {/match}

                                                {:case _}
                                                    instance.@{field.field_name} = @{raw_var};
                                            {/match}
                                        }
                                        {#if let Some(default_expr) = &field.default_expr}
                                            else {
                                                instance.@{field.field_name} = @{default_expr};
                                            }
                                        {/if}
                                    {:else}
                                        {
                                            const @{raw_var} = obj["@{field.json_key}"] as @{field.ts_type};
                                            {#match &field.type_cat}
                                                {:case TypeCategory::Primitive}
                                                    {#if has_validators}
                                                        {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, type_name)}
                                                        @{validation_code}
                                                    {/if}
                                                    instance.@{field.field_name} = @{raw_var};

                                                {:case TypeCategory::Date}
                                                    {
                                                        const __dateVal = typeof @{raw_var} === "string" ? new Date(@{raw_var}) : @{raw_var} as Date;
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, "__dateVal", &field.json_key, type_name)}
                                                            @{validation_code}
                                                        {/if}
                                                        instance.@{field.field_name} = __dateVal;
                                                    }

                                                {:case TypeCategory::Array(inner)}
                                                    if (Array.isArray(@{raw_var})) {
                                                        {#if has_validators}
                                                            {$let validation_code = generate_field_validations(&field.validators, &raw_var, &field.json_key, type_name)}
                                                            @{validation_code}
                                                        {/if}
                                                        instance.@{field.field_name} = @{raw_var} as @{inner}[];
                                                    }

                                                {:case TypeCategory::Map(key_type, value_type)}
                                                    if (typeof @{raw_var} === "object" && @{raw_var} !== null) {
                                                        instance.@{field.field_name} = new Map(
                                                            Object.entries(@{raw_var} as Record<string, unknown>).map(([k, v]) => [k as @{key_type}, v as @{value_type}])
                                                        );
                                                    }

                                                {:case TypeCategory::Set(inner)}
                                                    if (Array.isArray(@{raw_var})) {
                                                        instance.@{field.field_name} = new Set(@{raw_var} as @{inner}[]);
                                                    }

                                                {:case TypeCategory::Serializable(inner_type_name)}
                                                    {
                                                        const __result = @{inner_type_name}.deserializeWithContext(@{raw_var}, ctx);
                                                        ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                    }

                                                {:case TypeCategory::Nullable(_)}
                                                    {#match field.nullable_inner_kind.unwrap_or(SerdeValueKind::Other)}
                                                        {:case SerdeValueKind::PrimitiveLike}
                                                            instance.@{field.field_name} = @{raw_var};
                                                        {:case SerdeValueKind::Date}
                                                            if (@{raw_var} === null) {
                                                                instance.@{field.field_name} = null;
                                                            } else {
                                                                instance.@{field.field_name} = typeof @{raw_var} === "string"
                                                                    ? new Date(@{raw_var} as any)
                                                                    : @{raw_var} as any;
                                                            }
                                                        {:case _}
                                                            if (@{raw_var} === null) {
                                                                instance.@{field.field_name} = null;
                                                            } else {
                                                                {#if let Some(inner_type) = &field.nullable_serializable_type}
                                                                    const __result = @{inner_type}.deserializeWithContext(@{raw_var}, ctx);
                                                                    ctx.assignOrDefer(instance, "@{field.field_name}", __result);
                                                                {:else}
                                                                    instance.@{field.field_name} = @{raw_var};
                                                                {/if}
                                                            }
                                                    {/match}

                                                {:case _}
                                                    instance.@{field.field_name} = @{raw_var};
                                            {/match}
                                        }
                                    {/if}
                                    {/if}
                                {/for}
                            {/if}

                            if (errors.length > 0) {
                                throw new DeserializeError(errors);
                            }

                            return instance as @{type_name};
                        }

                        export function @{fn_validate_field}(
                            field: K,
                            value: @{type_name}[K]
                        ): Array<{ field: string; message: string }> {
                            {#if has_validators}
                            const errors: Array<{ field: string; message: string }> = [];
                            switch (field) {
                                {#for field in &fields_with_validators}
                                case "@{field.field_name}": {
                                    const __val = value as @{field.ts_type};
                                    {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name)}
                                    @{validation_code}
                                    break;
                                }
                                {/for}
                            }
                            return errors;
                            {:else}
                            return [];
                            {/if}
                        }

                        export function @{fn_validate_fields}(
                            partial: Partial<@{type_name}>
                        ): Array<{ field: string; message: string }> {
                            {#if has_validators}
                            const errors: Array<{ field: string; message: string }> = [];
                            {#for field in &fields_with_validators}
                            if ("@{field.field_name}" in partial && partial.@{field.field_name} !== undefined) {
                                const __val = partial.@{field.field_name} as @{field.ts_type};
                                {$let validation_code = generate_field_validations(&field.validators, "__val", &field.json_key, type_name)}
                                @{validation_code}
                            }
                            {/for}
                            return errors;
                            {:else}
                            return [];
                            {/if}
                        }

                        export function @{fn_is}(obj: unknown): obj is @{full_type_name} {
                            if (typeof obj !== "object" || obj === null || Array.isArray(obj)) {
                                return false;
                            }
                            {#if has_required}
                                const o = obj as Record<string, unknown>;
                                {$let mut first = true}
                                return {#for field in &required_fields}{#if !first} && {/if}{$do first = false}"@{field.json_key}" in o{/for};
                            {:else}
                                return true;
                            {/if}
                        }
                    }
                };

                result.add_import("Result", "macroforge/utils");
                result.add_import("DeserializeContext", "macroforge/serde");
                result.add_import("DeserializeError", "macroforge/serde");
                result.add_type_import("DeserializeOptions", "macroforge/serde");
                result.add_import("PendingRef", "macroforge/serde");
                Ok(result)
            } else if let Some(members) = type_alias.as_union() {
                // Union type - could be literal union, type ref union, or mixed

                // Build generic type signature if type has type params
                let type_params = type_alias.type_params();
                let (generic_decl, generic_args) = if type_params.is_empty() {
                    (String::new(), String::new())
                } else {
                    let params = type_params.join(", ");
                    (format!("<{}>", params), format!("<{}>", params))
                };
                let full_type_name = format!("{}{}", type_name, generic_args);

                // Create a set of type parameter names for filtering
                let type_param_set: std::collections::HashSet<&str> =
                    type_params.iter().map(|s| s.as_str()).collect();

                let literals: Vec<String> = members
                    .iter()
                    .filter_map(|m| m.as_literal().map(|s| s.to_string()))
                    .collect();
                let type_refs: Vec<String> = members
                    .iter()
                    .filter_map(|m| m.as_type_ref().map(|s| s.to_string()))
                    .collect();

                // Separate primitives, generic type params, and serializable types
                let primitive_types: Vec<String> = type_refs
                    .iter()
                    .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Primitive))
                    .cloned()
                    .collect();

                // Generic type parameters (like T, U) - these are passed through as-is
                let generic_type_params: Vec<String> = type_refs
                    .iter()
                    .filter(|t| type_param_set.contains(t.as_str()))
                    .cloned()
                    .collect();

                // Build SerializableTypeRef with both full type and base type for runtime access
                let serializable_types: Vec<SerializableTypeRef> = type_refs
                    .iter()
                    .filter(|t| {
                        !matches!(
                            TypeCategory::from_ts_type(t),
                            TypeCategory::Primitive | TypeCategory::Date
                        ) && !type_param_set.contains(t.as_str())
                    })
                    .map(|t| SerializableTypeRef {
                        full_type: t.clone(),
                        base_type: extract_base_type(t),
                    })
                    .collect();

                let date_types: Vec<String> = type_refs
                    .iter()
                    .filter(|t| matches!(TypeCategory::from_ts_type(t), TypeCategory::Date))
                    .cloned()
                    .collect();

                let has_primitives = !primitive_types.is_empty();
                let has_serializables = !serializable_types.is_empty();
                let has_dates = !date_types.is_empty();
                let has_generic_params = !generic_type_params.is_empty();

                let is_literal_only = !literals.is_empty() && type_refs.is_empty();
                let is_primitive_only = has_primitives
                    && !has_serializables
                    && !has_dates
                    && !has_generic_params
                    && literals.is_empty();
                let is_serializable_only = !has_primitives
                    && !has_dates
                    && !has_generic_params
                    && has_serializables
                    && literals.is_empty();
                let has_literals = !literals.is_empty();

                // Pre-compute the expected types string for error messages
                let expected_types_str = if has_serializables {
                    serializable_types
                        .iter()
                        .map(|t| t.full_type.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                } else {
                    type_refs.join(", ")
                };

                let (fn_deserialize, fn_deserialize_internal, fn_is) = (
                    format!("{}Deserialize{}", to_camel_case(type_name), generic_decl),
                    format!(
                        "{}DeserializeWithContext{}",
                        to_camel_case(type_name),
                        generic_decl
                    ),
                    format!("{}Is{}", to_camel_case(type_name), generic_decl),
                );

            let mut result = ts_template! {
                {>> "Deserializes input to this type.\nAutomatically detects whether input is a JSON string or object.\n@param input - JSON string or object to deserialize\n@param opts - Optional deserialization options\n@returns Result containing the deserialized value or validation errors" <<}
                export function @{fn_deserialize}(input: unknown, opts?: DeserializeOptions): Result<@{full_type_name}, Array<{ field: string; message: string }>> {
                    try {
                        // Auto-detect: if string, parse as JSON first
                        const data = typeof input === "string" ? JSON.parse(input) : input;

                        const ctx = DeserializeContext.create();
                        const resultOrRef = @{fn_deserialize_internal}(data, ctx);

                        if (PendingRef.is(resultOrRef)) {
                            return Result.err([{ field: "_root", message: "@{type_name}.deserialize: root cannot be a forward reference" }]);
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
                        return Result.err([{ field: "_root", message }]);
                    }
                }

                {>> "Deserializes with an existing context for nested/cyclic object graphs.\n@param value - The raw value to deserialize\n@param ctx - The deserialization context" <<}
                export function @{fn_deserialize_internal}(value: any, ctx: DeserializeContext): @{full_type_name} | PendingRef {
                                if (value?.__ref !== undefined) {
                                    return ctx.getOrDefer(value.__ref) as @{full_type_name} | PendingRef;
                                }

                                {#if is_literal_only}
                                    const allowedValues = [{#for lit in &literals}@{lit}, {/for}] as const;
                                    if (!allowedValues.includes(value)) {
                                        throw new DeserializeError([{
                                            field: "_root",
                                            message: "Invalid value for @{type_name}: expected one of " + allowedValues.map(v => JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
                                        }]);
                                    }
                                    return value as @{full_type_name};
                                {:else if is_primitive_only}
                                    {#for prim in &primitive_types}
                                        if (typeof value === "@{prim}") {
                                            return value as @{full_type_name};
                                        }
                                    {/for}

                                    throw new DeserializeError([{
                                        field: "_root",
                                        message: "@{type_name}.deserializeWithContext: expected @{expected_types_str}, got " + typeof value
                                    }]);
                                {:else if is_serializable_only}
                                    if (typeof value !== "object" || value === null) {
                                        throw new DeserializeError([{
                                            field: "_root",
                                            message: "@{type_name}.deserializeWithContext: expected an object"
                                        }]);
                                    }

                                    const __typeName = (value as any).__type;
                                    if (typeof __typeName !== "string") {
                                        throw new DeserializeError([{
                                            field: "_root",
                                            message: "@{type_name}.deserializeWithContext: missing __type field for union dispatch"
                                        }]);
                                    }

                                    {#for type_ref in &serializable_types}
                                        if (__typeName === "@{type_ref.full_type}") {
                                            {$let deserialize_with_context_fn = nested_deserialize_fn_name(&type_ref.base_type)}
                                            return @{deserialize_with_context_fn}(value, ctx) as @{full_type_name};
                                        }
                                    {/for}

                                    throw new DeserializeError([{
                                        field: "_root",
                                        message: "@{type_name}.deserializeWithContext: unknown type \"" + __typeName + "\". Expected one of: @{expected_types_str}"
                                    }]);
                                {:else}
                                    {#if has_literals}
                                        const allowedLiterals = [{#for lit in &literals}@{lit}, {/for}] as const;
                                        if (allowedLiterals.includes(value as any)) {
                                            return value as @{full_type_name};
                                        }
                                    {/if}

                                    {#if has_primitives}
                                        {#for prim in &primitive_types}
                                            if (typeof value === "@{prim}") {
                                                return value as @{full_type_name};
                                            }
                                        {/for}
                                    {/if}

                                    {#if has_dates}
                                        if (value instanceof Date) {
                                            return value as @{full_type_name};
                                        }
                                        if (typeof value === "string") {
                                            const __dateVal = new Date(value);
                                            if (!isNaN(__dateVal.getTime())) {
                                                return __dateVal as unknown as @{full_type_name};
                                            }
                                        }
                                    {/if}

                                    {#if has_serializables}
                                        if (typeof value === "object" && value !== null) {
                                            const __typeName = (value as any).__type;
                                            if (typeof __typeName === "string") {
                                                {#for type_ref in &serializable_types}
                                                    if (__typeName === "@{type_ref.full_type}") {
                                                        {$let deserialize_with_context_fn = nested_deserialize_fn_name(&type_ref.base_type)}
                                                        return @{deserialize_with_context_fn}(value, ctx) as @{full_type_name};
                                                    }
                                                {/for}
                                            }
                                        }
                                    {/if}

                                    {#if has_generic_params}
                                        return value as @{full_type_name};
                                    {/if}

                                    throw new DeserializeError([{
                                        field: "_root",
                                        message: "@{type_name}.deserializeWithContext: value does not match any union member"
                                    }]);
                    {/if}
                }

                export function @{fn_is}(value: unknown): value is @{full_type_name} {
                                {#if is_literal_only}
                                    const allowedValues = [{#for lit in &literals}@{lit}, {/for}] as const;
                                    return allowedValues.includes(value as any);
                                {:else if is_primitive_only}
                                    {$let mut first = true}
                                    return {#for prim in &primitive_types}{#if !first} || {/if}{$do first = false}typeof value === "@{prim}"{/for};
                                {:else if is_serializable_only}
                                    if (typeof value !== "object" || value === null) {
                                        return false;
                                    }
                                    const __typeName = (value as any).__type;
                                    {$let mut first = true}
                                    return {#for type_ref in &serializable_types}{#if !first} || {/if}{$do first = false}__typeName === "@{type_ref.full_type}"{/for};
                                {:else}
                                    {#if has_literals}
                                        const allowedLiterals = [{#for lit in &literals}@{lit}, {/for}] as const;
                                        if (allowedLiterals.includes(value as any)) return true;
                                    {/if}
                                    {#if has_primitives}
                                        {#for prim in &primitive_types}
                                            if (typeof value === "@{prim}") return true;
                                        {/for}
                                    {/if}
                                    {#if has_dates}
                                        if (value instanceof Date) return true;
                                    {/if}
                                    {#if has_serializables}
                                        if (typeof value === "object" && value !== null) {
                                            const __typeName = (value as any).__type;
                                            {$let mut first = true}
                                            if ({#for type_ref in &serializable_types}{#if !first} || {/if}{$do first = false}__typeName === "@{type_ref.full_type}"{/for}) return true;
                                        }
                                    {/if}
                                    {#if has_generic_params}
                                        return true;
                                    {:else}
                                        return false;
                                    {/if}
                    {/if}
                }
            };
                result.add_import("Result", "macroforge/utils");
                result.add_import("DeserializeContext", "macroforge/serde");
                result.add_import("DeserializeError", "macroforge/serde");
                result.add_type_import("DeserializeOptions", "macroforge/serde");
                result.add_import("PendingRef", "macroforge/serde");
                Ok(result)
            } else {
                // Fallback for other type alias forms (simple alias, tuple, etc.)
                let (
                    fn_deserialize,
                    fn_deserialize_internal,
                    fn_validate_field,
                    fn_validate_fields,
                    fn_is,
                ) = (
                    format!("{}Deserialize{}", to_camel_case(type_name), generic_decl),
                    format!(
                        "{}DeserializeWithContext{}",
                        to_camel_case(type_name),
                        generic_args
                    ),
                    format!(
                        "{}ValidateField{}",
                        to_camel_case(type_name),
                        validate_field_generic_decl
                    ),
                    format!("{}ValidateFields", to_camel_case(type_name)),
                    format!("{}Is{}", to_camel_case(type_name), generic_decl),
                );

                let mut result = ts_template! {
                    {>> "Deserializes input to this type.\nAutomatically detects whether input is a JSON string or object.\n@param input - JSON string or object to deserialize\n@param opts - Optional deserialization options\n@returns Result containing the deserialized value or validation errors" <<}
                    export function @{fn_deserialize}(input: unknown, opts?: DeserializeOptions): Result<@{full_type_name}, Array<{ field: string; message: string }>> {
                        try {
                            // Auto-detect: if string, parse as JSON first
                            const data = typeof input === "string" ? JSON.parse(input) : input;

                            const ctx = DeserializeContext.create();
                            const result = @{fn_deserialize_internal}(data, ctx);
                            ctx.applyPatches();
                            if (opts?.freeze) {
                                ctx.freezeAll();
                            }
                            return Result.ok<@{full_type_name}>(result);
                        } catch (e) {
                            if (e instanceof DeserializeError) {
                                return Result.err(e.errors);
                            }
                            const message = e instanceof Error ? e.message : String(e);
                            return Result.err([{ field: "_root", message }]);
                        }
                    }

                    {>> "Deserializes with an existing context for nested/cyclic object graphs.\n@param value - The raw value to deserialize\n@param ctx - The deserialization context" <<}
                    export function @{fn_deserialize_internal}(value: any, ctx: DeserializeContext): @{full_type_name} {
                        if (value?.__ref !== undefined) {
                            return ctx.getOrDefer(value.__ref) as @{full_type_name};
                        }
                        return value as @{type_name};
                    }

                    export function @{fn_validate_field}(
                        field: K,
                        value: @{type_name}[K]
                    ): Array<{ field: string; message: string }> {
                        return [];
                    }

                    export function @{fn_validate_fields}(
                        partial: Partial<@{type_name}>
                    ): Array<{ field: string; message: string }> {
                        return [];
                    }

                    export function @{fn_is}(value: unknown): value is @{full_type_name} {
                        return value != null;
                    }
                };
                result.add_import("Result", "macroforge/utils");
                result.add_import("DeserializeContext", "macroforge/serde");
                result.add_import("DeserializeError", "macroforge/serde");
                result.add_type_import("DeserializeOptions", "macroforge/serde");
                Ok(result)
            }
        }
    }
}

/// Get JavaScript typeof string for a TypeScript primitive type
#[allow(dead_code)]
fn get_js_typeof(ts_type: &str) -> &'static str {
    match ts_type.trim() {
        "string" => "string",
        "number" => "number",
        "boolean" => "boolean",
        "bigint" => "bigint",
        _ => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_field_has_validators() {
        let field = DeserializeField {
            json_key: "email".into(),
            field_name: "email".into(),
            ts_type: "string".into(),
            type_cat: TypeCategory::Primitive,
            optional: false,
            has_default: false,
            default_expr: None,
            flatten: false,
            validators: vec![ValidatorSpec {
                validator: Validator::Email,
                custom_message: None,
            }],
            nullable_inner_kind: None,
            array_elem_kind: None,
            nullable_serializable_type: None,
            deserialize_with: None,
        };
        assert!(field.has_validators());

        let field_no_validators = DeserializeField {
            validators: vec![],
            ..field
        };
        assert!(!field_no_validators.has_validators());
    }

    #[test]
    fn test_validation_condition_generation() {
        let condition = generate_validation_condition(&Validator::Email, "value");
        assert!(condition.contains("test(value)"));

        let condition = generate_validation_condition(&Validator::MaxLength(255), "str");
        assert_eq!(condition, "str.length > 255");
    }

    #[test]
    fn test_validator_message() {
        assert_eq!(
            get_validator_message(&Validator::Email),
            "must be a valid email"
        );
    }
}
