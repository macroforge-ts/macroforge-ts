// Re-export Result and Option from @rydshift/mirror for use in generated code
export { Result, Option } from "@rydshift/mirror/declarative";

/**
 *
 *
 * // Option types

 export interface Some<T> {
	readonly tag: "Some";
	readonly value: T;
 }

 export interface None {
	readonly tag: "None";
 }

 export type Option<T> = Some<T> | None;

 export namespace Option {
	export function some<T>(value: T): Option<T> {
		return { tag: "Some", value };
	}

	export function none<T>(): Option<T> {
		return { tag: "None" };
	}

	export function isSome<T>(opt: Option<T>): opt is Some<T> {
		return opt.tag === "Some";
	}

	export function isNone<T>(opt: Option<T>): opt is None {
		return opt.tag === "None";
	}

	export function isSomeAnd<T>(opt: Option<T>, f: (value: T) => boolean): boolean {
		return isSome(opt) && f(opt.value);
	}

	export function isNoneOr<T>(opt: Option<T>, f: (value: T) => boolean): boolean {
		return isNone(opt) || f(opt.value);
	}

	export function expect<T>(opt: Option<T>, msg: string): T {
		if (isSome(opt)) return opt.value;
		throw new Error(msg);
	}

	export function unwrap<T>(opt: Option<T>): T {
		if (isSome(opt)) return opt.value;
		throw new Error("called `Option::unwrap()` on a `None` value");
	}

	export function unwrapOr<T>(opt: Option<T>, defaultValue: T): T {
		return isSome(opt) ? opt.value : defaultValue;
	}

	export function unwrapOrElse<T>(opt: Option<T>, f: () => T): T {
		return isSome(opt) ? opt.value : f();
	}

	export function unwrapOrDefault<T>(opt: Option<T>): T {
		return isSome(opt) ? opt.value : ("" as unknown as T);
	}

	export function map<T, U>(opt: Option<T>, f: (value: T) => U): Option<U> {
		return isSome(opt) ? some(f(opt.value)) : none();
	}

	export function mapOr<T, U>(opt: Option<T>, defaultValue: U, f: (value: T) => U): U {
		return isSome(opt) ? f(opt.value) : defaultValue;
	}

	export function mapOrElse<T, U>(opt: Option<T>, defaultFn: () => U, f: (value: T) => U): U {
		return isSome(opt) ? f(opt.value) : defaultFn();
	}

	export function inspect<T>(opt: Option<T>, f: (value: T) => void): Option<T> {
		if (isSome(opt)) f(opt.value);
		return opt;
	}

	export function okOr<T, E>(opt: Option<T>, err: E): Result<T, E> {
		return isSome(opt) ? Result.ok(opt.value) : Result.err(err);
	}

	export function okOrElse<T, E>(opt: Option<T>, errFn: () => E): Result<T, E> {
		return isSome(opt) ? Result.ok(opt.value) : Result.err(errFn());
	}

	export function and<T, U>(opt: Option<T>, other: Option<U>): Option<U> {
		return isSome(opt) ? other : none();
	}

	export function andThen<T, U>(opt: Option<T>, f: (value: T) => Option<U>): Option<U> {
		return isSome(opt) ? f(opt.value) : none();
	}

	export function or<T>(opt: Option<T>, other: Option<T>): Option<T> {
		return isSome(opt) ? opt : other;
	}

	export function orElse<T>(opt: Option<T>, f: () => Option<T>): Option<T> {
		return isSome(opt) ? opt : f();
	}

	export function filter<T>(opt: Option<T>, predicate: (value: T) => boolean): Option<T> {
		return isSome(opt) && predicate(opt.value) ? opt : none();
	}

	export function xor<T>(opt: Option<T>, other: Option<T>): Option<T> {
		if (isSome(opt) && isNone(other)) return opt;
		if (isNone(opt) && isSome(other)) return other;
		return none();
	}

	export function zip<T, U>(opt: Option<T>, other: Option<U>): Option<[T, U]> {
		return isSome(opt) && isSome(other) ? some([opt.value, other.value]) : none();
	}

	export function zipWith<T, U, R>(opt: Option<T>, other: Option<U>, f: (a: T, b: U) => R): Option<R> {
		return isSome(opt) && isSome(other) ? some(f(opt.value, other.value)) : none();
	}

	export function flatten<T>(opt: Option<Option<T>>): Option<T> {
		return isSome(opt) ? opt.value : none();
	}

	export function transpose<T, E>(opt: Option<Result<T, E>>): Result<Option<T>, E> {
		if (isNone(opt)) return Result.ok(none());
		if (Result.isOk(opt.value)) return Result.ok(some(opt.value.value));
		return Result.err(opt.value.error);
	}

	export function match<T, U>(opt: Option<T>, onSome: (value: T) => U, onNone: () => U): U {
		return isSome(opt) ? onSome(opt.value) : onNone();
	}
 }

 // Result types

 export interface Ok<T> {
	readonly tag: "Ok";
	readonly value: T;
 }

 export interface Err<E> {
	readonly tag: "Err";
	readonly error: E;
 }

 export type Result<T, E> = Ok<T> | Err<E>;

 export namespace Result {
	export function ok<T, E = never>(value: T): Result<T, E> {
		return { tag: "Ok", value };
	}

	export function err<T = never, E = unknown>(error: E): Result<T, E> {
		return { tag: "Err", error };
	}

	export function isOk<T, E>(res: Result<T, E>): res is Ok<T> {
		return res.tag === "Ok";
	}

	export function isErr<T, E>(res: Result<T, E>): res is Err<E> {
		return res.tag === "Err";
	}

	export function isOkAnd<T, E>(res: Result<T, E>, f: (value: T) => boolean): boolean {
		return isOk(res) && f(res.value);
	}

	export function isErrAnd<T, E>(res: Result<T, E>, f: (error: E) => boolean): boolean {
		return isErr(res) && f(res.error);
	}

	export function toOk<T, E>(res: Result<T, E>): Option<T> {
		return isOk(res) ? Option.some(res.value) : Option.none();
	}

	export function toErr<T, E>(res: Result<T, E>): Option<E> {
		return isErr(res) ? Option.some(res.error) : Option.none();
	}

	export function expect<T, E>(res: Result<T, E>, msg: string): T {
		if (isOk(res)) return res.value;
		throw new Error(`${msg}: ${res.error}`);
	}

	export function expectErr<T, E>(res: Result<T, E>, msg: string): E {
		if (isErr(res)) return res.error;
		throw new Error(`${msg}: ${res.value}`);
	}

	export function unwrap<T, E>(res: Result<T, E>): T {
		if (isOk(res)) return res.value;
		throw new Error(`called \`Result::unwrap()\` on an \`Err\` value: ${(res as Err<E>).error}`);
	}

	export function unwrapErr<T, E>(res: Result<T, E>): E {
		if (isErr(res)) return res.error;
		throw new Error(`called \`Result::unwrap_err()\` on an \`Ok\` value: ${(res as Ok<T>).value}`);
	}

	export function unwrapOr<T, E>(res: Result<T, E>, defaultValue: T): T {
		return isOk(res) ? res.value : defaultValue;
	}

	export function unwrapOrElse<T, E>(res: Result<T, E>, f: (error: E) => T): T {
		return isOk(res) ? res.value : f(res.error);
	}

	export function unwrapOrDefault<T, E>(res: Result<T, E>): T {
		return isOk(res) ? res.value : ("" as unknown as T);
	}

	export function map<T, U, E>(res: Result<T, E>, f: (value: T) => U): Result<U, E> {
		return isOk(res) ? ok(f(res.value)) : err(res.error);
	}

	export function mapOr<T, U, E>(res: Result<T, E>, defaultValue: U, f: (value: T) => U): U {
		return isOk(res) ? f(res.value) : defaultValue;
	}

	export function mapOrElse<T, U, E>(res: Result<T, E>, defaultFn: (error: E) => U, f: (value: T) => U): U {
		return isOk(res) ? f(res.value) : defaultFn(res.error);
	}

	export function mapErr<T, E, F>(res: Result<T, E>, f: (error: E) => F): Result<T, F> {
		return isOk(res) ? ok(res.value) : err(f(res.error));
	}

	export function inspect<T, E>(res: Result<T, E>, f: (value: T) => void): Result<T, E> {
		if (isOk(res)) f(res.value);
		return res;
	}

	export function inspectErr<T, E>(res: Result<T, E>, f: (error: E) => void): Result<T, E> {
		if (isErr(res)) f(res.error);
		return res;
	}

	export function and<T, U, E>(res: Result<T, E>, other: Result<U, E>): Result<U, E> {
		return isOk(res) ? other : err(res.error);
	}

	export function andThen<T, U, E>(res: Result<T, E>, f: (value: T) => Result<U, E>): Result<U, E> {
		return isOk(res) ? f(res.value) : err(res.error);
	}

	export function or<T, E, F>(res: Result<T, E>, other: Result<T, F>): Result<T, F> {
		return isOk(res) ? ok(res.value) : other;
	}

	export function orElse<T, E, F>(res: Result<T, E>, f: (error: E) => Result<T, F>): Result<T, F> {
		return isOk(res) ? ok(res.value) : f(res.error);
	}

	export function flatten<T, E>(res: Result<Result<T, E>, E>): Result<T, E> {
		return isOk(res) ? res.value : err(res.error);
	}

	export function transpose<T, E>(res: Result<Option<T>, E>): Option<Result<T, E>> {
		if (isErr(res)) return Option.some(err(res.error));
		if (Option.isNone(res.value)) return Option.none();
		return Option.some(ok(res.value.value));
	}

	export function match<T, E, U>(res: Result<T, E>, onOk: (value: T) => U, onErr: (error: E) => U): U {
		return isOk(res) ? onOk(res.value) : onErr(res.error);
	}
 }

 // ThisError

 export interface ThisError<K extends string = string> {
	readonly kind: K;
	readonly message: string;
	readonly source?: Error;
 }

 export namespace ThisError {
	export function create<K extends string>(
		kind: K,
		message: string,
		source?: Error,
	): ThisError<K> {
		return { kind, message, source };
	}

	export function define<K extends string>(kind: K) {
		return (message: string, source?: Error): ThisError<K> => ({
			kind,
			message,
			source,
		});
	}

	export function toError(e: ThisError): Error {
		const err = new Error(e.message);
		err.name = e.kind;
		if (e.source) err.cause = e.source;
		return err;
	}
 }

 */
