import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { columnConfigSerializeWithContext } from './column-config.svelte';
import { overviewDisplaySerializeWithContext } from './overview-display.svelte';
import { rowHeightSerializeWithContext } from './row-height.svelte';
import { exitSucceed as __mf_exitSucceed } from 'macroforge/reexports/effect';
import { exitFail as __mf_exitFail } from 'macroforge/reexports/effect';
import { exitIsSuccess as __mf_exitIsSuccess } from 'macroforge/reexports/effect';
import type { Exit as __mf_Exit } from 'macroforge/reexports/effect';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { overviewDisplayDeserializeWithContext } from './overview-display.svelte';
import { rowHeightDeserializeWithContext } from './row-height.svelte';
import { Result } from 'macroforge/reexports';
import { Option } from 'macroforge/reexports';
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

export function overviewSettingsDefaultValue(): OverviewSettings {
    return {
        rowHeight: 'Medium',
        cardOrRow: 'Table',
        perPage: 0,
        columnConfigs: []
    } as OverviewSettings;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function overviewSettingsSerialize(
    value: OverviewSettings
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(overviewSettingsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function overviewSettingsSerializeWithContext(
    value: OverviewSettings,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'OverviewSettings', __id };
    result['rowHeight'] = rowHeightSerializeWithContext(value.rowHeight, ctx);
    result['cardOrRow'] = overviewDisplaySerializeWithContext(value.cardOrRow, ctx);
    result['perPage'] = value.perPage;
    result['columnConfigs'] = value.columnConfigs.map((item) =>
        columnConfigSerializeWithContext(item, ctx)
    );
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function overviewSettingsDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
): __mf_Exit<Array<{ field: string; message: string }>, OverviewSettings> {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = overviewSettingsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return __mf_exitFail([
                {
                    field: '_root',
                    message: 'OverviewSettings.deserialize: root cannot be a forward reference'
                }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return __mf_exitSucceed(resultOrRef);
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return __mf_exitFail(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return __mf_exitFail([{ field: '_root', message }]);
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function overviewSettingsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): OverviewSettings | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'OverviewSettings.deserializeWithContext: expected an object'
            }
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
        throw new __mf_DeserializeError(errors);
    }
    const instance: any = {};
    if (obj.__id !== undefined) {
        ctx.register(obj.__id as number, instance);
    }
    ctx.trackForFreeze(instance);
    {
        const __raw_rowHeight = obj['rowHeight'] as RowHeight;
        {
            const __result = rowHeightDeserializeWithContext(__raw_rowHeight, ctx);
            ctx.assignOrDefer(instance, 'rowHeight', __result);
        }
    }
    {
        const __raw_cardOrRow = obj['cardOrRow'] as OverviewDisplay;
        {
            const __result = overviewDisplayDeserializeWithContext(__raw_cardOrRow, ctx);
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
        throw new __mf_DeserializeError(errors);
    }
    return instance as OverviewSettings;
}
export function overviewSettingsValidateField<K extends keyof OverviewSettings>(
    field: K,
    value: OverviewSettings[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function overviewSettingsValidateFields(
    partial: Partial<OverviewSettings>
): Array<{ field: string; message: string }> {
    return [];
}
export function overviewSettingsHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'rowHeight' in o && 'cardOrRow' in o && 'perPage' in o && 'columnConfigs' in o;
}
export function overviewSettingsIs(obj: unknown): obj is OverviewSettings {
    if (!overviewSettingsHasShape(obj)) {
        return false;
    }
    const result = overviewSettingsDeserialize(obj);
    return __mf_exitIsSuccess(result);
}

/** Nested error structure matching the data shape */ export type OverviewSettingsErrors = {
    _errors: Option<Array<string>>;
    rowHeight: Option<Array<string>>;
    cardOrRow: Option<Array<string>>;
    perPage: Option<Array<string>>;
    columnConfigs: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type OverviewSettingsTainted = {
    rowHeight: Option<boolean>;
    cardOrRow: Option<boolean>;
    perPage: Option<boolean>;
    columnConfigs: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface OverviewSettingsFieldControllers {
    readonly rowHeight: FieldController<RowHeight>;
    readonly cardOrRow: FieldController<OverviewDisplay>;
    readonly perPage: FieldController<number>;
    readonly columnConfigs: ArrayFieldController<ColumnConfig>;
} /** Gigaform instance containing reactive state and field controllers */
export interface OverviewSettingsGigaform {
    readonly data: OverviewSettings;
    readonly errors: OverviewSettingsErrors;
    readonly tainted: OverviewSettingsTainted;
    readonly fields: OverviewSettingsFieldControllers;
    validate(): Result<OverviewSettings, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<OverviewSettings>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function overviewSettingsCreateForm(
    overrides?: Partial<OverviewSettings>
): OverviewSettingsGigaform {
    let data = $state({ ...overviewSettingsDefaultValue(), ...overrides });
    let errors = $state<OverviewSettingsErrors>({
        _errors: Option.none(),
        rowHeight: Option.none(),
        cardOrRow: Option.none(),
        perPage: Option.none(),
        columnConfigs: Option.none()
    });
    let tainted = $state<OverviewSettingsTainted>({
        rowHeight: Option.none(),
        cardOrRow: Option.none(),
        perPage: Option.none(),
        columnConfigs: Option.none()
    });
    const fields: OverviewSettingsFieldControllers = {
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
                const fieldErrors = overviewSettingsValidateField('rowHeight', data.rowHeight);
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
                const fieldErrors = overviewSettingsValidateField('cardOrRow', data.cardOrRow);
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
                const fieldErrors = overviewSettingsValidateField('perPage', data.perPage);
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
                const fieldErrors = overviewSettingsValidateField(
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
        return overviewSettingsDeserialize(data);
    }
    function reset(newOverrides?: Partial<OverviewSettings>): void {
        data = { ...overviewSettingsDefaultValue(), ...newOverrides };
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
} /** Parses FormData and validates it, returning a Result with the parsed data or errors. Delegates validation to deserialize() from @derive(Deserialize). */
export function overviewSettingsFromFormData(
    formData: FormData
): Result<OverviewSettings, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const rowHeightObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('rowHeight.')) {
                const fieldName = key.slice('rowHeight.'.length);
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
        const cardOrRowObj: Record<string, unknown> = {};
        for (const [key, value] of Array.from(formData.entries())) {
            if (key.startsWith('cardOrRow.')) {
                const fieldName = key.slice('cardOrRow.'.length);
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
        const columnConfigsItems: Array<Record<string, unknown>> = [];
        let idx = 0;
        while (formData.has('columnConfigs.' + idx + '.') || idx === 0) {
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
            if (idx > 1000) break;
        }
        obj.columnConfigs = columnConfigsItems;
    }
    return overviewSettingsDeserialize(obj);
}

export const OverviewSettings = {
    defaultValue: overviewSettingsDefaultValue,
    serialize: overviewSettingsSerialize,
    serializeWithContext: overviewSettingsSerializeWithContext,
    deserialize: overviewSettingsDeserialize,
    deserializeWithContext: overviewSettingsDeserializeWithContext,
    validateFields: overviewSettingsValidateFields,
    hasShape: overviewSettingsHasShape,
    is: overviewSettingsIs,
    createForm: overviewSettingsCreateForm,
    fromFormData: overviewSettingsFromFormData
} as const;
