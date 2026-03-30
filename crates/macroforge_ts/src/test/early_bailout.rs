use super::*;

// ==================== EARLY BAILOUT TESTS ====================

#[test]
fn test_early_bailout_no_derive_returns_unchanged() {
    // Code without @derive should be returned unchanged immediately
    use crate::expand_inner;

    let source = r#"
class User {
    name: string;
    constructor(name: string) {
        this.name = name;
    }
}
"#;

    let result = expand_inner(source, "test.ts", None).unwrap();

    // Code should be returned exactly as-is
    assert_eq!(
        result.code, source,
        "Code without @derive should be returned unchanged"
    );
    assert!(result.types.is_none(), "No type output expected");
    assert!(result.diagnostics.is_empty(), "No diagnostics expected");
    assert!(
        result.source_mapping.is_none(),
        "No source mapping expected"
    );
}

#[test]
fn test_early_bailout_svelte_runes_unchanged() {
    // Svelte runes ($state, $derived) without @derive should be returned unchanged
    use crate::expand_inner;

    let source = r#"
let count = $state(0);
let double = $derived(count * 2);

function increment() {
    count++;
}
"#;

    let result = expand_inner(source, "Counter.svelte.ts", None).unwrap();

    // Svelte runes code should be returned exactly as-is
    assert_eq!(
        result.code, source,
        "Svelte runes without @derive should be unchanged"
    );
    assert!(
        result.diagnostics.is_empty(),
        "No diagnostics for Svelte runes"
    );
}

#[test]
fn test_early_bailout_svelte_props_unchanged() {
    // Svelte $props() rune should be returned unchanged
    use crate::expand_inner;

    let source = r#"
interface Props {
    name: string;
    count?: number;
}

let { name, count = 0 }: Props = $props();
"#;

    let result = expand_inner(source, "Component.svelte.ts", None).unwrap();

    assert_eq!(
        result.code, source,
        "Svelte $props() without @derive should be unchanged"
    );
    assert!(
        result.diagnostics.is_empty(),
        "No diagnostics for Svelte $props"
    );
}

#[test]
fn test_early_bailout_complex_svelte_component_unchanged() {
    // Complex Svelte component code without @derive should be returned unchanged
    use crate::expand_inner;

    let source = r#"
interface Props {
    items: string[];
    selected?: string;
}

let { items, selected = '' }: Props = $props();

let filteredItems = $derived(
    items.filter(item => item.includes(selected))
);

let count = $state(0);
let message = $derived.by(() => {
    if (count === 0) return 'No items';
    return `${count} items`;
});

function handleClick() {
    count++;
}

$effect(() => {
    console.log('Count changed:', count);
});
"#;

    let result = expand_inner(source, "List.svelte.ts", None).unwrap();

    assert_eq!(
        result.code, source,
        "Complex Svelte code without @derive should be unchanged"
    );
    assert!(result.diagnostics.is_empty(), "No diagnostics expected");
}

#[test]
fn test_with_derive_processes_normally() {
    // Code WITH @derive should be processed normally
    use crate::expand_inner;

    let source = r#"
/** @derive(Debug) */
class User {
    name: string;
}
"#;

    let result = expand_inner(source, "test.ts", None).unwrap();

    // Should have processed the macro
    assert!(
        result.code.contains("toString"),
        "Debug macro should generate toString"
    );
    assert_ne!(result.code, source, "Code with @derive should be modified");
}

#[test]
fn test_derive_in_string_literal_still_skipped() {
    // @derive inside a string literal should still trigger processing
    // (we do a simple contains check, not parsing)
    use crate::expand_inner;

    let source = r#"
const msg = "Use @derive to add methods";
class User {
    name: string;
}
"#;

    let result = expand_inner(source, "test.ts", None).unwrap();

    // The contains("@derive") check will find the string literal
    // This is a conservative approach - we process the file but find no actual decorators
    // The result should have no errors and the code may have minor changes from parsing
    assert!(
        result.diagnostics.iter().all(|d| d.level != "error"),
        "No errors expected even with @derive in string literal"
    );
}

#[test]
fn test_early_bailout_empty_file() {
    // Empty file should be returned unchanged
    use crate::expand_inner;

    let source = "";
    let result = expand_inner(source, "empty.ts", None).unwrap();

    assert_eq!(
        result.code, source,
        "Empty file should be returned unchanged"
    );
    assert!(
        result.diagnostics.is_empty(),
        "No diagnostics for empty file"
    );
}

#[test]
fn test_early_bailout_only_comments() {
    // File with only comments should be returned unchanged
    use crate::expand_inner;

    let source = r#"
// This is a comment
/* Another comment */
/**
 * JSDoc comment without derive
 */
"#;

    let result = expand_inner(source, "comments.ts", None).unwrap();

    assert_eq!(
        result.code, source,
        "Comments-only file should be returned unchanged"
    );
    assert!(
        result.diagnostics.is_empty(),
        "No diagnostics for comments-only file"
    );
}

#[test]
fn test_early_bailout_regular_typescript() {
    // Regular TypeScript without macros should be returned unchanged
    use crate::expand_inner;

    let source = r#"
interface User {
    id: string;
    name: string;
    email: string;
}

type Role = 'admin' | 'user' | 'guest';

enum Status {
    Active,
    Inactive,
    Pending
}

function createUser(name: string): User {
    return {
        id: crypto.randomUUID(),
        name,
        email: `${name}@example.com`
    };
}

const users: Map<string, User> = new Map();

export { User, Role, Status, createUser, users };
"#;

    let result = expand_inner(source, "types.ts", None).unwrap();

    assert_eq!(
        result.code, source,
        "Regular TypeScript should be returned unchanged"
    );
    assert!(
        result.diagnostics.is_empty(),
        "No diagnostics for regular TypeScript"
    );
}

#[test]
fn test_no_false_positive_derive_in_prose_jsdoc() {
    // Regression test: `@derive` mentioned in prose doc comments (not as a directive)
    // should NOT trigger macro expansion. This file has zero actual macro annotations.
    let source = r#"
import type { Option } from "effect";
import { Exit } from "effect";
import type { Utc } from "effect/DateTime";

/** Deserialize result format from @derive(Deserialize) */
export type DeserializeResult<T> =
  | { success: true; value: T }
  | { success: false; errors: Array<{ field: string; message: string }> };

/** Converts a deserialize result to an Effect Exit */
export function toExit<T>(
  result: DeserializeResult<T>,
): Exit.Exit<T, Array<{ field: string; message: string }>> {
  if (result.success) {
    return Exit.succeed(result.value);
  } else {
    return Exit.fail(result.errors);
  }
}

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
  getError(): Option.Option<Array<string>>;
  setError(value: Option.Option<Array<string>>): void;
  getTainted(): Option.Option<boolean>;
  setTainted(value: Option.Option<boolean>): void;
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
export type SwitchFieldController = FieldController<boolean>;

/** Text area field controller */
export type TextAreaFieldController = FieldController<string | null>;

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
export type TagsFieldController = FieldController<ReadonlyArray<string>>;

/** Combobox item type */
export interface ComboboxItem<T = unknown> {
  readonly label: string;
  readonly value: T;
}

/** Configuration for combobox fields that store graph edge objects */
export interface EdgeConfig {
  /** Field path within the edge object that references the entity (e.g. "in") */
  readonly entityField: string;
}

/** Combobox field controller */
export interface ComboboxFieldController<
  T = string,
> extends FieldController<T | null> {
  readonly items: ReadonlyArray<ComboboxItem<T>>;
  readonly allowCustom?: boolean;
  readonly roundedClass?: string;
  /** URLs to fetch items from (populated by @comboboxController({ fetchUrls: [...] })) */
  readonly fetchUrls?: ReadonlyArray<string>;
  /** Key path to extract the display label from fetched items (default: "name") */
  readonly itemLabelKeyName?: string;
  /** Key path to extract the value from fetched items (default: "id") */
  readonly itemValueKeyName?: string;
  /** Edge configuration for fields that store graph edges instead of direct entities */
  readonly edgeConfig?: EdgeConfig;
}

/** Combobox multiple field controller */
export interface ComboboxMultipleFieldController<
  T = string,
> extends FieldController<ReadonlyArray<T>> {
  readonly items: ReadonlyArray<ComboboxItem<T>>;
  readonly allowCustom?: boolean;
  readonly roundedClass?: string;
  /** URLs to fetch items from (populated by @comboboxController({ fetchUrls: [...] })) */
  readonly fetchUrls?: ReadonlyArray<string>;
  /** Key path to extract the display label from fetched items (default: "name") */
  readonly itemLabelKeyName?: string;
  /** Key path to extract the value from fetched items (default: "id") */
  readonly itemValueKeyName?: string;
  /** Edge configuration for fields that store graph edges instead of direct entities */
  readonly edgeConfig?: EdgeConfig;
}

/** Duration field controller - value is a [seconds, nanos] tuple */
export type DurationFieldController = FieldController<[number, number] | null>;

/** Date-time field controller */
export type DateTimeFieldController = FieldController<Utc | null>;

/** Date-only field controller (no time component) */
export type DateFieldController = FieldController<Utc | null>;

/** Multi-date picker field controller */
export type DatePickerMultipleFieldController = FieldController<Array<Utc> | null>;

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
  TVariantControllers = { [K in TVariant]?: Record<string, FieldController<unknown>> },
> extends FieldController<{ type: TVariant; [key: string]: unknown } | null> {
  readonly variants: Record<
    TVariant,
    { label: string; fields?: Record<string, unknown> }
  >;
  readonly defaultVariant?: TVariant;
  /** Derived variant detected from the current value (tag field or shape matching). */
  readonly currentVariant: TVariant;
  readonly legend?: string;
  readonly selectLabel?: string;
  readonly variantControllers?: TVariantControllers;
}

/** Base interface for array field controllers */
export interface ArrayFieldController<T> extends FieldController<ReadonlyArray<T>> {
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
export interface BaseGigaform<TData> {
  data: TData;
  validate(): Exit.Exit<TData, Array<{ field: string; message: string }>>;
  asyncValidate(): Promise<
    Exit.Exit<TData, ReadonlyArray<{ field: string; message: string }>>
  >;
  reset(overrides: Partial<TData> | null): void;
}

/** Gigaform with variant support (for unions/enums) */
export interface VariantGigaform<
  TData,
  TVariant extends string,
> extends BaseGigaform<TData> {
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

/** Wrapper for nullable nested struct field controllers */
export interface NullableControllers<_T> {
  isNull(): boolean;
  initialize(): void;
  clear(): void;
}
"#;

    // The quick-check used by expand_inner/expand_for_cache must reject this file
    // so it never reaches the expansion engine at all.
    assert!(
        !crate::has_macro_annotations(source),
        "has_macro_annotations should return false for a file with @derive only in prose JSDoc"
    );

    GLOBALS.set(&Default::default(), || {
        let program = parse_module(source);
        let host = MacroExpander::new().unwrap();
        let result = host.expand(source, &program, "types.ts").unwrap();

        // File should NOT be changed — no actual macro annotations
        assert!(
            !result.changed,
            "File with @derive in prose JSDoc should NOT trigger expansion. Got changed=true with code:\n{}",
            result.code
        );

        // Should have zero error diagnostics
        let errors: Vec<_> = result
            .diagnostics
            .iter()
            .filter(|d| d.level == DiagnosticLevel::Error)
            .collect();
        assert!(
            errors.is_empty(),
            "Should have no errors for a file with no macros, got: {:?}",
            errors
        );
    });
}
