/**
 * # Macroforge Structural Module
 *
 * Runtime fallbacks for the `PartialEq`, `Hash`, `Clone`, `Debug`, `PartialOrd`
 * and `Ord` derives. Derived functions dispatch on each field's declared type;
 * these run where the declaration says nothing about the runtime shape:
 * unions, object literals, tuples, `unknown`, and named types the macro could
 * not resolve. They agree: `structuralEquals(a, b)` implies equal
 * `structuralHash` values and a `structuralCompare` of `0`, and a clone equals
 * its source.
 *
 * @module @macroforge/core/structural
 */

type TraitName = 'equals' | 'hashCode' | 'clone' | 'toString' | 'compare';

/**
 * The static trait function a derived class carries, such as `User.equals`.
 * Only the class's own statics count: every constructor inherits
 * `Function.prototype.toString`, which prints the class's source.
 */
function classTrait(value: object, name: TraitName): Function | undefined {
  const constructor: unknown = value.constructor;
  if (typeof constructor !== 'function') return undefined;
  if (!Object.prototype.hasOwnProperty.call(constructor, name)) return undefined;
  const trait: unknown = Reflect.get(constructor, name);
  return typeof trait === 'function' ? trait : undefined;
}

/** Whether `value` is a plain object literal rather than a class instance. */
function isPlainObject(value: object): boolean {
  const prototype: unknown = Object.getPrototypeOf(value);
  return prototype === Object.prototype || prototype === null;
}

/** Whether `value` is a `Headers`, in runtimes that have the class. */
function isHeaders(value: object): value is Headers {
  return typeof Headers !== 'undefined' && value instanceof Headers;
}

function bytesOf(value: ArrayBuffer | ArrayBufferView): Uint8Array {
  return value instanceof ArrayBuffer
    ? new Uint8Array(value)
    : new Uint8Array(value.buffer, value.byteOffset, value.byteLength);
}

function bytesEqual(left: Uint8Array, right: Uint8Array): boolean {
  return left.length === right.length && left.every((byte, index) => byte === right[index]);
}

/**
 * Structural equality: derived classes by their own `equals`, built-ins by
 * value, arrays, maps and sets by their contents (a map's keys by identity),
 * and plain objects by their own enumerable properties in any order.
 * Instances of other classes are equal only when identical. Cycles compare
 * equal where they close.
 */
export function structuralEquals(left: unknown, right: unknown): boolean {
  return equalsWithin(left, right, new Map());
}

function equalsWithin(left: unknown, right: unknown, seen: Map<object, Set<object>>): boolean {
  if (left === right) return true;
  if (typeof left !== 'object' || typeof right !== 'object' || left === null || right === null) {
    return false;
  }
  if (Object.getPrototypeOf(left) !== Object.getPrototypeOf(right)) return false;
  const partners = seen.get(left);
  if (partners?.has(right)) return true;
  if (partners) partners.add(right);
  else seen.set(left, new Set([right]));

  const equals = classTrait(left, 'equals');
  if (equals) return equals(left, right) === true;
  if (left instanceof Date && right instanceof Date) return left.getTime() === right.getTime();
  if (left instanceof RegExp && right instanceof RegExp) {
    return left.source === right.source && left.flags === right.flags;
  }
  if (left instanceof URL && right instanceof URL) return left.href === right.href;
  if (left instanceof URLSearchParams && right instanceof URLSearchParams) {
    return left.toString() === right.toString();
  }
  if (left instanceof Error && right instanceof Error) return left.message === right.message;
  if (isHeaders(left) && isHeaders(right)) {
    return equalsWithin(Array.from(left), Array.from(right), seen);
  }
  if (Array.isArray(left) && Array.isArray(right)) {
    return (
      left.length === right.length &&
      left.every((item, index) => equalsWithin(item, right[index], seen))
    );
  }
  if (left instanceof Map && right instanceof Map) {
    return (
      left.size === right.size &&
      Array.from(left).every(
        ([key, item]) => right.has(key) && equalsWithin(item, right.get(key), seen),
      )
    );
  }
  if (left instanceof Set && right instanceof Set) {
    return (
      left.size === right.size &&
      Array.from(left).every(
        (item) =>
          right.has(item) || Array.from(right).some((other) => equalsWithin(item, other, seen)),
      )
    );
  }
  if (
    (left instanceof ArrayBuffer || ArrayBuffer.isView(left)) &&
    (right instanceof ArrayBuffer || ArrayBuffer.isView(right))
  ) {
    return bytesEqual(bytesOf(left), bytesOf(right));
  }
  if (!isPlainObject(left)) return false;
  const leftEntries = Object.entries(left);
  const rightEntries = new Map(Object.entries(right));
  return (
    leftEntries.length === rightEntries.size &&
    leftEntries.every(
      ([key, item]) => rightEntries.has(key) && equalsWithin(item, rightEntries.get(key), seen),
    )
  );
}

function stringHash(text: string): number {
  let hash = 0;
  for (let index = 0; index < text.length; index++) {
    hash = (hash * 31 + text.charCodeAt(index)) | 0;
  }
  return hash;
}

/**
 * A 32-bit hash agreeing with {@link structuralEquals}: derived classes by
 * their own `hashCode`, maps, sets and plain objects independent of order.
 */
export function structuralHash(value: unknown): number {
  return hashWithin(value, new Set());
}

function hashWithin(value: unknown, path: Set<object>): number {
  switch (typeof value) {
    case 'number':
      return Number.isInteger(value) ? value | 0 : stringHash(value.toString());
    case 'bigint':
      return stringHash(value.toString());
    case 'string':
      return stringHash(value);
    case 'boolean':
      return value ? 1231 : 1237;
    case 'object':
      break;
    case 'undefined':
    case 'symbol':
    case 'function':
      return 0;
  }
  // Only the objects being hashed above this one are skipped, so a value
  // reached twice through separate fields hashes the same both times.
  if (value === null || path.has(value)) return 0;
  path.add(value);
  const hash = hashObject(value, path);
  path.delete(value);
  return hash;
}

function hashObject(value: object, path: Set<object>): number {
  const hashCode = classTrait(value, 'hashCode');
  if (hashCode) return Number(hashCode(value)) | 0;
  if (value instanceof Date) return value.getTime() | 0;
  if (value instanceof RegExp) return stringHash(value.source + value.flags);
  if (value instanceof URL) return stringHash(value.href);
  if (value instanceof URLSearchParams) return stringHash(value.toString());
  if (value instanceof Error) return stringHash(value.message);
  if (isHeaders(value)) return hashWithin(Array.from(value), path);
  if (Array.isArray(value)) {
    return value.reduce<number>((hash, item) => (hash * 31 + hashWithin(item, path)) | 0, 1);
  }
  // Maps, sets and plain objects sum their entries, since equality ignores
  // their order.
  if (value instanceof Map) {
    return Array.from(value).reduce<number>(
      (hash, [key, item]) => (hash + ((hashWithin(key, path) * 31 + hashWithin(item, path)) | 0)) | 0,
      0,
    );
  }
  if (value instanceof Set) {
    return Array.from(value).reduce<number>((hash, item) => (hash + hashWithin(item, path)) | 0, 0);
  }
  if (value instanceof ArrayBuffer || ArrayBuffer.isView(value)) {
    return bytesOf(value).reduce((hash, byte) => (hash * 31 + byte) | 0, 0);
  }
  if (!isPlainObject(value)) return 0;
  return Object.entries(value).reduce(
    (hash, [key, item]) => (hash + ((stringHash(key) * 31 + hashWithin(item, path)) | 0)) | 0,
    0,
  );
}

/**
 * A deep copy agreeing with {@link structuralEquals}: derived classes by
 * their own `clone`, built-ins and collections rebuilt, plain objects copied
 * property by property. Instances of other classes, and a map's keys, are
 * shared rather than copied, since equality sees their identity.
 */
export function structuralClone<T>(value: T): T;
export function structuralClone(value: unknown): unknown {
  return cloneWithin(value, new Map());
}

function cloneWithin(value: unknown, seen: Map<object, unknown>): unknown {
  if (typeof value !== 'object' || value === null) return value;
  if (seen.has(value)) return seen.get(value);

  const clone = classTrait(value, 'clone');
  if (clone) return clone(value);
  if (value instanceof Date) return new Date(value.getTime());
  if (value instanceof RegExp) return new RegExp(value.source, value.flags);
  if (value instanceof URL) return new URL(value.href);
  if (value instanceof URLSearchParams) return new URLSearchParams(value);
  if (isHeaders(value)) return new Headers(value);
  if (Array.isArray(value)) {
    const copy: unknown[] = [];
    seen.set(value, copy);
    for (const item of value) copy.push(cloneWithin(item, seen));
    return copy;
  }
  if (value instanceof Map) {
    const copy = new Map<unknown, unknown>();
    seen.set(value, copy);
    for (const [key, item] of value) copy.set(key, cloneWithin(item, seen));
    return copy;
  }
  if (value instanceof Set) {
    const copy = new Set<unknown>();
    seen.set(value, copy);
    for (const item of value) copy.add(cloneWithin(item, seen));
    return copy;
  }
  if (value instanceof ArrayBuffer) return value.slice(0);
  if (ArrayBuffer.isView(value)) {
    const bytes = bytesOf(value).slice();
    return value instanceof DataView
      ? new DataView(bytes.buffer)
      : Reflect.construct(value.constructor, [bytes.buffer]);
  }
  if (!isPlainObject(value)) return value;
  const copy: Record<string, unknown> = Object.create(Object.getPrototypeOf(value));
  seen.set(value, copy);
  for (const [key, item] of Object.entries(value)) copy[key] = cloneWithin(item, seen);
  return copy;
}

/**
 * A readable rendering for `Debug`: derived classes by their own `toString`,
 * primitives and built-ins as `String` renders them, collections and objects
 * by their contents. A cycle renders as `[Circular]`.
 */
export function structuralDebug(value: unknown): string {
  return debugWithin(value, new Set());
}

function debugWithin(value: unknown, path: Set<object>): string {
  if (typeof value === 'symbol') return value.toString();
  if (typeof value !== 'object' || value === null) return String(value);
  if (path.has(value)) return '[Circular]';
  path.add(value);
  const rendered = debugObject(value, path);
  path.delete(value);
  return rendered;
}

function debugObject(value: object, path: Set<object>): string {
  const toString = classTrait(value, 'toString');
  if (toString) return String(toString(value));
  if (Array.isArray(value)) {
    return '[' + value.map((item) => debugWithin(item, path)).join(', ') + ']';
  }
  if (value instanceof Map) {
    const entries = Array.from(value, ([key, item]) => {
      return debugWithin(key, path) + ': ' + debugWithin(item, path);
    });
    return 'Map {' + (entries.length ? ' ' + entries.join(', ') + ' ' : '') + '}';
  }
  if (value instanceof Set) {
    const items = Array.from(value, (item) => debugWithin(item, path));
    return 'Set {' + (items.length ? ' ' + items.join(', ') + ' ' : '') + '}';
  }
  if (
    value instanceof Date ||
    value instanceof RegExp ||
    value instanceof URL ||
    value instanceof URLSearchParams ||
    value instanceof Error ||
    value instanceof ArrayBuffer ||
    ArrayBuffer.isView(value)
  ) {
    return String(value);
  }
  const fields = Object.entries(value).map(([key, item]) => key + ': ' + debugWithin(item, path));
  const name = isPlainObject(value) ? '' : value.constructor.name + ' ';
  return name + '{' + (fields.length ? ' ' + fields.join(', ') + ' ' : '') + '}';
}

/**
 * A total order agreeing with {@link structuralEquals}: `0` exactly when the
 * values are structurally equal. Values of different kinds order by kind
 * (`undefined`, `null`, booleans, numbers, bigints, strings, symbols,
 * functions, objects); strings by UTF-16 code units, which is what `===`
 * distinguishes; derived classes by their own `compare`. Values equal only
 * when identical (symbols, functions, other class instances) order by when
 * this function first saw them.
 */
export function structuralCompare(left: unknown, right: unknown): number {
  if (structuralEquals(left, right)) return 0;
  return orderWithin(left, right) || identityOrder(left, right);
}

const identities = new WeakMap<object, number>();
const symbolIdentities = new Map<symbol, number>();
let nextIdentity = 0;

function identityOf(value: unknown): number {
  if (typeof value === 'symbol') {
    const known = symbolIdentities.get(value);
    if (known !== undefined) return known;
    symbolIdentities.set(value, ++nextIdentity);
    return nextIdentity;
  }
  if ((typeof value === 'object' && value !== null) || typeof value === 'function') {
    const known = identities.get(value);
    if (known !== undefined) return known;
    identities.set(value, ++nextIdentity);
    return nextIdentity;
  }
  return 0;
}

function identityOrder(left: unknown, right: unknown): number {
  return Math.sign(identityOf(left) - identityOf(right));
}

function kindRank(value: unknown): number {
  if (value === null) return 1;
  switch (typeof value) {
    case 'undefined':
      return 0;
    case 'boolean':
      return 2;
    case 'number':
      return 3;
    case 'bigint':
      return 4;
    case 'string':
      return 5;
    case 'symbol':
      return 6;
    case 'function':
      return 7;
    default:
      return 8;
  }
}

/** `-1`, `0` or `1` for two numbers; `NaN` orders after every number. */
function numberOrder(left: number, right: number): number {
  if (left < right) return -1;
  if (left > right) return 1;
  if (Number.isNaN(left)) return Number.isNaN(right) ? 0 : 1;
  return Number.isNaN(right) ? -1 : 0;
}

/** `-1`, `0` or `1` for two strings, by UTF-16 code units. */
function textOrder(left: string, right: string): number {
  return left < right ? -1 : left > right ? 1 : 0;
}

function orderWithin(left: unknown, right: unknown): number {
  const rank = kindRank(left) - kindRank(right);
  if (rank !== 0) return Math.sign(rank);
  if (typeof left === 'boolean' && typeof right === 'boolean') return left ? 1 : -1;
  if (typeof left === 'number' && typeof right === 'number') return numberOrder(left, right);
  if (typeof left === 'bigint' && typeof right === 'bigint') {
    return left < right ? -1 : left > right ? 1 : 0;
  }
  if (typeof left === 'string' && typeof right === 'string') return textOrder(left, right);
  if (typeof left === 'object' && typeof right === 'object' && left !== null && right !== null) {
    return orderObjects(left, right);
  }
  return 0;
}

/** Built-ins first, then collections, then plain objects, then other classes. */
function objectRank(value: object): number {
  if (value instanceof Date) return 0;
  if (value instanceof RegExp) return 1;
  if (value instanceof URL) return 2;
  if (value instanceof URLSearchParams) return 3;
  if (value instanceof Error) return 4;
  if (isHeaders(value)) return 5;
  if (value instanceof ArrayBuffer || ArrayBuffer.isView(value)) return 6;
  if (Array.isArray(value)) return 7;
  if (value instanceof Map) return 8;
  if (value instanceof Set) return 9;
  if (isPlainObject(value)) return 10;
  return 11;
}

function orderObjects(left: object, right: object): number {
  if (Object.getPrototypeOf(left) === Object.getPrototypeOf(right)) {
    const compare = classTrait(left, 'compare');
    if (compare) return Math.sign(Number(compare(left, right)));
  }
  const rank = objectRank(left) - objectRank(right);
  if (rank !== 0) return Math.sign(rank);
  if (left instanceof Date && right instanceof Date) {
    return numberOrder(left.getTime(), right.getTime());
  }
  if (left instanceof RegExp && right instanceof RegExp) {
    return textOrder(left.source + '/' + left.flags, right.source + '/' + right.flags);
  }
  if (left instanceof URL && right instanceof URL) return textOrder(left.href, right.href);
  if (left instanceof URLSearchParams && right instanceof URLSearchParams) {
    return textOrder(left.toString(), right.toString());
  }
  if (left instanceof Error && right instanceof Error) {
    return textOrder(left.message, right.message);
  }
  if (isHeaders(left) && isHeaders(right)) {
    return orderSequences(Array.from(left), Array.from(right));
  }
  if (
    (left instanceof ArrayBuffer || ArrayBuffer.isView(left)) &&
    (right instanceof ArrayBuffer || ArrayBuffer.isView(right))
  ) {
    return orderSequences(Array.from(bytesOf(left)), Array.from(bytesOf(right)));
  }
  if (Array.isArray(left) && Array.isArray(right)) return orderSequences(left, right);
  // Collections and objects order by their entries sorted, so the order
  // ignores insertion order exactly as equality does.
  if (left instanceof Map && right instanceof Map) {
    return orderSequences(sortedEntries(Array.from(left)), sortedEntries(Array.from(right)));
  }
  if (left instanceof Set && right instanceof Set) {
    const leftItems = Array.from(left).sort(structuralCompare);
    return orderSequences(leftItems, Array.from(right).sort(structuralCompare));
  }
  if (isPlainObject(left) && isPlainObject(right)) {
    const leftEntries = sortedEntries(Object.entries(left));
    return orderSequences(leftEntries, sortedEntries(Object.entries(right)));
  }
  return 0;
}

function sortedEntries(entries: [unknown, unknown][]): [unknown, unknown][] {
  return entries.sort(([leftKey], [rightKey]) => structuralCompare(leftKey, rightKey));
}

/** Lexicographic: the first unequal element decides, then the length. */
function orderSequences(left: readonly unknown[], right: readonly unknown[]): number {
  const shared = Math.min(left.length, right.length);
  for (let index = 0; index < shared; index++) {
    const order = structuralCompare(left[index], right[index]);
    if (order !== 0) return order;
  }
  return Math.sign(left.length - right.length);
}
