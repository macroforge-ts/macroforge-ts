/**
 * Examples demonstrating derive macros on enums and type aliases.
 * These showcase the new enum and type alias support for all built-in macros.
 *
 * Generated functions use suffix naming style (default):
 * - toStringStatus, cloneStatus, equalsStatus, etc.
 * - toStringPoint, clonePoint, equalsPoint, etc.
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
