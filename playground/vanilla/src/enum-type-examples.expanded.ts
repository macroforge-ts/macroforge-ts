/**
 * Examples demonstrating derive macros on enums and type aliases.
 * These showcase the new enum and type alias support for all built-in macros.
 */

// ==================== ENUM EXAMPLES ====================

/**  */
export enum Status {
  Active = "active",
  Inactive = "inactive",
  Pending = "pending",
}

export namespace Status {export function toString(value: Status): string {const key = Status[value as unknown as keyof typeof Status]; if(key!== undefined){return "Status." + key;}return "Status(" + String(value)+ ")" ; }}

export namespace Status {export function clone(value: Status): Status {return value;}}

export namespace Status {export function equals(a: Status, b: Status): boolean {return a === b;}}

export namespace Status {export function toJSON(value: Status): string | number {return value;}}

export namespace Status {export function fromJSON(data: unknown): Status {for(const key of Object.keys(Status)){const enumValue = Status[key as keyof typeof Status]; if(enumValue === data){return data as Status;}}throw new Error("Invalid Status value: " + JSON.stringify(data));}}

/**  */
export enum Priority {
  Low = 1,
  Medium = 2,
  High = 3,
  Critical = 4,
}

export namespace Priority {export function toString(value: Priority): string {const key = Priority[value as unknown as keyof typeof Priority]; if(key!== undefined){return "Priority." + key;}return "Priority(" + String(value)+ ")" ; }}

export namespace Priority {export function clone(value: Priority): Priority {return value;}}

export namespace Priority {export function equals(a: Priority, b: Priority): boolean {return a === b;}}

export namespace Priority {export function toJSON(value: Priority): string | number {return value;}}

export namespace Priority {export function fromJSON(data: unknown): Priority {for(const key of Object.keys(Priority)){const enumValue = Priority[key as keyof typeof Priority]; if(enumValue === data){return data as Priority;}}throw new Error("Invalid Priority value: " + JSON.stringify(data));}}

/**  */
export enum Color {
  Red,
  Green,
  Blue,
}

export namespace Color {export function toString(value: Color): string {const key = Color[value as unknown as keyof typeof Color]; if(key!== undefined){return "Color." + key;}return "Color(" + String(value)+ ")" ; }}

export namespace Color {export function equals(a: Color, b: Color): boolean {return a === b;}}

// ==================== TYPE ALIAS EXAMPLES ====================

/**  */
export type Point = {
  x: number;
  y: number;
};

export namespace Point {export function toString(value: Point): string {const parts: string[]= []; parts.push("x: " + value.x); parts.push("y: " + value.y); return "Point { " + parts.join(", " )+ " }" ; }}

export namespace Point {export function clone(value: Point): Point {return {x: value.x, y: value.y, };}}

export namespace Point {export function equals(a: Point, b: Point): boolean {if(a === b)return true; return a.x === b.x && a.y === b.y;}}

export namespace Point {export function toJSON(value: Point): Record<string, unknown> {const result: Record<string, unknown> = {}; result["x" ]= value.x; result["y" ]= value.y; return result;}}

export namespace Point {export function is(data: unknown): data is Point {if(typeof data!== "object" || data === null || Array.isArray(data)){return false;}const obj = data as Record<string, unknown>; if(!("x" in obj)){return false;}if(!("y" in obj)){return false;}return true;}export function fromJSON(data: unknown): Point {if(!is(data)){throw new Error("Invalid Point: type validation failed" );}return data;}}

/**  */
export type UserProfile = {
  id: string;
  username: string;
  email: string;
  age: number;
  isVerified: boolean;
};

export namespace UserProfile {export function toString(value: UserProfile): string {const parts: string[]= []; parts.push("id: " + value.id); parts.push("username: " + value.username); parts.push("email: " + value.email); parts.push("age: " + value.age); parts.push("isVerified: " + value.isVerified); return "UserProfile { " + parts.join(", " )+ " }" ; }}

export namespace UserProfile {export function clone(value: UserProfile): UserProfile {return {id: value.id, username: value.username, email: value.email, age: value.age, isVerified: value.isVerified, };}}

export namespace UserProfile {export function equals(a: UserProfile, b: UserProfile): boolean {if(a === b)return true; return a.id === b.id && a.username === b.username && a.email === b.email && a.age === b.age && a.isVerified === b.isVerified;}}

export namespace UserProfile {export function toJSON(value: UserProfile): Record<string, unknown> {const result: Record<string, unknown> = {}; result["id" ]= value.id; result["username" ]= value.username; result["email" ]= value.email; result["age" ]= value.age; result["isVerified" ]= value.isVerified; return result;}}

export namespace UserProfile {export function is(data: unknown): data is UserProfile {if(typeof data!== "object" || data === null || Array.isArray(data)){return false;}const obj = data as Record<string, unknown>; if(!("id" in obj)){return false;}if(!("username" in obj)){return false;}if(!("email" in obj)){return false;}if(!("age" in obj)){return false;}if(!("isVerified" in obj)){return false;}return true;}export function fromJSON(data: unknown): UserProfile {if(!is(data)){throw new Error("Invalid UserProfile: type validation failed" );}return data;}}

/**  */
export type Coordinate3D = {
  x: number;
  y: number;
  z: number;
};

export namespace Coordinate3D {export function toString(value: Coordinate3D): string {const parts: string[]= []; parts.push("x: " + value.x); parts.push("y: " + value.y); parts.push("z: " + value.z); return "Coordinate3D { " + parts.join(", " )+ " }" ; }}

export namespace Coordinate3D {export function clone(value: Coordinate3D): Coordinate3D {return {x: value.x, y: value.y, z: value.z, };}}

export namespace Coordinate3D {export function equals(a: Coordinate3D, b: Coordinate3D): boolean {if(a === b)return true; return a.x === b.x && a.y === b.y && a.z === b.z;}}

/**  */
export type ApiStatus = "loading" | "success" | "error";

export namespace ApiStatus {export function toString(value: ApiStatus): string {return "ApiStatus(" + JSON.stringify(value)+ ")" ;}}

export namespace ApiStatus {export function equals(a: ApiStatus, b: ApiStatus): boolean {if(a === b)return true; if(typeof a === "object" && typeof b === "object" && a!== null && b!== null){return JSON.stringify(a)=== JSON.stringify(b);}return false;}}

// ==================== USAGE EXAMPLES ====================

// Enum usage
export const currentStatus = Status.Active;
export const highPriority = Priority.High;

// Using generated namespace functions on enums
export function demoEnumFunctions() {
  // Debug - toString
  console.log("Status string:", Status.toString(Status.Active));
  console.log("Priority string:", Priority.toString(Priority.High));

  // Clone - returns the same value for enums (primitives)
  const clonedStatus = Status.clone(Status.Pending);
  console.log("Cloned status:", clonedStatus);

  // Eq - equals and hashCode
  const areEqual = Status.equals(Status.Active, Status.Active);
  console.log("Are equal:", areEqual);
  const hash = Status.hashCode(Status.Active);
  console.log("Hash code:", hash);

  // Serialize - toJSON
  const json = Status.toJSON(Status.Inactive);
  console.log("Serialized:", json);

  // Deserialize - fromJSON
  const parsed = Status.fromJSON("pending");
  console.log("Parsed:", parsed);
}

// Type alias usage
export const origin: Point = { x: 0, y: 0 };
export const user: UserProfile = {
  id: "user-123",
  username: "johndoe",
  email: "john@example.com",
  age: 30,
  isVerified: true,
};

// Using generated namespace functions on type aliases
export function demoTypeFunctions() {
  const point1: Point = { x: 10, y: 20 };
  const point2: Point = { x: 10, y: 20 };

  // Debug - toString
  console.log("Point string:", Point.toString(point1));
  console.log("User string:", UserProfile.toString(user));

  // Clone - creates a shallow copy
  const clonedPoint = Point.clone(point1);
  console.log("Cloned point:", clonedPoint);

  // Eq - equals and hashCode
  const pointsEqual = Point.equals(point1, point2);
  console.log("Points equal:", pointsEqual);
  const pointHash = Point.hashCode(point1);
  console.log("Point hash:", pointHash);

  // Serialize - toJSON
  const pointJson = Point.toJSON(point1);
  console.log("Point JSON:", pointJson);

  // Deserialize - fromJSON
  const parsedPoint = Point.fromJSON({ x: 5, y: 10 });
  console.log("Parsed point:", parsedPoint);
}

// Run demos
demoEnumFunctions();
demoTypeFunctions();