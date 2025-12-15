import { SerializeContext } from 'macroforge/serde';
import { __serializeColumnConfig } from './column-config.svelte';
import { __serializeOverviewDisplay } from './overview-display.svelte';
import { __serializeRowHeight } from './row-height.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeOverviewDisplay } from './overview-display.svelte';
import { __deserializeRowHeight } from './row-height.svelte';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { ColumnConfig } from './column-config.svelte';
import type { OverviewDisplay } from './overview-display.svelte';
import type { RowHeight } from './row-height.svelte';
import type { Table } from './table.svelte';

export interface OverviewSettings {
    rowHeight: RowHeight;

    cardOrRow: OverviewDisplay;
    perPage: number;
    columnConfigs: ColumnConfig[];
}

export function defaultValueOverviewSettings(): OverviewSettings {
    return {
        rowHeight: 'Medium',
        cardOrRow: 'Table',
        perPage: 0,
        columnConfigs: []
    } as OverviewSettings;
}

export function toStringifiedJSONOverviewSettings(value: OverviewSettings): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeOverviewSettings(value, ctx));
}
export function toObjectOverviewSettings(value: OverviewSettings): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeOverviewSettings(value, ctx);
}
export function __serializeOverviewSettings(
    value: OverviewSettings,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'OverviewSettings', __id };
    result['rowHeight'] = __serializeRowHeight(value.rowHeight, ctx);
    result['cardOrRow'] = __serializeOverviewDisplay(value.cardOrRow, ctx);
    result['perPage'] = value.perPage;
    result['columnConfigs'] = value.columnConfigs.map((item) => __serializeColumnConfig(item, ctx));
    return result;
}

export function fromStringifiedJSONOverviewSettings(
    json: string,
    opts?: DeserializeOptions
): Result<OverviewSettings, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectOverviewSettings(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectOverviewSettings(
    obj: unknown,
    opts?: DeserializeOptions
): Result<OverviewSettings, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeOverviewSettings(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'OverviewSettings.fromObject: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function __deserializeOverviewSettings(
    value: any,
    ctx: DeserializeContext
): OverviewSettings | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'OverviewSettings.__deserialize: expected an object' }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('rowHeight' in obj)) {
        errors.push({ field: 'rowHeight', message: 'missing required field' });
    }
    if (!('cardOrRow' in obj)) {
        errors.push({ field: 'cardOrRow', message: 'missing required field' });
    }
    if (!('perPage' in obj)) {
        errors.push({ field: 'perPage', message: 'missing required field' });
    }
    if (!('columnConfigs' in obj)) {
        errors.push({ field: 'columnConfigs', message: 'missing required field' });
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_rowHeight = obj['rowHeight'] as RowHeight;
        {
            const __result = __deserializeRowHeight(__raw_rowHeight, ctx);
            ctx.assignOrDefer(instance, 'rowHeight', __result);
        }
    }
    {
        const __raw_cardOrRow = obj['cardOrRow'] as OverviewDisplay;
        {
            const __result = __deserializeOverviewDisplay(__raw_cardOrRow, ctx);
            ctx.assignOrDefer(instance, 'cardOrRow', __result);
        }
    }
    {
        const __raw_perPage = obj['perPage'] as number;
        instance.perPage = __raw_perPage;
    }
    {
        const __raw_columnConfigs = obj['columnConfigs'] as ColumnConfig[];
        if (Array.isArray(__raw_columnConfigs)) {
            instance.columnConfigs = __raw_columnConfigs as ColumnConfig[];
        }
    }
    if (errors.length > 0) {
        throw new DeserializeError(errors);
    }
    return instance as OverviewSettings;
}
export function validateFieldOverviewSettings<K extends keyof OverviewSettings>(
    field: K,
    value: OverviewSettings[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsOverviewSettings(
    partial: Partial<OverviewSettings>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeOverviewSettings(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'rowHeight' in o && 'cardOrRow' in o && 'perPage' in o && 'columnConfigs' in o;
}
export function isOverviewSettings(obj: unknown): obj is OverviewSettings {
    if (!hasShapeOverviewSettings(obj)) {
        return false;
    }
    const result = fromObjectOverviewSettings(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsOverviewSettings = {
    _errors: Option<Array<string>>;
    rowHeight: Option<Array<string>>;
    cardOrRow: Option<Array<string>>;
    perPage: Option<Array<string>>;
    columnConfigs: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedOverviewSettings = {
    rowHeight: Option<boolean>;
    cardOrRow: Option<boolean>;
    perPage: Option<boolean>;
    columnConfigs: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersOverviewSettings {
    readonly rowHeight: FieldController<RowHeight>;
    readonly cardOrRow: FieldController<OverviewDisplay>;
    readonly perPage: FieldController<number>;
    readonly columnConfigs: ArrayFieldController<ColumnConfig>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformOverviewSettings {
    readonly data: OverviewSettings;
    readonly errors: ErrorsOverviewSettings;
    readonly tainted: TaintedOverviewSettings;
    readonly fields: FieldControllersOverviewSettings;
    validate(): Result<OverviewSettings, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<OverviewSettings>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormOverviewSettings(
    overrides?: Partial<OverviewSettings>
): GigaformOverviewSettings {
    let data = $state({ ...defaultValueOverviewSettings(), ...overrides });
    let errors = $state<ErrorsOverviewSettings>({
        _errors: Option.none(),
        rowHeight: Option.none(),
        cardOrRow: Option.none(),
        perPage: Option.none(),
        columnConfigs: Option.none()
    });
    let tainted = $state<TaintedOverviewSettings>({
        rowHeight: Option.none(),
        cardOrRow: Option.none(),
        perPage: Option.none(),
        columnConfigs: Option.none()
    });
    const fields: FieldControllersOverviewSettings = {
        rowHeight: {
            path: ['rowHeight'] as const,
            name: 'rowHeight',
            constraints: { required: true },

            get: () => data.rowHeight,
            set: (value: RowHeight) => {
                data.rowHeight = value;
            },
            transform: (value: RowHeight): RowHeight => value,
            getError: () => errors.rowHeight,
            setError: (value: Option<Array<string>>) => {
                errors.rowHeight = value;
            },
            getTainted: () => tainted.rowHeight,
            setTainted: (value: Option<boolean>) => {
                tainted.rowHeight = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOverviewSettings('rowHeight', data.rowHeight);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        cardOrRow: {
            path: ['cardOrRow'] as const,
            name: 'cardOrRow',
            constraints: { required: true },

            get: () => data.cardOrRow,
            set: (value: OverviewDisplay) => {
                data.cardOrRow = value;
            },
            transform: (value: OverviewDisplay): OverviewDisplay => value,
            getError: () => errors.cardOrRow,
            setError: (value: Option<Array<string>>) => {
                errors.cardOrRow = value;
            },
            getTainted: () => tainted.cardOrRow,
            setTainted: (value: Option<boolean>) => {
                tainted.cardOrRow = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOverviewSettings('cardOrRow', data.cardOrRow);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        perPage: {
            path: ['perPage'] as const,
            name: 'perPage',
            constraints: { required: true },

            get: () => data.perPage,
            set: (value: number) => {
                data.perPage = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.perPage,
            setError: (value: Option<Array<string>>) => {
                errors.perPage = value;
            },
            getTainted: () => tainted.perPage,
            setTainted: (value: Option<boolean>) => {
                tainted.perPage = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOverviewSettings('perPage', data.perPage);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        columnConfigs: {
            path: ['columnConfigs'] as const,
            name: 'columnConfigs',
            constraints: { required: true },

            get: () => data.columnConfigs,
            set: (value: ColumnConfig[]) => {
                data.columnConfigs = value;
            },
            transform: (value: ColumnConfig[]): ColumnConfig[] => value,
            getError: () => errors.columnConfigs,
            setError: (value: Option<Array<string>>) => {
                errors.columnConfigs = value;
            },
            getTainted: () => tainted.columnConfigs,
            setTainted: (value: Option<boolean>) => {
                tainted.columnConfigs = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldOverviewSettings(
                    'columnConfigs',
                    data.columnConfigs
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['columnConfigs', index] as const,
                name: `columnConfigs.${index}`,
                constraints: { required: true },
                get: () => data.columnConfigs[index]!,
                set: (value: ColumnConfig) => {
                    data.columnConfigs[index] = value;
                },
                transform: (value: ColumnConfig): ColumnConfig => value,
                getError: () => errors.columnConfigs,
                setError: (value: Option<Array<string>>) => {
                    errors.columnConfigs = value;
                },
                getTainted: () => tainted.columnConfigs,
                setTainted: (value: Option<boolean>) => {
                    tainted.columnConfigs = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: ColumnConfig) => {
                data.columnConfigs.push(item);
            },
            remove: (index: number) => {
                data.columnConfigs.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.columnConfigs[a]!;
                data.columnConfigs[a] = data.columnConfigs[b]!;
                data.columnConfigs[b] = tmp;
            }
        }
    };
    function validate(): Result<OverviewSettings, Array<{ field: string; message: string }>> {
        return fromObjectOverviewSettings(data);
    }
    function reset(newOverrides?: Partial<OverviewSettings>): void {
        data = { ...defaultValueOverviewSettings(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            rowHeight: Option.none(),
            cardOrRow: Option.none(),
            perPage: Option.none(),
            columnConfigs: Option.none()
        };
        tainted = {
            rowHeight: Option.none(),
            cardOrRow: Option.none(),
            perPage: Option.none(),
            columnConfigs: Option.none()
        };
    }
    return {
        get data() {
            return data;
        },
        set data(v) {
            data = v;
        },
        get errors() {
            return errors;
        },
        set errors(v) {
            errors = v;
        },
        get tainted() {
            return tainted;
        },
        set tainted(v) {
            tainted = v;
        },
        fields,
        validate,
        reset
    };
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to fromStringifiedJSON() from @derive(Deserialize). */
export function fromFormDataOverviewSettings(
    formData: FormData
): Result<OverviewSettings, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        // Collect nested object fields with prefix "rowHeight."
        const rowHeightObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('rowHeight.')) {
                const fieldName = key.slice('rowHeight.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = rowHeightObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.rowHeight = rowHeightObj;
    }
    {
        // Collect nested object fields with prefix "cardOrRow."
        const cardOrRowObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('cardOrRow.')) {
                const fieldName = key.slice('cardOrRow.'.length);
                // Handle deeper nesting by splitting on dots
                const parts = fieldName.split('.');
                let current = cardOrRowObj;
                for (let i = 0; i < parts.length - 1; i++) {
                    const part = parts[i]!;
                    if (!(part in current)) {
                        current[part] = {};
                    }
                    current = current[part] as Record<string, unknown>;
                }
                current[parts[parts.length - 1]!] = value;
            }
        }
        obj.cardOrRow = cardOrRowObj;
    }
    {
        const perPageStr = formData.get('perPage');
        obj.perPage = perPageStr ? parseFloat(perPageStr as string) : 0;
        if (obj.perPage !== undefined && isNaN(obj.perPage as number)) obj.perPage = 0;
    }
    {
        // Collect array items from indexed form fields
        const columnConfigsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('columnConfigs.' + idx + '.') || idx === 0) {
            // Check if any field with this index exists
            const hasAny = Array.from(formData.keys()).some((k) =>
                k.startsWith('columnConfigs.' + idx + '.')
            );
            if (!hasAny && idx > 0) break;
            if (hasAny) {
                const item: Record<string, unknown> = {};
                for (const [key, value] of Array.from(formData.entries())) {
                    if (key.startsWith('columnConfigs.' + idx + '.')) {
                        const fieldName = key.slice(
                            'columnConfigs.'.length + String(idx).length + 1
                        );
                        item[fieldName] = value;
                    }
                }
                columnConfigsItems.push(item);
            }
            idx++;
            if (idx > 1000) break; // Safety limit
        }
        obj.columnConfigs = columnConfigsItems;
    }
    return fromStringifiedJSONOverviewSettings(JSON.stringify(obj));
}
