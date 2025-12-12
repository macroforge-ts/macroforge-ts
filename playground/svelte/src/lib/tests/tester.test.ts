/**
 * Svelte 5 SuperForm tests.
 * Tests the client-side SuperForm class with the new Svelte 5 API.
 */
import './svelte5-mocks.js';

import { describe, it, expect, beforeEach, test } from 'vitest';
import { Schema } from 'effect';
import { effect } from '$lib/form/adapters/effect.js';
import { superValidate, type SuperValidated } from '$lib/form/superValidate.js';
import { SuperForm } from '$lib/form/client/super-form/index.svelte.js';
import { SuperFormError } from '$lib/form/errors.js';

// Test schema using Effect
const UserSchema = Schema.Struct({
    name: Schema.String.annotations({ default: 'Unknown' }),
    email: Schema.String.pipe(
        Schema.filter((s) => /^[^@]+@[^@]+\.[^@]+$/.test(s) || 'must be a valid email', {
            jsonSchema: {}
        })
    ),
    score: Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0)),
    tags: Schema.Array(Schema.String.pipe(Schema.minLength(2))).pipe(Schema.minItems(3))
});

type User = Schema.Schema.Type<typeof UserSchema>;

const adapter = effect(UserSchema);

describe('SuperForm - Svelte 5 Class API', () => {
    let validated: SuperValidated<User>;
    let form: SuperForm<User>;

    beforeEach(async () => {
        validated = await superValidate(adapter);
        form = new SuperForm(validated, { validators: adapter, dataType: 'json' });
    });

    describe('Basic Form Properties', () => {
        it('should create a SuperForm instance', () => {
            expect(form).toBeInstanceOf(SuperForm);
        });

        it('should have initial form data from validation', () => {
            const data = form.form;
            expect(data.name).toBe('Unknown');
            expect(data.email).toBe('');
            expect(data.score).toBe(0);
            expect(data.tags).toEqual([]);
        });

        it('should have constraints from the schema', () => {
            const constraints = form.constraints.current;
            expect(constraints).toBeDefined();
        });

        it('should have an errors controller', () => {
            expect(form.errors.current).toBeDefined();
        });

        it('should not be submitting initially', () => {
            expect(form.submitting.current).toBe(false);
        });

        it('should not be delayed initially', () => {
            expect(form.delayed.current).toBe(false);
        });

        it('should not have timed out initially', () => {
            expect(form.timeout.current).toBe(false);
        });
    });

    describe('Form State Access', () => {
        it('should access form data via form property', () => {
            const data = form.form;
            expect(data).toHaveProperty('name');
            expect(data).toHaveProperty('email');
            expect(data).toHaveProperty('score');
            expect(data).toHaveProperty('tags');
        });

        it('should access form data via formController', () => {
            const controller = form.formController;
            expect(controller).toBeDefined();

            const snapshot = controller.capture(false);
            expect(snapshot.data.name).toBe('Unknown');
        });

        it('should access errors via errors controller', () => {
            const errorsController = form.errors.current;
            const errors = errorsController.current;
            expect(errors).toEqual({});
        });

        it('should access all flattened errors', () => {
            const allErrors = form.allErrors.current;
            expect(Array.isArray(allErrors)).toBe(true);
        });
    });

    describe('Form Data Modification', () => {
        it('should update form data via state box', () => {
            const controller = form.formController;

            // Modify through the stateBox
            controller.stateBox.current = {
                value: { ...form.form, name: 'Updated Name' },
                opts: true // taint
            };

            expect(form.form.name).toBe('Updated Name');
        });
    });

    describe('Capture and Restore', () => {
        it('should capture form state', () => {
            const snapshot = form.capture();

            expect(snapshot).toHaveProperty('data');
            expect(snapshot.data.name).toBe('Unknown');
        });

        it('should capture without files', () => {
            const snapshot = form.capture(true);

            expect(snapshot).toBeDefined();
            expect(snapshot.data).toBeDefined();
        });
    });

    describe('Form Reset', () => {
        it('should reset to initial state', async () => {
            // Modify form
            const controller = form.formController;
            controller.stateBox.current = {
                value: { ...form.form, name: 'Modified' },
                opts: true
            };

            expect(form.form.name).toBe('Modified');

            // Reset
            form.reset();

            expect(form.form.name).toBe('Unknown');
        });

        it('should reset to custom state', async () => {
            const newState: User = {
                name: 'Custom Reset',
                email: 'custom@test.com',
                score: 50,
                tags: ['aa', 'bb', 'cc']
            };

            form.reset({ newState });

            expect(form.form.name).toBe('Custom Reset');
            expect(form.form.email).toBe('custom@test.com');
        });
    });

    describe('Form Validation', () => {
        it('should validate a specific field', async () => {
            // Set invalid value
            const controller = form.formController;
            controller.stateBox.current = {
                value: { ...form.form, score: -5 },
                opts: false // don't taint
            };

            const errors = await form.validate(['score']);

            expect(errors).toBeDefined();
        });

        it('should validate entire form', async () => {
            const result = await form.validateForm();

            expect(result).toHaveProperty('valid');
            expect(result).toHaveProperty('data');
            expect(result).toHaveProperty('errors');
        });
    });

    describe('Tainted State', () => {
        it('should track tainted state', () => {
            const isTainted = form.isTainted.current;
            expect(typeof isTainted).toBe('function');
        });

        it('should not be tainted initially', () => {
            expect(form.isTainted.current()).toBe(false);
        });
    });

    describe('Options', () => {
        it('should have form options', () => {
            const options = form.options.current;

            expect(options).toBeDefined();
            expect(options.dataType).toBe('json');
        });

        it('should have default options applied', () => {
            const options = form.options.current;

            // Check some defaults from defaultFormOptions
            expect(options.resetForm).toBe(true);
            expect(options.scrollToError).toBe('smooth');
        });
    });
});

describe('SuperForm - Error Handling', () => {
    it('should throw if nested data without dataType json', async () => {
        const NestedSchema = Schema.Struct({
            user: Schema.Struct({
                name: Schema.String
            })
        });

        const nestedAdapter = effect(NestedSchema);
        const validated = await superValidate(nestedAdapter);

        // Without dataType: 'json', nested data should throw
        expect(() => new SuperForm(validated)).toThrowError(SuperFormError);
    });

    it('should not throw with dataType json for nested data', async () => {
        const NestedSchema = Schema.Struct({
            user: Schema.Struct({
                name: Schema.String
            })
        });

        const nestedAdapter = effect(NestedSchema);
        const validated = await superValidate(nestedAdapter);

        // With dataType: 'json', should work
        expect(() => new SuperForm(validated, { dataType: 'json' })).not.toThrow();
    });
});

describe('SuperForm - With Valid Data', () => {
    it('should accept pre-validated data', async () => {
        const validData: User = {
            name: 'Alice',
            email: 'alice@test.com',
            score: 100,
            tags: ['aa', 'bb', 'cc']
        };

        const validated = await superValidate(validData, adapter);
        expect(validated.valid).toBe(true);

        const form = new SuperForm(validated, { dataType: 'json' });

        expect(form.form.name).toBe('Alice');
        expect(form.form.email).toBe('alice@test.com');
    });
});

describe('SuperForm - Multiple Instances', () => {
    it('should allow multiple independent form instances', async () => {
        const validated1 = await superValidate(adapter);
        const validated2 = await superValidate(adapter);

        const form1 = new SuperForm(validated1, { dataType: 'json' });
        const form2 = new SuperForm(validated2, { dataType: 'json' });

        // Modify form1
        form1.formController.stateBox.current = {
            value: { ...form1.form, name: 'Form 1' },
            opts: true
        };

        // form2 should be unaffected
        expect(form1.form.name).toBe('Form 1');
        expect(form2.form.name).toBe('Unknown');
    });
});
