/**
 * Examples demonstrating derive macros on enums and type aliases.
 * These showcase the new enum and type alias support for all built-in macros.
 *
 * Generated functions use prefix naming style:
 * - statusToString, statusClone, statusEquals, etc.
 * - pointToString, pointClone, pointEquals, etc.
 */

// ==================== ENUM EXAMPLES ====================

/** @derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize) */
export enum Status {
    Active = 'active',
    Inactive = 'inactive',
    Pending = 'pending'
}

/** @derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize) */
export enum Priority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4
}

/** @derive(Debug, PartialEq, Hash) */
export enum Color {
    Red,
    Green,
    Blue
}

// ==================== TYPE ALIAS EXAMPLES ====================

/** @derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize) */
export type Point = {
    x: number;
    y: number;
};

/** @derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize) */
export type UserProfile = {
    id: string;
    username: string;
    email: string;
    age: number;
    isVerified: boolean;
};

/** @derive(Debug, Clone, PartialEq, Hash) */
export type Coordinate3D = {
    x: number;
    y: number;
    z: number;
};

/** @derive(Debug, PartialEq, Hash) */
export type ApiStatus = 'loading' | 'success' | 'error';

// ==================== USAGE EXAMPLES ====================

// Enum usage
export const currentStatus = Status.Active;
export const highPriority = Priority.High;

// Using generated standalone functions on enums (prefix naming style)
export function demoEnumFunctions() {
    // Debug - statusToString
    console.log('Status string:', statusToString(Status.Active));
    console.log('Priority string:', priorityToString(Priority.High));

    // Clone - returns the same value for enums (primitives)
    const clonedStatus = statusClone(Status.Pending);
    console.log('Cloned status:', clonedStatus);

    // PartialEq - statusEquals
    const areEqual = statusEquals(Status.Active, Status.Active);
    console.log('Are equal:', areEqual);

    // Hash - statusHashCode
    const hash = statusHashCode(Status.Active);
    console.log('Hash code:', hash);

    // Serialize - statusSerialize
    const json = statusSerialize(Status.Inactive);
    console.log('Serialized:', json);

    // Deserialize - statusDeserialize
    const parsed = statusDeserialize('pending');
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

// Using generated standalone functions on type aliases (prefix naming style)
export function demoTypeFunctions() {
    const point1: Point = { x: 10, y: 20 };
    const point2: Point = { x: 10, y: 20 };

    // Debug - pointToString
    console.log('Point string:', pointToString(point1));
    console.log('User string:', userProfileToString(user));

    // Clone - creates a shallow copy
    const clonedPoint = pointClone(point1);
    console.log('Cloned point:', clonedPoint);

    // PartialEq - pointEquals
    const pointsEqual = pointEquals(point1, point2);
    console.log('Points equal:', pointsEqual);

    // Hash - pointHashCode
    const pointHash = pointHashCode(point1);
    console.log('Point hash:', pointHash);

    // Serialize - pointSerialize
    const pointJson = pointSerialize(point1);
    console.log('Point JSON:', pointJson);

    // Deserialize - pointDeserialize
    const parsedPoint = pointDeserialize({ x: 5, y: 10 });
    console.log('Parsed point:', parsedPoint);
}

// Run demos
demoEnumFunctions();
demoTypeFunctions();
