/**
 * Test types for Gigaform macro testing.
 * These interfaces are used to test the factory pattern, field controllers,
 * and integration with Default, Serialize, and Deserialize macros.
 */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface SimpleForm {
  /** @serde({ validate: ["nonEmpty"] }) */
  /** @textController({ label: "Full Name" }) */
  name: string;

  /** @serde({ validate: ["email"] }) */
  /** @textController({ label: "Email Address", placeholder: "user@example.com" }) */
  email: string;

  /** @serde({ validate: ["positive", "int"] }) */
  /** @numberController({ label: "Age", min: 0, max: 150 }) */
  age: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface FormWithOptionalFields {
  name: string;

  /** @serde({ validate: ["minLength(2)"] }) */
  nickname?: string;

  bio?: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface FormWithArrays {
  title: string;

  /** @serde({ validate: ["minItems(1)", "maxItems(10)"] }) */
  tags: string[];

  scores: number[];
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface FormWithValidation {
  /** @serde({ validate: ["nonEmpty", "minLength(2)", "maxLength(50)"] }) */
  username: string;

  /** @serde({ validate: ["email"] }) */
  email: string;

  /** @serde({ validate: ["minLength(8)", "maxLength(100)"] }) */
  password: string;

  /** @serde({ validate: ["url"] }) */
  website?: string;

  /** @serde({ validate: ["uuid"] }) */
  externalId?: string;

  /** @serde({ validate: ["between(0, 100)"] }) */
  score: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface FormWithControllerMetadata {
  /** @textController({ label: "First Name", placeholder: "John", description: "Your given name" }) */
  firstName: string;

  /** @textController({ label: "Last Name", placeholder: "Doe" }) */
  lastName: string;

  /** @numberController({ label: "Experience (years)", min: 0, max: 50, step: 1 }) */
  yearsExperience: number;

  /** @toggleController({ label: "Subscribe to newsletter" }) */
  subscribed: boolean;
}

// Test for nested objects (when Gigaform supports them)
/** @derive(Default, Serialize, Deserialize) */
export interface Address {
  street: string;
  city: string;
  zipCode: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface FormWithNestedType {
  name: string;
  address: Address;
}

/**
 * @derive(Default, Serialize, Deserialize, Gigaform)
 * @gigaform({ i18nPrefix: "contactForm" })
 */
export interface FormWithI18n {
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;

  /** @serde({ validate: ["email"] }) */
  email: string;

  /** @serde({ validate: ["nonEmpty", "minLength(10)"] }) */
  message: string;
}
