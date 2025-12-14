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

export function toStringStatus(value: Status): string {
    const key = Status[value as unknown as keyof typeof Status];
    if (key !== undefined) {
        return 'Status.' + key;
    }
    return 'Status(' + String(value) + ')';
}

export function cloneStatus(value: Status): Status {
    return value;
}

export function equalsStatus(a: Status, b: Status): boolean {
    return a === b;
}

export function hashCodeStatus(value: Status): number {
    if (typeof value === 'string') {
        let hash = 0;
        for (let i = 0; i < value.length; i++) {
            hash = (hash * 31 + value.charCodeAt(i)) | 0;
        }
        return hash;
    }
    return value as number;
}

export function toStringifiedJSONStatus(value: Status): string {
    return JSON.stringify(value);
}
export function __serializeStatus(_ctx: SerializeContext): string | number {
    return value;
}

export function fromStringifiedJSONStatus(json: string): Status {
    const data = JSON.parse(json);
    return __deserializeStatus(data);
}
export function __deserializeStatus(data: unknown): Status {
    for (const key of Object.keys(Status)) {
        const enumValue = Status[key as keyof typeof Status];
        if (enumValue === data) {
            return data as Status;
        }
    }
    throw new Error('Invalid Status value: ' + JSON.stringify(data));
}
export function isStatus(value: unknown): value is Status {
    for (const key of Object.keys(Status)) {
        const enumValue = Status[key as keyof typeof Status];
        if (enumValue === value) {
            return true;
        }
    }
    return false;
}

export enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4
}

export function toStringPriority(value: Priority): string {
    const key = Priority[value as unknown as keyof typeof Priority];
    if (key !== undefined) {
        return 'Priority.' + key;
    }
    return 'Priority(' + String(value) + ')';
}

export function clonePriority(value: Priority): Priority {
    return value;
}

export function equalsPriority(a: Priority, b: Priority): boolean {
    return a === b;
}

export function hashCodePriority(value: Priority): number {
    if (typeof value === 'string') {
        let hash = 0;
        for (let i = 0; i < value.length; i++) {
            hash = (hash * 31 + value.charCodeAt(i)) | 0;
        }
        return hash;
    }
    return value as number;
}

export function toStringifiedJSONPriority(value: Priority): string {
    return JSON.stringify(value);
}
export function __serializePriority(_ctx: SerializeContext): string | number {
    return value;
}

export function fromStringifiedJSONPriority(json: string): Priority {
    const data = JSON.parse(json);
    return __deserializePriority(data);
}
export function __deserializePriority(data: unknown): Priority {
    for (const key of Object.keys(Priority)) {
        const enumValue = Priority[key as keyof typeof Priority];
        if (enumValue === data) {
            return data as Priority;
        }
    }
    throw new Error('Invalid Priority value: ' + JSON.stringify(data));
}
export function isPriority(value: unknown): value is Priority {
    for (const key of Object.keys(Priority)) {
        const enumValue = Priority[key as keyof typeof Priority];
        if (enumValue === value) {
            return true;
        }
    }
    return false;
}

export enum Color {
    Red,
    Green,
    Blue
}

export function toStringColor(value: Color): string {
    const key = Color[value as unknown as keyof typeof Color];
    if (key !== undefined) {
        return 'Color.' + key;
    }
    return 'Color(' + String(value) + ')';
}

export function equalsColor(a: Color, b: Color): boolean {
    return a === b;
}

export function hashCodeColor(value: Color): number {
    if (typeof value === 'string') {
        let hash = 0;
        for (let i = 0; i < value.length; i++) {
            hash = (hash * 31 + value.charCodeAt(i)) | 0;
        }
        return hash;
    }
    return value as number;
}

// ==================== TYPE ALIAS EXAMPLES ====================

export type Point = {
    x: number;
    y: number;
};

export function toStringPoint(value: Point): string {
    const parts: string[] = [];
    parts.push('x: ' + value.x);
    parts.push('y: ' + value.y);
    return 'Point { ' + parts.join(', ') + ' }';
}

export function clonePoint(value: Point): Point {
    return { x: value.x, y: value.y };
}

export function equalsPoint(a: Point, b: Point): boolean {
    if (a === b) return true;
    return a.x === b.x && a.y === b.y;
}

export function hashCodePoint(value: Point): number {
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

export function toStringifiedJSONPoint(value: Point): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializePoint(value, ctx));
}
export function toObjectPoint(value: Point): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializePoint(value, ctx);
}
export function __serializePoint(value: Point, ctx: SerializeContext): Record<string, unknown> {
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

export function fromStringifiedJSONPoint(
    json: string,
    opts?: DeserializeOptions
): Result<Point, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectPoint(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectPoint(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Point, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializePoint(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Point.fromObject: root cannot be a forward reference' }
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
}
export function __deserializePoint(value: any, ctx: DeserializeContext): Point | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Point | PendingRef;
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'Point.__deserialize: expected an object' }
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
export function validateFieldPoint<K extends keyof Point>(
    field: K,
    value: Point[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsPoint(
    partial: Partial<Point>
): Array<{ field: string; message: string }> {
    return [];
}
export function isPoint(obj: unknown): obj is Point {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'x' in o && 'y' in o;
}

export type UserProfile = {
    id: string;
    username: string;
    email: string;
    age: number;
    isVerified: boolean;
};

export function toStringUserProfile(value: UserProfile): string {
    const parts: string[] = [];
    parts.push('id: ' + value.id);
    parts.push('username: ' + value.username);
    parts.push('email: ' + value.email);
    parts.push('age: ' + value.age);
    parts.push('isVerified: ' + value.isVerified);
    return 'UserProfile { ' + parts.join(', ') + ' }';
}

export function cloneUserProfile(value: UserProfile): UserProfile {
    return {
        id: value.id,
        username: value.username,
        email: value.email,
        age: value.age,
        isVerified: value.isVerified
    };
}

export function equalsUserProfile(a: UserProfile, b: UserProfile): boolean {
    if (a === b) return true;
    return (
        a.id === b.id &&
        a.username === b.username &&
        a.email === b.email &&
        a.age === b.age &&
        a.isVerified === b.isVerified
    );
}

export function hashCodeUserProfile(value: UserProfile): number {
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

export function toStringifiedJSONUserProfile(value: UserProfile): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeUserProfile(value, ctx));
}
export function toObjectUserProfile(value: UserProfile): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeUserProfile(value, ctx);
}
export function __serializeUserProfile(
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

export function fromStringifiedJSONUserProfile(
    json: string,
    opts?: DeserializeOptions
): Result<UserProfile, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectUserProfile(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectUserProfile(
    obj: unknown,
    opts?: DeserializeOptions
): Result<UserProfile, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeUserProfile(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'UserProfile.fromObject: root cannot be a forward reference'
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
}
export function __deserializeUserProfile(
    value: any,
    ctx: DeserializeContext
): UserProfile | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as UserProfile | PendingRef;
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'UserProfile.__deserialize: expected an object' }
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
export function validateFieldUserProfile<K extends keyof UserProfile>(
    field: K,
    value: UserProfile[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsUserProfile(
    partial: Partial<UserProfile>
): Array<{ field: string; message: string }> {
    return [];
}
export function isUserProfile(obj: unknown): obj is UserProfile {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'id' in o && 'username' in o && 'email' in o && 'age' in o && 'isVerified' in o;
}

export type Coordinate3D = {
    x: number;
    y: number;
    z: number;
};

export function toStringCoordinate3D(value: Coordinate3D): string {
    const parts: string[] = [];
    parts.push('x: ' + value.x);
    parts.push('y: ' + value.y);
    parts.push('z: ' + value.z);
    return 'Coordinate3D { ' + parts.join(', ') + ' }';
}

export function cloneCoordinate3D(value: Coordinate3D): Coordinate3D {
    return { x: value.x, y: value.y, z: value.z };
}

export function equalsCoordinate3D(a: Coordinate3D, b: Coordinate3D): boolean {
    if (a === b) return true;
    return a.x === b.x && a.y === b.y && a.z === b.z;
}

export function hashCodeCoordinate3D(value: Coordinate3D): number {
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

export type ApiStatus = 'loading' | 'success' | 'error';

export function toStringApiStatus(value: ApiStatus): string {
    return 'ApiStatus(' + JSON.stringify(value) + ')';
}

export function equalsApiStatus(a: ApiStatus, b: ApiStatus): boolean {
    if (a === b) return true;
    if (typeof a === 'object' && typeof b === 'object' && a !== null && b !== null) {
        return JSON.stringify(a) === JSON.stringify(b);
    }
    return false;
}

export function hashCodeApiStatus(value: ApiStatus): number {
    const str = JSON.stringify(value);
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
        hash = (hash * 31 + str.charCodeAt(i)) | 0;
    }
    return hash;
}

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
