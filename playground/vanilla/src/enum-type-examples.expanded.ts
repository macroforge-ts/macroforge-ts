import { DeserializeContext } from 'macroforge/serde';
import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
/**
 * Examples demonstrating derive macros on enums and type aliases.
 * These showcase the new enum and type alias support for all built-in macros.
 *
 * Generated functions use suffix naming style (default):
 * - toStringStatus, cloneStatus, equalsStatus, etc.
 * - toStringPoint, clonePoint, equalsPoint, etc.
 */

// ==================== ENUM EXAMPLES ====================

export enum Status {
    Active = 'active',
    Inactive = 'inactive',
    Pending = 'pending'
}

export function statusToString(value: Status): string {
    const key = Status[value as unknown as keyof typeof Status];
    if (key !== undefined) {
        return 'Status.' + key;
    }
    return 'Status(' + String(value) + ')';
}

export function statusClone(value: Status): Status {
    return value;
}

export function statusEquals(a: Status, b: Status): boolean {
    return a === b;
}

export function statusHashCode(value: Status): number {
    if (typeof value === 'string') {
        let hash = 0;
        for (let i = 0; i < value.length; i++) {
            hash = (hash * 31 + value.charCodeAt(i)) | 0;
        }
        return hash;
    }
    return value as number;
}

/** Serializes this enum value to a JSON string. */ export function statusSerialize(
    value: Status
): string {
    return JSON.stringify(value);
} /** Serializes with an existing context for nested/cyclic object graphs. */
export function statusSerializeWithContext(value: Status, _ctx: SerializeContext): string | number {
    return value;
}

/** Deserializes input to an enum value.
Automatically detects whether input is a JSON string or value.
@param input - JSON string or value to deserialize
@returns The enum value
@throws Error if the value is not a valid enum member */ export function statusDeserialize(
    input: unknown
): Status {
    const data = typeof input === 'string' ? JSON.parse(input) : input;
    return statusDeserializeWithContext(data);
} /** Deserializes with an existing context (for consistency with other types). */
export function statusDeserializeWithContext(data: unknown): Status {
    for (const key of Object.keys(Status)) {
        const enumValue = Status[key as keyof typeof Status];
        if (enumValue === data) {
            return data as Status;
        }
    }
    throw new Error('Invalid Status value: ' + JSON.stringify(data));
}
export function statusIs(value: unknown): value is Status {
    for (const key of Object.keys(Status)) {
        const enumValue = Status[key as keyof typeof Status];
        if (enumValue === value) {
            return true;
        }
    }
    return false;
}

export const Status = {
    toString: statusToString,
    clone: statusClone,
    equals: statusEquals,
    hashCode: statusHashCode,
    serialize: statusSerialize,
    serializeWithContext: statusSerializeWithContext,
    deserialize: statusDeserialize,
    deserializeWithContext: statusDeserializeWithContext,
    is: statusIs
} as const;

export enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4
}

export function priorityToString(value: Priority): string {
    const key = Priority[value as unknown as keyof typeof Priority];
    if (key !== undefined) {
        return 'Priority.' + key;
    }
    return 'Priority(' + String(value) + ')';
}

export function priorityClone(value: Priority): Priority {
    return value;
}

export function priorityEquals(a: Priority, b: Priority): boolean {
    return a === b;
}

export function priorityHashCode(value: Priority): number {
    if (typeof value === 'string') {
        let hash = 0;
        for (let i = 0; i < value.length; i++) {
            hash = (hash * 31 + value.charCodeAt(i)) | 0;
        }
        return hash;
    }
    return value as number;
}

/** Serializes this enum value to a JSON string. */ export function prioritySerialize(
    value: Priority
): string {
    return JSON.stringify(value);
} /** Serializes with an existing context for nested/cyclic object graphs. */
export function prioritySerializeWithContext(
    value: Priority,
    _ctx: SerializeContext
): string | number {
    return value;
}

/** Deserializes input to an enum value.
Automatically detects whether input is a JSON string or value.
@param input - JSON string or value to deserialize
@returns The enum value
@throws Error if the value is not a valid enum member */ export function priorityDeserialize(
    input: unknown
): Priority {
    const data = typeof input === 'string' ? JSON.parse(input) : input;
    return priorityDeserializeWithContext(data);
} /** Deserializes with an existing context (for consistency with other types). */
export function priorityDeserializeWithContext(data: unknown): Priority {
    for (const key of Object.keys(Priority)) {
        const enumValue = Priority[key as keyof typeof Priority];
        if (enumValue === data) {
            return data as Priority;
        }
    }
    throw new Error('Invalid Priority value: ' + JSON.stringify(data));
}
export function priorityIs(value: unknown): value is Priority {
    for (const key of Object.keys(Priority)) {
        const enumValue = Priority[key as keyof typeof Priority];
        if (enumValue === value) {
            return true;
        }
    }
    return false;
}

export const Priority = {
    toString: priorityToString,
    clone: priorityClone,
    equals: priorityEquals,
    hashCode: priorityHashCode,
    serialize: prioritySerialize,
    serializeWithContext: prioritySerializeWithContext,
    deserialize: priorityDeserialize,
    deserializeWithContext: priorityDeserializeWithContext,
    is: priorityIs
} as const;

export enum Color {
    Red,
    Green,
    Blue
}

export function colorToString(value: Color): string {
    const key = Color[value as unknown as keyof typeof Color];
    if (key !== undefined) {
        return 'Color.' + key;
    }
    return 'Color(' + String(value) + ')';
}

export function colorEquals(a: Color, b: Color): boolean {
    return a === b;
}

export function colorHashCode(value: Color): number {
    if (typeof value === 'string') {
        let hash = 0;
        for (let i = 0; i < value.length; i++) {
            hash = (hash * 31 + value.charCodeAt(i)) | 0;
        }
        return hash;
    }
    return value as number;
}

export const Color = {
    toString: colorToString,
    equals: colorEquals,
    hashCode: colorHashCode
} as const;

// ==================== TYPE ALIAS EXAMPLES ====================

export type Point = {
    x: number;
    y: number;
};

export function pointToString(value: Point): string {
    const parts: string[] = [];
    parts.push('x: ' + value.x);
    parts.push('y: ' + value.y);
    return 'Point { ' + parts.join(', ') + ' }';
}

export function pointClone(value: Point): Point {
    return { x: value.x, y: value.y };
}

export function pointEquals(a: Point, b: Point): boolean {
    if (a === b) return true;
    return a.x === b.x && a.y === b.y;
}

export function pointHashCode(value: Point): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (Number.isInteger(value.x)
                ? value.x | 0
                : value.x
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (Number.isInteger(value.y)
                ? value.y | 0
                : value.y
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    return hash;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function pointSerialize(
    value: Point
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(pointSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function pointSerializeWithContext(
    value: Point,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'Point', __id };
    result['x'] = value.x;
    result['y'] = value.y;
    return result;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function pointDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<Point, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = pointDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Point.deserialize: root cannot be a forward reference' }
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
        return Result.err([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function pointDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): Point | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Point | PendingRef;
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Point.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('x' in obj)) {
        errors.push({ field: 'x', message: 'missing required field' });
    }
    if (!('y' in obj)) {
        errors.push({ field: 'y', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_x = obj['x'] as number;
        instance.x = __raw_x;
    }
    {
        const __raw_y = obj['y'] as number;
        instance.y = __raw_y;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as Point;
}
export function pointValidateField<K extends keyof Point>(
    field: K,
    value: Point[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function pointValidateFields(
    partial: Partial<Point>
): Array<{ field: string; message: string }> {
    return [];
}
export function pointIs(obj: unknown): obj is Point {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'x' in o && 'y' in o;
}

export const Point = {
    toString: pointToString,
    clone: pointClone,
    equals: pointEquals,
    hashCode: pointHashCode,
    serialize: pointSerialize,
    serializeWithContext: pointSerializeWithContext,
    deserialize: pointDeserialize,
    deserializeWithContext: pointDeserializeWithContext,
    validateFields: pointValidateFields,
    is: pointIs
} as const;

export type UserProfile = {
    id: string;
    username: string;
    email: string;
    age: number;
    isVerified: boolean;
};

export function userProfileToString(value: UserProfile): string {
    const parts: string[] = [];
    parts.push('id: ' + value.id);
    parts.push('username: ' + value.username);
    parts.push('email: ' + value.email);
    parts.push('age: ' + value.age);
    parts.push('isVerified: ' + value.isVerified);
    return 'UserProfile { ' + parts.join(', ') + ' }';
}

export function userProfileClone(value: UserProfile): UserProfile {
    return {
        id: value.id,
        username: value.username,
        email: value.email,
        age: value.age,
        isVerified: value.isVerified
    };
}

export function userProfileEquals(a: UserProfile, b: UserProfile): boolean {
    if (a === b) return true;
    return (
        a.id === b.id &&
        a.username === b.username &&
        a.email === b.email &&
        a.age === b.age &&
        a.isVerified === b.isVerified
    );
}

export function userProfileHashCode(value: UserProfile): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (value.id ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    hash =
        (hash * 31 +
            (value.username ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    hash =
        (hash * 31 +
            (value.email ?? '').split('').reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0)) |
        0;
    hash =
        (hash * 31 +
            (Number.isInteger(value.age)
                ? value.age | 0
                : value.age
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash = (hash * 31 + (value.isVerified ? 1231 : 1237)) | 0;
    return hash;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function userProfileSerialize(
    value: UserProfile
): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(userProfileSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function userProfileSerializeWithContext(
    value: UserProfile,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'UserProfile', __id };
    result['id'] = value.id;
    result['username'] = value.username;
    result['email'] = value.email;
    result['age'] = value.age;
    result['isVerified'] = value.isVerified;
    return result;
}

/** Deserializes input to this type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function userProfileDeserialize(
    input: unknown,
    opts?: DeserializeOptions
): Result<UserProfile, Array<{ field: string; message: string }>> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = DeserializeContext.create();
        const resultOrRef = userProfileDeserializeWithContext(data, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'UserProfile.deserialize: root cannot be a forward reference'
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
        return Result.err([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function userProfileDeserializeWithContext(
    value: any,
    ctx: DeserializeContext
): UserProfile | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as UserProfile | PendingRef;
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'UserProfile.deserializeWithContext: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('id' in obj)) {
        errors.push({ field: 'id', message: 'missing required field' });
    }
    if (!('username' in obj)) {
        errors.push({ field: 'username', message: 'missing required field' });
    }
    if (!('email' in obj)) {
        errors.push({ field: 'email', message: 'missing required field' });
    }
    if (!('age' in obj)) {
        errors.push({ field: 'age', message: 'missing required field' });
    }
    if (!('isVerified' in obj)) {
        errors.push({ field: 'isVerified', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_id = obj['id'] as string;
        instance.id = __raw_id;
    }
    {
        const __raw_username = obj['username'] as string;
        instance.username = __raw_username;
    }
    {
        const __raw_email = obj['email'] as string;
        instance.email = __raw_email;
    }
    {
        const __raw_age = obj['age'] as number;
        instance.age = __raw_age;
    }
    {
        const __raw_isVerified = obj['isVerified'] as boolean;
        instance.isVerified = __raw_isVerified;
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as UserProfile;
}
export function userProfileValidateField<K extends keyof UserProfile>(
    field: K,
    value: UserProfile[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function userProfileValidateFields(
    partial: Partial<UserProfile>
): Array<{ field: string; message: string }> {
    return [];
}
export function userProfileIs(obj: unknown): obj is UserProfile {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'username' in o && 'email' in o && 'age' in o && 'isVerified' in o;
}

export const UserProfile = {
    toString: userProfileToString,
    clone: userProfileClone,
    equals: userProfileEquals,
    hashCode: userProfileHashCode,
    serialize: userProfileSerialize,
    serializeWithContext: userProfileSerializeWithContext,
    deserialize: userProfileDeserialize,
    deserializeWithContext: userProfileDeserializeWithContext,
    validateFields: userProfileValidateFields,
    is: userProfileIs
} as const;

export type Coordinate3D = {
    x: number;
    y: number;
    z: number;
};

export function coordinate3DToString(value: Coordinate3D): string {
    const parts: string[] = [];
    parts.push('x: ' + value.x);
    parts.push('y: ' + value.y);
    parts.push('z: ' + value.z);
    return 'Coordinate3D { ' + parts.join(', ') + ' }';
}

export function coordinate3DClone(value: Coordinate3D): Coordinate3D {
    return { x: value.x, y: value.y, z: value.z };
}

export function coordinate3DEquals(a: Coordinate3D, b: Coordinate3D): boolean {
    if (a === b) return true;
    return a.x === b.x && a.y === b.y && a.z === b.z;
}

export function coordinate3DHashCode(value: Coordinate3D): number {
    let hash = 17;
    hash =
        (hash * 31 +
            (Number.isInteger(value.x)
                ? value.x | 0
                : value.x
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (Number.isInteger(value.y)
                ? value.y | 0
                : value.y
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    hash =
        (hash * 31 +
            (Number.isInteger(value.z)
                ? value.z | 0
                : value.z
                      .toString()
                      .split('')
                      .reduce((h, c) => (h * 31 + c.charCodeAt(0)) | 0, 0))) |
        0;
    return hash;
}

export const Coordinate3D = {
    toString: coordinate3DToString,
    clone: coordinate3DClone,
    equals: coordinate3DEquals,
    hashCode: coordinate3DHashCode
} as const;

export type ApiStatus = 'loading' | 'success' | 'error';

export function apiStatusToString(value: ApiStatus): string {
    return 'ApiStatus(' + JSON.stringify(value) + ')';
}

export function apiStatusEquals(a: ApiStatus, b: ApiStatus): boolean {
    if (a === b) return true;
    if (typeof a === 'object' && typeof b === 'object' && a !== null && b !== null) {
        return JSON.stringify(a) === JSON.stringify(b);
    }
    return false;
}

export function apiStatusHashCode(value: ApiStatus): number {
    const str = JSON.stringify(value);
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
        hash = (hash * 31 + str.charCodeAt(i)) | 0;
    }
    return hash;
}

export const ApiStatus = {
    toString: apiStatusToString,
    equals: apiStatusEquals,
    hashCode: apiStatusHashCode
} as const;

// ==================== USAGE EXAMPLES ====================

// Enum usage
export const currentStatus = Status.Active;
export const highPriority = Priority.High;

// Using generated standalone functions on enums (suffix naming style)
export function demoEnumFunctions() {
    // Debug - toStringStatus
    console.log('Status string:', toStringStatus(Status.Active));
    console.log('Priority string:', toStringPriority(Priority.High));

    // Clone - returns the same value for enums (primitives)
    const clonedStatus = cloneStatus(Status.Pending);
    console.log('Cloned status:', clonedStatus);

    // PartialEq - equalsStatus
    const areEqual = equalsStatus(Status.Active, Status.Active);
    console.log('Are equal:', areEqual);

    // Hash - hashCodeStatus
    const hash = hashCodeStatus(Status.Active);
    console.log('Hash code:', hash);

    // Serialize - toJSONStatus
    const json = toJSONStatus(Status.Inactive);
    console.log('Serialized:', json);

    // Deserialize - fromJSONStatus
    const parsed = fromJSONStatus('pending');
    console.log('Parsed:', parsed);
}

// Type alias usage
export const origin: Point = { x: 0, y: 0 };
export const user: UserProfile = {
    id: 'user-123',
    username: 'johndoe',
    email: 'john@example.com',
    age: 30,
    isVerified: true
};

// Using generated standalone functions on type aliases (suffix naming style)
export function demoTypeFunctions() {
    const point1: Point = { x: 10, y: 20 };
    const point2: Point = { x: 10, y: 20 };

    // Debug - toStringPoint
    console.log('Point string:', toStringPoint(point1));
    console.log('User string:', toStringUserProfile(user));

    // Clone - creates a shallow copy
    const clonedPoint = clonePoint(point1);
    console.log('Cloned point:', clonedPoint);

    // PartialEq - equalsPoint
    const pointsEqual = equalsPoint(point1, point2);
    console.log('Points equal:', pointsEqual);

    // Hash - hashCodePoint
    const pointHash = hashCodePoint(point1);
    console.log('Point hash:', pointHash);

    // Serialize - toJSONPoint
    const pointJson = toJSONPoint(point1);
    console.log('Point JSON:', pointJson);

    // Deserialize - fromJSONPoint
    const parsedPoint = fromJSONPoint({ x: 5, y: 10 });
    console.log('Parsed point:', parsedPoint);
}

// Run demos
demoEnumFunctions();
demoTypeFunctions();
