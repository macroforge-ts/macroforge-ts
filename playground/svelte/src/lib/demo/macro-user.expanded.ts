import { Derive, Debug, debug } from "@ts-macros/swc-napi";
import { Schema } from "effect";
import {
  TaxRate,
  Site,
  Represents,
  Ordered,
  Did,
  AccountName,
  Sector,
  PhoneNumber,
  Email,
  Colors,
} from "../types/bindings";
import { JSON } from "@playground/macro";

@Derive(Debug, JSON)
export class MacroUser {
  @debug({ rename: "userId" })
  id: string;

  name: string;
  role: string;
  favoriteMacro: "Derive" | "JsonNative";
  since: string;

  @debug({ skip: true })
  apiToken: string;

  constructor(
    id: string,
    name: string,
    role: string,
    favoriteMacro: "Derive" | "JsonNative",
    since: string,
    apiToken: string,
  ) {
    this.id = id;
    this.name = name;
    this.role = role;
    this.favoriteMacro = favoriteMacro;
    this.since = since;
    this.apiToken = apiToken;
  }

    toString(): string {
    const parts: string[] = [];
    parts.push("userId: " + this.id);
    parts.push("name: " + this.name);
    parts.push("role: " + this.role);
    parts.push("favoriteMacro: " + this.favoriteMacro);
    parts.push("since: " + this.since);
    return "MacroUser { " + parts.join(", ") + " }";
}

    toJSON(): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    result.id = this.id;
    result.name = this.name;
    result.role = this.role;
    result.favoriteMacro = this.favoriteMacro;
    result.since = this.since;
    result.apiToken = this.apiToken;
    return result;
}
}

const showcaseUser = new MacroUser(
  "usr_2626",
  "Alya Vector",
  "Macro Explorer",
  "Derive",
  "2024-09-12",
  "svelte-secret-token",
);

export const showcaseUserSummary = showcaseUser.toString();
export const showcaseUserJson = showcaseUser.toJSON();

export class Account extends Schema.Class<Account>("Account")({
  id: Schema.propertySignature(Schema.String).annotations({
    missingMessage: () => `'Id' is required`,
  }),
  taxRate: Schema.propertySignature(
    Schema.Union(
      Schema.String.pipe(Schema.nonEmptyString()),
      TaxRate,
    ).annotations({
      message: () => ({
        message: `Please enter a valid value`,
        override: true,
      }),
    }),
  ).annotations({ missingMessage: () => `'Tax Rate' is required` }),
  site: Schema.propertySignature(
    Schema.Union(Schema.String.pipe(Schema.nonEmptyString()), Site).annotations(
      {
        message: () => ({
          message: `Please enter a valid value`,
          override: true,
        }),
      },
    ),
  ).annotations({ missingMessage: () => `'Site' is required` }),
  salesRep: Schema.OptionFromNullishOr(
    Schema.Array(Represents).annotations({
      identifier: `RepresentsRef`,
    }),
    null,
  ),
  orders: Schema.propertySignature(
    Schema.Array(
      Ordered.annotations({
        identifier: `OrderedRef`,
      }),
    ),
  ).annotations({ missingMessage: () => `'Orders' is required` }),
  activity: Schema.propertySignature(
    Schema.Array(
      Did.annotations({
        identifier: `DidRef`,
      }),
    ),
  ).annotations({ missingMessage: () => `'Activity' is required` }),
  customFields: Schema.propertySignature(
    Schema.Array(
      Schema.Tuple(
        Schema.String.pipe(
          Schema.nonEmptyString({ message: () => `Please enter a value` }),
        ),
        Schema.String.pipe(
          Schema.nonEmptyString({ message: () => `Please enter a value` }),
        ),
      ),
    ),
  ).annotations({ missingMessage: () => `'Custom Fields' is required` }),
  accountName: Schema.propertySignature(AccountName).annotations({
    missingMessage: () => `'Account Name' is required`,
  }),
  sector: Schema.propertySignature(Sector).annotations({
    missingMessage: () => `'Sector' is required`,
  }),
  memo: Schema.OptionFromNullishOr(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => `Please enter a value` }),
    ),
    null,
  ),
  phones: Schema.propertySignature(Schema.Array(PhoneNumber)).annotations({
    missingMessage: () => `'Phones' is required`,
  }),
  email: Schema.propertySignature(Email).annotations({
    missingMessage: () => `'Email' is required`,
  }),
  leadSource: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => `Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => `'Lead Source' is required` }),
  colors: Schema.propertySignature(Colors).annotations({
    missingMessage: () => `'Colors' is required`,
  }),
  needsReview: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => `'Needs Review' is required`,
  }),
  hasAlert: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => `'Has Alert' is required`,
  }),
  accountType: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => `Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => `'Account Type' is required` }),
  subtype: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => `Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => `'Subtype' is required` }),
  isTaxExempt: Schema.propertySignature(Schema.Boolean).annotations({
    missingMessage: () => `'Is Tax Exempt' is required`,
  }),
  paymentTerms: Schema.propertySignature(
    Schema.String.pipe(
      Schema.nonEmptyString({ message: () => `Please enter a value` }),
    ),
  ).annotations({ missingMessage: () => `'Payment Terms' is required` }),
  tags: Schema.propertySignature(
    Schema.Array(
      Schema.String.pipe(
        Schema.nonEmptyString({ message: () => `Please enter a value` }),
      ),
    ),
  ).annotations({ missingMessage: () => `'Tags' is required` }),
  dateAdded: Schema.propertySignature(Schema.DateTimeUtc).annotations({
    missingMessage: () => `'Date Added' is required`,
  }),
}) {}
