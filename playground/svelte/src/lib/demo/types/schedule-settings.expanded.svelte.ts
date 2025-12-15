import { SerializeContext } from 'macroforge/serde';
import { __serializeRowHeight } from './row-height.svelte';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { __deserializeRowHeight } from './row-height.svelte';
import { Option } from 'macroforge/utils';
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

export function defaultValueScheduleSettings(): ScheduleSettings {
    return {
        daysPerWeek: 0,
        rowHeight: 'Medium',
        visibleRoutes: [],
        detailedCards: false
    } as ScheduleSettings;
}

export function toStringifiedJSONScheduleSettings(value: ScheduleSettings): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializeScheduleSettings(value, ctx));
}
export function toObjectScheduleSettings(value: ScheduleSettings): Record<string, unknown> {
    const ctx = SerializeContext.create();
    return __serializeScheduleSettings(value, ctx);
}
export function __serializeScheduleSettings(
    value: ScheduleSettings,
    ctx: SerializeContext
): Record<string, unknown> {
    const existingId = ctx.getId(value);
    if (existingId !== undefined) {
        return { __ref: existingId };
    }
    const __id = ctx.register(value);
    const result: Record<string, unknown> = { __type: 'ScheduleSettings', __id };
    result['daysPerWeek'] = value.daysPerWeek;
    result['rowHeight'] = __serializeRowHeight(value.rowHeight, ctx);
    result['visibleRoutes'] = value.visibleRoutes;
    result['detailedCards'] = value.detailedCards;
    return result;
}

export function fromStringifiedJSONScheduleSettings(
    json: string,
    opts?: DeserializeOptions
): Result<ScheduleSettings, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectScheduleSettings(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectScheduleSettings(
    obj: unknown,
    opts?: DeserializeOptions
): Result<ScheduleSettings, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializeScheduleSettings(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                {
                    field: '_root',
                    message: 'ScheduleSettings.fromObject: root cannot be a forward reference'
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
export function __deserializeScheduleSettings(
    value: any,
    ctx: DeserializeContext
): ScheduleSettings | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref);
    }
    if (typeof value !== 'object' || value === null || Array.isArray(value)) {
        throw new DeserializeError([
            { field: '_root', message: 'ScheduleSettings.__deserialize: expected an object' }
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
        throw new DeserializeError(errors);
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
            const __result = __deserializeRowHeight(__raw_rowHeight, ctx);
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
        throw new DeserializeError(errors);
    }
    return instance as ScheduleSettings;
}
export function validateFieldScheduleSettings<K extends keyof ScheduleSettings>(
    field: K,
    value: ScheduleSettings[K]
): Array<{ field: string; message: string }> {
    return [];
}
export function validateFieldsScheduleSettings(
    partial: Partial<ScheduleSettings>
): Array<{ field: string; message: string }> {
    return [];
}
export function hasShapeScheduleSettings(obj: unknown): boolean {
    if (typeof obj !== 'object' || obj === null || Array.isArray(obj)) {
        return false;
    }
    const o = obj as Record<string, unknown>;
    return 'daysPerWeek' in o && 'rowHeight' in o && 'visibleRoutes' in o && 'detailedCards' in o;
}
export function isScheduleSettings(obj: unknown): obj is ScheduleSettings {
    if (!hasShapeScheduleSettings(obj)) {
        return false;
    }
    const result = fromObjectScheduleSettings(obj);
    return Result.isOk(result);
}

/** Nested error structure matching the data shape */ export type ErrorsScheduleSettings = {
    _errors: Option<Array<string>>;
    daysPerWeek: Option<Array<string>>;
    rowHeight: Option<Array<string>>;
    visibleRoutes: Option<Array<string>>;
    detailedCards: Option<Array<string>>;
}; /** Nested boolean structure for tracking touched/dirty fields */
export type TaintedScheduleSettings = {
    daysPerWeek: Option<boolean>;
    rowHeight: Option<boolean>;
    visibleRoutes: Option<boolean>;
    detailedCards: Option<boolean>;
}; /** Type-safe field controllers for this form */
export interface FieldControllersScheduleSettings {
    readonly daysPerWeek: FieldController<number>;
    readonly rowHeight: FieldController<RowHeight>;
    readonly visibleRoutes: ArrayFieldController<string>;
    readonly detailedCards: FieldController<boolean>;
} /** Gigaform instance containing reactive state and field controllers */
export interface GigaformScheduleSettings {
    readonly data: ScheduleSettings;
    readonly errors: ErrorsScheduleSettings;
    readonly tainted: TaintedScheduleSettings;
    readonly fields: FieldControllersScheduleSettings;
    validate(): Result<ScheduleSettings, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<ScheduleSettings>): void;
} /** Creates a new Gigaform instance with reactive state and field controllers. */
export function createFormScheduleSettings(
    overrides?: Partial<ScheduleSettings>
): GigaformScheduleSettings {
    let data = $state({ ...defaultValueScheduleSettings(), ...overrides });
    let errors = $state<ErrorsScheduleSettings>({
        _errors: Option.none(),
        daysPerWeek: Option.none(),
        rowHeight: Option.none(),
        visibleRoutes: Option.none(),
        detailedCards: Option.none()
    });
    let tainted = $state<TaintedScheduleSettings>({
        daysPerWeek: Option.none(),
        rowHeight: Option.none(),
        visibleRoutes: Option.none(),
        detailedCards: Option.none()
    });
    const fields: FieldControllersScheduleSettings = {
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
            setError: (value: Option<Array<string>>) => {
                errors.daysPerWeek = value;
            },
            getTainted: () => tainted.daysPerWeek,
            setTainted: (value: Option<boolean>) => {
                tainted.daysPerWeek = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldScheduleSettings('daysPerWeek', data.daysPerWeek);
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
            setError: (value: Option<Array<string>>) => {
                errors.rowHeight = value;
            },
            getTainted: () => tainted.rowHeight,
            setTainted: (value: Option<boolean>) => {
                tainted.rowHeight = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldScheduleSettings('rowHeight', data.rowHeight);
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
            setError: (value: Option<Array<string>>) => {
                errors.visibleRoutes = value;
            },
            getTainted: () => tainted.visibleRoutes,
            setTainted: (value: Option<boolean>) => {
                tainted.visibleRoutes = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldScheduleSettings(
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
                setError: (value: Option<Array<string>>) => {
                    errors.visibleRoutes = value;
                },
                getTainted: () => tainted.visibleRoutes,
                setTainted: (value: Option<boolean>) => {
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
            setError: (value: Option<Array<string>>) => {
                errors.detailedCards = value;
            },
            getTainted: () => tainted.detailedCards,
            setTainted: (value: Option<boolean>) => {
                tainted.detailedCards = value;
            },
            validate: (): Array<string> => {
                const fieldErrors = validateFieldScheduleSettings(
                    'detailedCards',
                    data.detailedCards
                );
                return fieldErrors.map((e: { field: string; message: string }) => e.message);
            }
        }
    };
    function validate(): Result<ScheduleSettings, Array<{ field: string; message: string }>> {
        return fromObjectScheduleSettings(data);
    }
    function reset(newOverrides?: Partial<ScheduleSettings>): void {
        data = { ...defaultValueScheduleSettings(), ...newOverrides };
        errors = {
            _errors: Option.none(),
            daysPerWeek: Option.none(),
            rowHeight: Option.none(),
            visibleRoutes: Option.none(),
            detailedCards: Option.none()
        };
        tainted = {
            daysPerWeek: Option.none(),
            rowHeight: Option.none(),
            visibleRoutes: Option.none(),
            detailedCards: Option.none()
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
export function fromFormDataScheduleSettings(
    formData: FormData
): Result<ScheduleSettings, Array<{ field: string; message: string }>> {
    const obj: Record<string, unknown> = {};
    {
        const daysPerWeekStr = formData.get('daysPerWeek');
        obj.daysPerWeek = daysPerWeekStr ? parseFloat(daysPerWeekStr as string) : 0;
        if (obj.daysPerWeek !== undefined && isNaN(obj.daysPerWeek as number)) obj.daysPerWeek = 0;
    }
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
    obj.visibleRoutes = formData.getAll('visibleRoutes') as Array<string>;
    {
        const detailedCardsVal = formData.get('detailedCards');
        obj.detailedCards =
            detailedCardsVal === 'true' || detailedCardsVal === 'on' || detailedCardsVal === '1';
    }
    return fromStringifiedJSONScheduleSettings(JSON.stringify(obj));
}
