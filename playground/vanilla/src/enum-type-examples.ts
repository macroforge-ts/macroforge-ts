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

/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
export enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4
}

/** @derive(Debug, PartialEq) */
export enum Color {
    Red,
    Green,
    Blue
}

// ==================== TYPE ALIAS EXAMPLES ====================

/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
export type Point = {
    x: number;
    y: number;
};

/** @derive(Debug, Clone, PartialEq, Serialize, Deserialize) */
export type UserProfile = {
    id: string;
    username: string;
    email: string;
    age: number;
    isVerified: boolean;
};

/** @derive(Debug, Clone, PartialEq) */
export type Coordinate3D = {
    x: number;
    y: number;
    z: number;
};

/** @derive(Debug, PartialEq) */
export type ApiStatus = 'loading' | 'success' | 'error';

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
