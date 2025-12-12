import type { Result } from "macroforge/result";

/** Base interface for field controllers */
export interface FieldController<T> {
  readonly path: readonly string[];
  readonly name: string;
  readonly constraints: Readonly<Record<string, unknown>>;
  readonly label: string | null;
  readonly description: string | null;
  readonly placeholder: string | null;
  readonly disabled: boolean | null;
  readonly readonly: boolean | null;
  get(): T;
  set(value: T): void;
  getError(): readonly string[] | null;
  setError(errors: readonly string[] | null): void;
  getTainted(): boolean;
  setTainted(tainted: boolean): void;
  validate(): Promise<boolean>;
}

/** Base interface for array field controllers */
export interface ArrayFieldController<T> extends FieldController<readonly T[]> {
  at(index: number): FieldController<T>;
  push(value: T): void;
  remove(index: number): void;
  swap(a: number, b: number): void;
}

/** Base Gigaform interface - generated forms extend this */
export interface BaseGigaform<TData, TErrors, TTainted, TFields> {
  readonly data: TData;
  readonly errors: TErrors;
  readonly tainted: TTainted;
  readonly fields: TFields;
  validate(): Promise<Result<TData, ReadonlyArray<{ field: string; message: string }>>>;
  reset(overrides: Partial<TData> | null): void;
}

/** Gigaform with variant support (for unions/enums) */
export interface VariantGigaform<TData, TErrors, TTainted, TFields, TVariant extends string>
  extends BaseGigaform<TData, TErrors, TTainted, TFields> {
  readonly currentVariant: TVariant;
  switchVariant(variant: TVariant): void;
}
