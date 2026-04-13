/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Table =
  | /** @default */ "Account"
  | "Did"
  | "Appointment"
  | "Lead"
  | "TaxRate"
  | "Site"
  | "Employee"
  | "Route"
  | "Company"
  | "Product"
  | "Service"
  | "User"
  | "Order"
  | "Payment"
  | "Package"
  | "Promotion"
  | "Represents"
  | "Ordered";
