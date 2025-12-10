/**
 * Gigaform - Svelte 5 Form State Management
 *
 * Integrates with the Gigaform macro (@derive(Gigaform)) for compile-time
 * generated form types and validation.
 *
 * @example
 * ```typescript
 * // Define form with Gigaform macro
 * // @derive(Default, Serialize, Deserialize, Gigaform)
 * export interface UserForm {
 *   // @serde({ validate: ["email"] })
 *   // @textController({ label: "Email" })
 *   email: string;
 *
 *   // @serde({ validate: ["minLength(2)"] })
 *   // @textController({ label: "Name" })
 *   name: string;
 * }
 *
 * // Use in Svelte component
 * import { createFormState, GigaformRoot, gigaformEnhance } from '$lib/form/gigaform';
 * import { UserForm } from './schema';
 *
 * const formState = createFormState(UserForm);
 * ```
 *
 * @module
 */

// Core form state creation
export { createFormState } from './createFormState.svelte';

// Context management
export {
    GigaformContext,
    setGigaformContext,
    getGigaformContext,
    tryGetGigaformContext
} from './context.svelte';

// Root component
export { default as GigaformRoot } from './GigaformRoot.svelte';

// SvelteKit enhance integration
export { gigaformEnhance } from './enhance';
export type { GigaformEnhanceOptions, ServerActionResult } from './enhance';

// Controller helpers
export {
    createBaseFieldProps,
    createTextController,
    createTextAreaController,
    createNumberController,
    createToggleController,
    createCheckboxController,
    createSwitchController,
    createSelectController,
    createRadioGroupController,
    createHiddenController,
    createTagsController,
    createControllerFromDescriptor
} from './controller-helpers';
export type {
    ControllerOptions,
    TextControllerOptions,
    TextAreaControllerOptions,
    NumberControllerOptions,
    BooleanControllerOptions,
    SelectControllerOptions
} from './controller-helpers';

// Types
export type {
    // Core types
    FormState,
    FormStateOptions,
    GigaformNamespace,
    FieldDescriptor,
    FieldConstraints,
    FieldsObject,
    // Result types
    GigaformResult,
    GigaformOk,
    GigaformErr
} from './types';

// Utility functions
export { hasErrors, getDescriptorByPath, collectFieldPaths } from './types';
