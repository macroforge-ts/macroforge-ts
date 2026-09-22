// The results the Svelte playground publishes for the Playwright specs. The
// specs compile this module, so every type here comes from the module that
// produces the value: the forms from their generated `createForm`, outcomes
// from the forms' own `validate()` signatures.
import type { Option } from 'effect';
import type { AttributesDemoResults } from './demo/attributes-demo-consumer';
import type { accountCreateForm } from './demo/types/account.svelte';
import type { appointmentCreateForm } from './demo/types/appointment.svelte';
import type { coordinatesCreateForm } from './demo/types/coordinates.svelte';
import type { employeeCreateForm } from './demo/types/employee.svelte';
import type { gradientCreateForm } from './demo/types/gradient.svelte';
import type { leadCreateForm } from './demo/types/lead.svelte';
import type { orderCreateForm } from './demo/types/order.svelte';
import type { phoneNumberCreateForm } from './demo/types/phone-number.svelte';
import type { taxRateCreateForm } from './demo/types/tax-rate.svelte';
import type { userCreateForm } from './demo/types/user.svelte';
import type {
    EventForm,
    ProductForm,
    UserRegistrationForm,
    ValidationResult
} from './demo/validator-form';
import type { OutcomeOf } from './validation-outcome';

type AccountForm = ReturnType<typeof accountCreateForm>;
type AppointmentForm = ReturnType<typeof appointmentCreateForm>;
type CoordinatesForm = ReturnType<typeof coordinatesCreateForm>;
type EmployeeForm = ReturnType<typeof employeeCreateForm>;
type GradientForm = ReturnType<typeof gradientCreateForm>;
type LeadForm = ReturnType<typeof leadCreateForm>;
type OrderForm = ReturnType<typeof orderCreateForm>;
type PhoneNumberForm = ReturnType<typeof phoneNumberCreateForm>;
type TaxRateForm = ReturnType<typeof taxRateCreateForm>;
type UserForm = ReturnType<typeof userCreateForm>;

/** The forms the gigaform pages publish, and their latest validation outcome. */
export interface GigaformResults {
    account?: AccountForm;
    accountValidation?: OutcomeOf<AccountForm> | null;
    appointment?: AppointmentForm;
    appointmentValidation?: OutcomeOf<AppointmentForm> | null;
    coordinates?: CoordinatesForm;
    coordinatesValidation?: OutcomeOf<CoordinatesForm> | null;
    employee?: EmployeeForm;
    employeeValidation?: OutcomeOf<EmployeeForm> | null;
    gradient?: GradientForm;
    gradientValidation?: OutcomeOf<GradientForm> | null;
    lead?: LeadForm;
    leadValidation?: OutcomeOf<LeadForm> | null;
    order?: OrderForm;
    orderValidation?: OutcomeOf<OrderForm> | null;
    phoneNumber?: PhoneNumberForm;
    phoneValidation?: OutcomeOf<PhoneNumberForm> | null;
    taxRate?: TaxRateForm;
    taxRateValidation?: OutcomeOf<TaxRateForm> | null;
    user?: UserForm;
    userValidation?: OutcomeOf<UserForm> | null;
}

export interface ValidatorFormResults {
    userRegistration?: ValidationResult<UserRegistrationForm>;
    product?: ValidationResult<ProductForm>;
    event?: ValidationResult<EventForm>;
}

/**
 * Everything the Svelte playground publishes for the Playwright specs. Each
 * page fills in the slices it renders.
 */
export interface SveltePlayground {
    attributes?: AttributesDemoResults;
    gigaform?: GigaformResults;
    validatorForm?: ValidatorFormResults;
    /**
     * The `Option` module the forms use. Field errors and tainted flags are
     * Options, so the specs build and read them with the same module.
     */
    effectOption?: typeof Option;
}

declare global {
    var sveltePlayground: SveltePlayground | undefined;
}

/** The playground's results object, created on first use. */
export function playgroundResults(): SveltePlayground {
    globalThis.sveltePlayground ??= {};
    return globalThis.sveltePlayground;
}
