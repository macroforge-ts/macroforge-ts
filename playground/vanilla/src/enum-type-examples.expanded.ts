import { DeserializeContext } from 'macroforge/serde';
import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
/**
 * Examples demonstrating derive macros on enums and type aliases.
 * These showcase the new enum and type alias support for all built-in macros.
 */

// ==================== ENUM EXAMPLES ====================

/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
export enum Status {
    Active = 'active',
    Inactive = 'inactive',
    Pending = 'pending'
}

export namespace Status {
    export function toString(value: Status): string {
        const key = Status[value as unknown as keyof typeof Status];
        if (key !== undefined) {
            return 'Status.' + key;
        }
        return 'Status(' + String(value) + ')';
    }
}

export namespace Status {
    export function clone(value: Status): Status {
        return value;
    }
}

export namespace Status {
    export function equals(a: Status, b: Status): boolean {
        return a === b;
    }
}

export namespace Status {
    export function toStringifiedJSON(value: Status): string {
        return JSON.stringify(value);
    }
    export function __serialize(_ctx: SerializeContext): string | number {
        return value;
    }
}

export namespace Status {
    export function fromStringifiedJSON(json: string): Status {
        const data = JSON.parse(json);
        return __deserialize(data);
    }
    export function __deserialize(data: unknown): Status {
        for (const key of Object.keys(Status)) {
            const enumValue = Status[key as keyof typeof Status];
            if (enumValue === data) {
                return data as Status;
            }
        }
        throw new Error('Invalid Status value: ' + JSON.stringify(data));
    }
}

/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
export enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4
}

export namespace Priority {
    export function toString(value: Priority): string {
        const key = Priority[value as unknown as keyof typeof Priority];
        if (key !== undefined) {
            return 'Priority.' + key;
        }
        return 'Priority(' + String(value) + ')';
    }
}

export namespace Priority {
    export function clone(value: Priority): Priority {
        return value;
    }
}

export namespace Priority {
    export function equals(a: Priority, b: Priority): boolean {
        return a === b;
    }
}

export namespace Priority {
    export function toStringifiedJSON(value: Priority): string {
        return JSON.stringify(value);
    }
    export function __serialize(_ctx: SerializeContext): string | number {
        return value;
    }
}

export namespace Priority {
    export function fromStringifiedJSON(json: string): Priority {
        const data = JSON.parse(json);
        return __deserialize(data);
    }
    export function __deserialize(data: unknown): Priority {
        for (const key of Object.keys(Priority)) {
            const enumValue = Priority[key as keyof typeof Priority];
            if (enumValue === data) {
                return data as Priority;
            }
        }
        throw new Error('Invalid Priority value: ' + JSON.stringify(data));
    }
}

/** @derive(Debug, PartialEq) */
export enum Color {
    Red,
    Green,
    Blue
}

export namespace Color {
    export function toString(value: Color): string {
        const key = Color[value as unknown as keyof typeof Color];
        if (key !== undefined) {
            return 'Color.' + key;
        }
        return 'Color(' + String(value) + ')';
    }
}

export namespace Color {
    export function equals(a: Color, b: Color): boolean {
        return a === b;
    }
}

// ==================== TYPE ALIAS EXAMPLES ====================

/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
export type Point = {
    x: number;
    y: number;
};

export namespace Point {
    export function toString(value: Point): string {
        const parts: string[] = [];
        parts.push('x: ' + value.x);
        parts.push('y: ' + value.y);
        return 'Point { ' + parts.join(', ') + ' }';
    }
}

export namespace Point {
    export function clone(value: Point): Point {
        return { x: value.x, y: value.y };
    }
}

export namespace Point {
    export function equals(a: Point, b: Point): boolean {
        if (a === b) return true;
        return a.x === b.x && a.y === b.y;
    }
}

export namespace Point {
    export function toStringifiedJSON(value: Point): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(__serialize(value, ctx));
    }
    export function toObject(value: Point): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return __serialize(value, ctx);
    }
    export function __serialize(value: Point, ctx: SerializeContext): Record<string, unknown> {
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
}

export namespace Point {
    export function fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<Point, Array<{ field: string; message: string }>> {
        try {
            const raw = JSON.parse(json);
            return fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([{ field: '_root', message }]);
        }
    }
    export function fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<Point, Array<{ field: string; message: string }>> {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = __deserialize(obj, ctx);
            if (PendingRef.is(resultOrRef)) {
                return Result.err([
                    {
                        field: '_root',
                        message: 'Point.fromObject: root cannot be a forward reference'
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
    export function __deserialize(value: any, ctx: DeserializeContext): Point | PendingRef {
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
    export function validateField<K extends keyof Point>(
        field: K,
        value: Point[K]
    ): Array<{ field: string; message: string }> {
        return [];
    }
    export function validateFields(
        partial: Partial<Point>
    ): Array<{ field: string; message: string }> {
        return [];
    }
}

/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
export type UserProfile = {
    id: string;
    username: string;
    email: string;
    age: number;
    isVerified: boolean;
};

export namespace UserProfile {
    export function toString(value: UserProfile): string {
        const parts: string[] = [];
        parts.push('id: ' + value.id);
        parts.push('username: ' + value.username);
        parts.push('email: ' + value.email);
        parts.push('age: ' + value.age);
        parts.push('isVerified: ' + value.isVerified);
        return 'UserProfile { ' + parts.join(', ') + ' }';
    }
}

export namespace UserProfile {
    export function clone(value: UserProfile): UserProfile {
        return {
            id: value.id,
            username: value.username,
            email: value.email,
            age: value.age,
            isVerified: value.isVerified
        };
    }
}

export namespace UserProfile {
    export function equals(a: UserProfile, b: UserProfile): boolean {
        if (a === b) return true;
        return (
            a.id === b.id &&
            a.username === b.username &&
            a.email === b.email &&
            a.age === b.age &&
            a.isVerified === b.isVerified
        );
    }
}

export namespace UserProfile {
    export function toStringifiedJSON(value: UserProfile): string {
        const ctx = SerializeContext.create();
        return JSON.stringify(__serialize(value, ctx));
    }
    export function toObject(value: UserProfile): Record<string, unknown> {
        const ctx = SerializeContext.create();
        return __serialize(value, ctx);
    }
    export function __serialize(
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
}

export namespace UserProfile {
    export function fromStringifiedJSON(
        json: string,
        opts?: DeserializeOptions
    ): Result<UserProfile, Array<{ field: string; message: string }>> {
        try {
            const raw = JSON.parse(json);
            return fromObject(raw, opts);
        } catch (e) {
            if (e instanceof DeserializeError) {
                return Result.err(e.errors);
            }
            const message = e instanceof Error ? e.message : String(e);
            return Result.err([{ field: '_root', message }]);
        }
    }
    export function fromObject(
        obj: unknown,
        opts?: DeserializeOptions
    ): Result<UserProfile, Array<{ field: string; message: string }>> {
        try {
            const ctx = DeserializeContext.create();
            const resultOrRef = __deserialize(obj, ctx);
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
    export function __deserialize(value: any, ctx: DeserializeContext): UserProfile | PendingRef {
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
    export function validateField<K extends keyof UserProfile>(
        field: K,
        value: UserProfile[K]
    ): Array<{ field: string; message: string }> {
        return [];
    }
    export function validateFields(
        partial: Partial<UserProfile>
    ): Array<{ field: string; message: string }> {
        return [];
    }
}

/** @derive(Debug, Clone, PartialEq) */
export type Coordinate3D = {
    x: number;
    y: number;
    z: number;
};

export namespace Coordinate3D {
    export function toString(value: Coordinate3D): string {
        const parts: string[] = [];
        parts.push('x: ' + value.x);
        parts.push('y: ' + value.y);
        parts.push('z: ' + value.z);
        return 'Coordinate3D { ' + parts.join(', ') + ' }';
    }
}

export namespace Coordinate3D {
    export function clone(value: Coordinate3D): Coordinate3D {
        return { x: value.x, y: value.y, z: value.z };
    }
}

export namespace Coordinate3D {
    export function equals(a: Coordinate3D, b: Coordinate3D): boolean {
        if (a === b) return true;
        return a.x === b.x && a.y === b.y && a.z === b.z;
    }
}

/** @derive(Debug, PartialEq) */
export type ApiStatus = 'loading' | 'success' | 'error';

export namespace ApiStatus {
    export function toString(value: ApiStatus): string {
        return 'ApiStatus(' + JSON.stringify(value) + ')';
    }
}

export namespace ApiStatus {
    export function equals(a: ApiStatus, b: ApiStatus): boolean {
        if (a === b) return true;
        if (typeof a === 'object' && typeof b === 'object' && a !== null && b !== null) {
            return JSON.stringify(a) === JSON.stringify(b);
        }
        return false;
    }
}

// ==================== USAGE EXAMPLES ====================

// Enum usage
export const currentStatus = Status.Active;
export const highPriority = Priority.High;

// Using generated namespace functions on enums
export function demoEnumFunctions() {
    // Debug - toString
    console.log('Status string:', Status.toString(Status.Active));
    console.log('Priority string:', Priority.toString(Priority.High));

    // Clone - returns the same value for enums (primitives)
    const clonedStatus = Status.clone(Status.Pending);
    console.log('Cloned status:', clonedStatus);

    // Eq - equals and hashCode
    const areEqual = Status.equals(Status.Active, Status.Active);
    console.log('Are equal:', areEqual);
    const hash = Status.hashCode(Status.Active);
    console.log('Hash code:', hash);

    // Serialize - toJSON
    const json = Status.toJSON(Status.Inactive);
    console.log('Serialized:', json);

    // Deserialize - fromJSON
    const parsed = Status.fromJSON('pending');
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

// Using generated namespace functions on type aliases
export function demoTypeFunctions() {
    const point1: Point = { x: 10, y: 20 };
    const point2: Point = { x: 10, y: 20 };

    // Debug - toString
    console.log('Point string:', Point.toString(point1));
    console.log('User string:', UserProfile.toString(user));

    // Clone - creates a shallow copy
    const clonedPoint = Point.clone(point1);
    console.log('Cloned point:', clonedPoint);

    // Eq - equals and hashCode
    const pointsEqual = Point.equals(point1, point2);
    console.log('Points equal:', pointsEqual);
    const pointHash = Point.hashCode(point1);
    console.log('Point hash:', pointHash);

    // Serialize - toJSON
    const pointJson = Point.toJSON(point1);
    console.log('Point JSON:', pointJson);

    // Deserialize - fromJSON
    const parsedPoint = Point.fromJSON({ x: 5, y: 10 });
    console.log('Parsed point:', parsedPoint);
}

// Run demos
demoEnumFunctions();
demoTypeFunctions();
