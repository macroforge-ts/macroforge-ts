import type { Result, Option } from "macroforge/utils";

/** Base interface for field controllers */
export interface FieldController<T> {
  readonly path: ReadonlyArray<string | number>;
  readonly name: string;
  readonly constraints: Record<string, unknown>;
  readonly label?: string;
  readonly description?: string;
  readonly placeholder?: string;
  readonly disabled?: boolean;
  readonly readonly?: boolean;
  get(): T;
  set(value: T): void;
  getError(): Option<Array<string>>;
  setError(value: Option<Array<string>>): void;
  getTainted(): Option<boolean>;
  setTainted(value: Option<boolean>): void;
  validate(): Array<string>;
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
  validate(): Promise<
    Result<TData, ReadonlyArray<{ field: string; message: string }>>
  >;
  reset(overrides: Partial<TData> | null): void;
}

/** Gigaform with variant support (for unions/enums) */
export interface VariantGigaform<
  TData,
  TErrors,
  TTainted,
  TFields,
  TVariant extends string,
> extends BaseGigaform<TData, TErrors, TTainted, TFields> {
  readonly currentVariant: TVariant;
  switchVariant(variant: TVariant): void;
}
