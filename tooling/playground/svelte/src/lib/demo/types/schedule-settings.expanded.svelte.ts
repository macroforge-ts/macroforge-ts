import { SerializeContext as __mf_SerializeContext } from 'macroforge/serde';
import { rowHeightSerializeWithContext } from './row-height.svelte';
import { DeserializeContext as __mf_DeserializeContext } from 'macroforge/serde';
import { DeserializeError as __mf_DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions as __mf_DeserializeOptions } from 'macroforge/serde';
import { PendingRef as __mf_PendingRef } from 'macroforge/serde';
import { rowHeightDeserializeWithContext } from './row-height.svelte';
import type { Exit } from '@playground/macro/gigaform';
import { toExit } from '@playground/macro/gigaform';
import type { Option as __gf_Option } from '@playground/macro/gigaform';
import { optionNone } from '@playground/macro/gigaform';
import type { FieldController } from '@playground/macro/gigaform';
import type { ArrayFieldController } from '@playground/macro/gigaform';
/** import macro {Gigaform} from "@playground/macro"; */

import type { RowHeight } from './row-height.svelte';

export interface ScheduleSettings {
    daysPerWeek: number;

    rowHeight: RowHeight;
    visibleRoutes: string[];
    detailedCards: boolean;
}

export function scheduleSettingsDefaultValue(): ScheduleSettings {
    return {
        daysPerWeek: 0,
        rowHeight: 'Medium',
        visibleRoutes: [],
        detailedCards: false
    } as ScheduleSettings;
}

/** Serializes a value to a JSON string.
@param value - The value to serialize
@returns JSON string representation with cycle detection metadata */ export function scheduleSettingsSerialize(
    value: ScheduleSettings
): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(scheduleSettingsSerializeWithContext(value, ctx));
} /** Serializes with an existing context for nested/cyclic object graphs.
@param value - The value to serialize
@param ctx - The serialization context */
export function scheduleSettingsSerializeWithContext(
    value: ScheduleSettings,
    ctx: __mf_SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'ScheduleSettings', __id };
    result['daysPerWeek'] = value.daysPerWeek;
    result['rowHeight'] = rowHeightSerializeWithContext(value.rowHeight, ctx);
    result['visibleRoutes'] = value.visibleRoutes;
    result['detailedCards'] = value.detailedCards;
    return result;
}

/** Deserializes input to this interface type.
Automatically detects whether input is a JSON string or object.
@param input - JSON string or object to deserialize
@param opts - Optional deserialization options
@returns Result containing the deserialized value or validation errors */ export function scheduleSettingsDeserialize(
    input: unknown,
    opts?: __mf_DeserializeOptions
):
    | { success: true; value: ScheduleSettings }
    | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === 'string' ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = scheduleSettingsDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: '_root',
                        message: 'ScheduleSettings.deserialize: root cannot be a forward reference'
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return { success: true, value: resultOrRef };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return { success: false, errors: e.errors };
        }
        const message = e instanceof Error ? e.message : String(e);
        return { success: false, errors: [{ field: '_root', message }] };
    }
} /** Deserializes with an existing context for nested/cyclic object graphs.
@param value - The raw value to deserialize
@param ctx - The deserialization context */
export function scheduleSettingsDeserializeWithContext(
    value: any,
    ctx: __mf_DeserializeContext
): ScheduleSettings | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new __mf_DeserializeError([
            {
                field: '_root',
                message: 'ScheduleSettings.deserializeWithContext: expected an object'
            }
        ]);
    }
    const obj = value as Record<string, unknown>;
    const errors: Array<{ field: string; message: string }> = [];
    if (!('daysPerWeek' in obj)) {
        errors.push({ field: 'daysPerWeek', message: 'missing required field' });
    }
    if (!('rowHeight' in obj)) {
        errors.push({ field: 'rowHeight', message: 'missing required field' });
    }
    if (!('visibleRoutes' in obj)) {
        errors.push({ field: 'visibleRoutes', message: 'missing required field' });
    }
    if (!('detailedCards' in obj)) {
        errors.push({ field: 'detailedCards', message: 'missing required field' });
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
        const __raw_daysPerWeek = obj['daysPerWeek'] as number;
        instance.daysPerWeek = __raw_daysPerWeek;
    }
    {
        const __raw_rowHeight = obj['rowHeight'] as RowHeight;
        {
            const __result = rowHeightDeserializeWithContext(__raw_rowHeight, ctx);
            ctx.assignOrDefer(instance, 'rowHeight', __result);
        }
    }
    {
        const __raw_visibleRoutes = obj['visibleRoutes'] as string[];
        if (Array.isArray(__raw_visibleRoutes)) {
            instance.visibleRoutes = __raw_visibleRoutes as string[];
        }
    }
    {
        const __raw_detailedCards = obj['detailedCards'] as boolean;
        instance.detailedCards = __raw_detailedCards;
    }
    if (errors.length > 0) {
        throw new __mf_DeserializeError(errors);
    }
    return instance as ScheduleSettings;
}
export function scheduleSettingsValidateField<K extends keyof ScheduleSettings>(
    _field: K,
    _value: ScheduleSettings[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function scheduleSettingsValidateFields(
    _partial: Partial<ScheduleSettings>
): Array<{ field: string; message: string }> {
    return [];
}
export function scheduleSettingsHasShape(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'daysPerWeek' in o && 'rowHeight' in o && 'visibleRoutes' in o && 'detailedCards' in o;
}
export function scheduleSettingsIs(obj: unknown): obj is ScheduleSettings {
    if (!scheduleSettingsHasShape(obj)) {
        return false;
    }
    const result = scheduleSettingsDeserialize(obj);
    return result.success;
}

/** Nested error structure matching the data shape */ export type ScheduleSettingsErrors = {
    _errors: __gf_Option<Array<string>>;
    daysPerWeek: __gf_Option<Array<string>>;
    rowHeight: __gf_Option<Array<string>>;
    visibleRoutes: __gf_Option<Array<string>>;
    detailedCards: __gf_Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type ScheduleSettingsTainted = {
    daysPerWeek: __gf_Option<boolean>;
    rowHeight: __gf_Option<boolean>;
    visibleRoutes: __gf_Option<boolean>;
    detailedCards: __gf_Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface ScheduleSettingsFieldControllers {
    readonly daysPerWeek: FieldController<number>;
    readonly rowHeight: FieldController<RowHeight>;
    readonly visibleRoutes: ArrayFieldController<string>;
    readonly detailedCards: FieldController<boolean>;
} /** Gigaform instance containing reactive state and field controllers */
export interface ScheduleSettingsGigaform {
    readonly data: ScheduleSettings;
    readonly errors: ScheduleSettingsErrors;
    readonly tainted: ScheduleSettingsTainted;
    readonly fields: ScheduleSettingsFieldControllers;
    validate(): Exit<ScheduleSettings, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ScheduleSettings>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function scheduleSettingsCreateForm(
    overrides?: Partial<ScheduleSettings>
): ScheduleSettingsGigaform {
    let data = $state({ ...scheduleSettingsDefaultValue(), ...overrides });
    let errors = $state<ScheduleSettingsErrors>({
        _errors: optionNone(),
        daysPerWeek: optionNone(),
        rowHeight: optionNone(),
        visibleRoutes: optionNone(),
        detailedCards: optionNone()
    });
    let tainted = $state<ScheduleSettingsTainted>({
        daysPerWeek: optionNone(),
        rowHeight: optionNone(),
        visibleRoutes: optionNone(),
        detailedCards: optionNone()
    });
    const fields: ScheduleSettingsFieldControllers = {
        daysPerWeek: {
            path: ['daysPerWeek'] as const,
            name: 'daysPerWeek',
            constraints: { required: true },
            get: () => data.daysPerWeek,
            set: (value: number) => {
                data.daysPerWeek = value;
            },
            transform: (value: number): number => value,
            getError: () => errors.daysPerWeek,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.daysPerWeek = value;
            },
            getTainted: () => tainted.daysPerWeek,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.daysPerWeek = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = scheduleSettingsValidateField('daysPerWeek', data.daysPerWeek);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
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
            setError: (value: __gf_Option<Array<string>>) => {
                errors.rowHeight = value;
            },
            getTainted: () => tainted.rowHeight,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.rowHeight = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = scheduleSettingsValidateField('rowHeight', data.rowHeight);
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        },
        visibleRoutes: {
            path: ['visibleRoutes'] as const,
            name: 'visibleRoutes',
            constraints: { required: true },
            get: () => data.visibleRoutes,
            set: (value: string[]) => {
                data.visibleRoutes = value;
            },
            transform: (value: string[]): string[] => value,
            getError: () => errors.visibleRoutes,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.visibleRoutes = value;
            },
            getTainted: () => tainted.visibleRoutes,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.visibleRoutes = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = scheduleSettingsValidateField(
                    'visibleRoutes',
                    data.visibleRoutes
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            },
            at: (index: number) => ({
                path: ['visibleRoutes', index] as const,
                name: `visibleRoutes.${index}`,
                constraints: { required: true },
                get: () => data.visibleRoutes[index]!,
                set: (value: string) => {
                    data.visibleRoutes[index] = value;
                },
                transform: (value: string): string => value,
                getError: () => errors.visibleRoutes,
                setError: (value: __gf_Option<Array<string>>) => {
                    errors.visibleRoutes = value;
                },
                getTainted: () => tainted.visibleRoutes,
                setTainted: (value: __gf_Option<boolean>) => {
                    tainted.visibleRoutes = value;
                },
                validate: (): Array<string> => []
            }),
            push: (item: string) => {
                data.visibleRoutes.push(item);
            },
            remove: (index: number) => {
                data.visibleRoutes.splice(index, 1);
            },
            swap: (a: number, b: number) => {
                const tmp = data.visibleRoutes[a]!;
                data.visibleRoutes[a] = data.visibleRoutes[b]!;
                data.visibleRoutes[b] = tmp;
            }
        },
        detailedCards: {
            path: ['detailedCards'] as const,
            name: 'detailedCards',
            constraints: { required: true },
            get: () => data.detailedCards,
            set: (value: boolean) => {
                data.detailedCards = value;
            },
            transform: (value: boolean): boolean => value,
            getError: () => errors.detailedCards,
            setError: (value: __gf_Option<Array<string>>) => {
                errors.detailedCards = value;
            },
            getTainted: () => tainted.detailedCards,
            setTainted: (value: __gf_Option<boolean>) => {
                tainted.detailedCards = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = scheduleSettingsValidateField(
                    'detailedCards',
                    data.detailedCards
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Exit<ScheduleSettings, Array<{ field: string; message: string }>> {
        return toExit(scheduleSettingsDeserialize(data));
    }
    function reset(newOverrides?: Partial<ScheduleSettings>): void {
        data = { ...scheduleSettingsDefaultValue(), ...newOverrides };
        errors = {
            _errors: optionNone(),
            daysPerWeek: optionNone(),
            rowHeight: optionNone(),
            visibleRoutes: optionNone(),
            detailedCards: optionNone()
        };
        tainted = {
            daysPerWeek: optionNone(),
            rowHeight: optionNone(),
            visibleRoutes: optionNone(),
            detailedCards: optionNone()
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
export function scheduleSettingsFromFormData(
    formData: FormData
): Exit<ScheduleSettings, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const daysPerWeekStr = formData.get('daysPerWeek');
        obj.daysPerWeek = daysPerWeekStr ? parseFloat(daysPerWeekStr as string) : 0;
        if (obj.daysPerWeek !== undefined && isNaN(obj.daysPerWeek as number)) obj.daysPerWeek = 0;
    }
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
    obj.visibleRoutes = formData.getAll('visibleRoutes') as Array<string>;
    {
        const detailedCardsVal = formData.get('detailedCards');
        obj.detailedCards =
            detailedCardsVal === 'true' || detailedCardsVal === 'on' || detailedCardsVal === '1';
    }
    return toExit(scheduleSettingsDeserialize(obj));
}

export const ScheduleSettings = {
    defaultValue: scheduleSettingsDefaultValue,
    serialize: scheduleSettingsSerialize,
    serializeWithContext: scheduleSettingsSerializeWithContext,
    deserialize: scheduleSettingsDeserialize,
    deserializeWithContext: scheduleSettingsDeserializeWithContext,
    validateFields: scheduleSettingsValidateFields,
    hasShape: scheduleSettingsHasShape,
    is: scheduleSettingsIs,
    createForm: scheduleSettingsCreateForm,
    fromFormData: scheduleSettingsFromFormData
} as const;
