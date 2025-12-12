import { box } from 'svelte-toolbelt';
import type { WritableBox } from 'svelte-toolbelt';
import { flattenErrors, updateErrors } from '$lib/form/errors.js';
import type { ValidationErrors } from '$lib/form/superValidate.js';
import type { FlattenedErrorList } from '$lib/form/types/client.js';
import { clone } from '$lib/form/utils.js';

export interface ErrorsBoxValue<T extends object> {
    value: ValidationErrors<T>;
    opts?: { force?: boolean } | undefined;
}

export class ErrorsController<T extends object> {
    private store: WritableBox<ErrorsBoxValue<T>>;
    private readonly flattenedErrors = box.with<FlattenedErrorList>(() =>
        flattenErrors(this.current)
    );
    private exposed?: WritableBox<ValidationErrors<T>> & {
        clear: () => ValidationErrors<T>;
    };

    constructor(
        initial: ValidationErrors<T>,
        private readonly baseline: () => ValidationErrors<T>,
        private readonly sync?: (value: ValidationErrors<T>) => void
    ) {
        let state = $state(clone(initial));
        let payload: ErrorsBoxValue<T> = {
            value: state
        };

        this.store = box.with<ErrorsBoxValue<T>>(
            () => payload,
            (newValue: ErrorsBoxValue<T>) => {
                const nextValue = clone(newValue.value);
                const nextOpts = newValue.opts ?? payload.opts;
                const normalized = updateErrors(nextValue, this.baseline(), nextOpts?.force);

                state = clone(normalized);
                payload = {
                    value: state,
                    opts: nextOpts
                };

                this.sync?.(clone(state));
            }
        );

        this.sync?.(clone(initial));
    }

    get current(): ValidationErrors<T> {
        return this.store.current.value;
    }

    set current(value: ValidationErrors<T> | ErrorsBoxValue<T>) {
        if ('value' in value) {
            this.assign(value);
        } else {
            this.assign({ value });
        }
    }

    get allErrors(): FlattenedErrorList {
        return this.flattenedErrors.current;
    }

    set(value: ValidationErrors<T>, options?: { force?: boolean }): ValidationErrors<T> {
        return this.assign({ value, opts: options });
    }

    update(
        updater: (value: ValidationErrors<T>) => ValidationErrors<T>,
        options?: { force?: boolean }
    ): ValidationErrors<T> {
        const next = updater(clone(this.current));
        return this.assign({ value: next, opts: options });
    }

    clear(): ValidationErrors<T> {
        return this.assign({ value: {} as ValidationErrors<T> });
    }

    expose(): WritableBox<ValidationErrors<T>> & {
        clear: () => ValidationErrors<T>;
    } {
        if (!this.exposed) {
            const exposed = box.with<ValidationErrors<T>>(
                () => this.current,
                (value: ValidationErrors<T>) => {
                    this.current = value;
                }
            ) as WritableBox<ValidationErrors<T>> & {
                clear: () => ValidationErrors<T>;
            };

            Object.defineProperty(exposed, 'clear', {
                value: () => this.clear(),
                enumerable: false
            });

            this.exposed = exposed;
        }
        return this.exposed;
    }

    private assign(payload: ErrorsBoxValue<T>): ValidationErrors<T> {
        this.store.current = payload;
        return this.current;
    }
}
