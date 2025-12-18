// Re-export Effect types for Gigaform
// NOTE: Gigaform requires returnTypes: "effect" in macroforge.config.ts
export {
  // Exit functions (deserialize return)
  exitSucceed,
  exitFail,
  exitIsSuccess,
  // Option functions (field controller state)
  optionSome,
  optionNone,
  optionIsNone,
} from "macroforge/reexports/effect";
export type { Exit, Option } from "macroforge/reexports/effect";

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
  /** Transform input value before setting (applies configured format like uppercase, trim, etc.) */
  transform(value: T): T;
  getError(): Option<Array<string>>;
  setError(value: Option<Array<string>>): void;
  getTainted(): Option<boolean>;
  setTainted(value: Option<boolean>): void;
  validate(): Array<string>;
}

/** Number field controller with numeric constraints */
export interface NumberFieldController extends FieldController<number | null> {
  readonly min?: number;
  readonly max?: number;
  readonly step?: number;
}

/** Select field controller with options */
export interface SelectFieldController<T = string> extends FieldController<T> {
  readonly options: ReadonlyArray<{ label: string; value: T }>;
}

/** Toggle/boolean field controller */
export interface ToggleFieldController extends FieldController<boolean> {
  readonly styleClasses?: string;
}

/** Checkbox field controller */
export interface CheckboxFieldController extends FieldController<boolean> {
  readonly styleClasses?: string;
}

/** Switch field controller */
export interface SwitchFieldController extends FieldController<boolean> {}

/** Text area field controller */
export interface TextAreaFieldController extends FieldController<
  string | null
> {}

/** Radio group option */
export interface RadioGroupOption {
  readonly label: string;
  readonly value: string;
  readonly icon?: unknown;
}

/** Radio group field controller */
export interface RadioGroupFieldController extends FieldController<string> {
  readonly options: ReadonlyArray<RadioGroupOption>;
  readonly orientation?: "horizontal" | "vertical";
}

/** Tags field controller (array of strings) */
export interface TagsFieldController extends FieldController<
  ReadonlyArray<string>
> {}

/** Combobox item type */
export interface ComboboxItem<T = unknown> {
  readonly label: string;
  readonly value: T;
}

/** Combobox field controller */
export interface ComboboxFieldController<
  T = string,
> extends FieldController<T | null> {
  readonly items: ReadonlyArray<ComboboxItem<T>>;
  readonly allowCustom?: boolean;
  readonly roundedClass?: string;
}

/** Combobox multiple field controller */
export interface ComboboxMultipleFieldController<
  T = string,
> extends FieldController<ReadonlyArray<T>> {
  readonly items: ReadonlyArray<ComboboxItem<T>>;
  readonly allowCustom?: boolean;
  readonly roundedClass?: string;
}

/** Date-time field controller */
export interface DateTimeFieldController extends FieldController<
  bigint | null
> {}

/** Hidden field controller */
export interface HiddenFieldController<
  T = unknown,
> extends FieldController<T> {}

/** Email field controller with subcontrollers */
export interface EmailFieldController extends FieldController<string | null> {
  /** Controller for the email string input */
  readonly emailController: FieldController<string | null>;
  /** Controller for the "can email" toggle */
  readonly canEmailController: FieldController<boolean>;
}

/** Phone field controller with subcontrollers */
export interface PhoneFieldController extends FieldController<unknown> {
  readonly phoneTypeController: ComboboxFieldController<string>;
  readonly numberController: FieldController<string | null>;
  readonly canCallController: ToggleFieldController;
  readonly canTextController: ToggleFieldController;
}

/** Enum/Variant field controller */
export interface EnumFieldController<
  TVariant extends string = string,
> extends FieldController<{ type: TVariant; [key: string]: unknown } | null> {
  readonly variants: Record<
    TVariant,
    { label: string; fields?: Record<string, unknown> }
  >;
  readonly defaultVariant?: TVariant;
  readonly legend?: string;
}

/** Base interface for array field controllers */
export interface ArrayFieldController<T> extends FieldController<readonly T[]> {
  at(index: number): FieldController<T>;
  push(value: T): void;
  remove(index: number): void;
  swap(a: number, b: number): void;
}

/** Item state for array fieldset items */
export interface ArrayFieldsetItem<T> {
  readonly _id: string;
  readonly isLeaving: boolean;
  readonly variant: "object" | "tuple";
  readonly val?: [PropertyKey, T];
}

/** Combobox hydration config for array fieldsets */
export interface ArrayFieldsetComboboxConfig<T = unknown> {
  readonly items: Array<{ label: string; value: T }>;
  readonly setItems?: (items: Array<{ label: string; value: T }>) => void;
  readonly itemLabelKeyName: string;
  readonly itemValueKeyName: string;
  readonly fetchConfigs: ReadonlyArray<{ url: string; schema?: unknown }>;
  readonly skipInitialFetch?: boolean;
}

/** Array fieldset controller with element controllers */
export interface ArrayFieldsetController<
  TItem,
  TElementControllers extends Record<string, FieldController<unknown>> = Record<
    string,
    FieldController<unknown>
  >,
> extends ArrayFieldController<TItem> {
  /** Template structure for new items */
  readonly itemStructure: TItem;
  /** Legend text for the fieldset */
  readonly legendText?: string;
  /** Radio group configuration for "main" item selection */
  readonly radioGroup?: {
    readonly mainFieldKey: string;
  };
  /** Whether to display items in card style */
  readonly card?: boolean;
  /** Whether items can be reordered via drag-and-drop */
  readonly reorderable?: boolean;
  /** Combobox fetch configurations for items */
  readonly comboboxFetchConfigs?: ReadonlyArray<ArrayFieldsetComboboxConfig>;
  /** Create element controllers for a specific item */
  elementControllers(context: {
    index: number;
    item: ArrayFieldsetItem<TItem>;
  }): TElementControllers;
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

/** Manual entry controllers for site address fields */
export interface SiteManualEntryControllers {
  readonly addressLine1: FieldController<string | null>;
  readonly addressLine2: FieldController<string | null>;
  readonly locality: FieldController<string | null>;
  readonly administrativeAreaLevel1: FieldController<string | null>;
  readonly postalCode: FieldController<string | null>;
  readonly country: FieldController<string | null>;
}

/** Filter configuration for duplicate site search */
export interface SiteDuplicateSearchFilters {
  readonly siteId?: string;
  readonly filters: ReadonlyArray<{
    field: string;
    op: string;
    value: unknown;
  }>;
}

/** Address lookup field controller for Google Places integration */
export interface AddressLookupFieldController<
  TSite = unknown,
> extends FieldController<TSite | null> {
  /** Label background class for floating label */
  readonly labelBgClass?: string;
}

/** Site fieldset controller with lookup and manual entry modes */
export interface SiteFieldsetController<
  TSite = unknown,
> extends FieldController<TSite | null> {
  /** Controller for Google Places address lookup */
  readonly lookupController: AddressLookupFieldController<TSite>;
  /** Controllers for manual address entry fields */
  readonly manualEntryControllers: SiteManualEntryControllers;
  /** Configuration for duplicate site search */
  readonly duplicateSearchFilters?: SiteDuplicateSearchFilters;
  /** Optional scrolling container getter for scroll position preservation */
  readonly scrollingContainer?: () => HTMLElement | null;
}
