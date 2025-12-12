/** import macro {Gigaform} from "@playground/macro"; */

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface User {
  id: string;
  email: string | null;
  /** @serde({ validate: ["nonEmpty"] }) */
  firstName: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  lastName: string;
  password: string | null;
  metadata: Metadata | null;
  settings: Settings;
  /** @default("Administrator") */
  role: UserRole;
  emailVerified: boolean;
  verificationToken: string | null;
  verificationExpires: string | null;
  passwordResetToken: string | null;
  passwordResetExpires: string | null;
  permissions: AppPermissions;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Service {
  id: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  quickCode: string;
  group: string | null;
  subgroup: string | null;
  unit: string | null;
  active: boolean;
  commission: boolean;
  favorite: boolean;
  averageTime: string | null;
  defaults: ServiceDefaults;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface ServiceDefaults {
  price: number;
  /** @serde({ validate: ["nonEmpty"] }) */
  description: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Did {
  /** @default("") */
  in: string | Actor;
  /** @default("") */
  out: string | Target;
  id: string;
  activityType: ActivityType;
  createdAt: string;
  metadata: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface PersonName {
  /** @serde({ validate: ["nonEmpty"] }) */
  firstName: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  lastName: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Promotion {
  id: string;
  date: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Site {
  id: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  addressLine1: string;
  addressLine2: string | null;
  sublocalityLevel1: string | null;
  /** @serde({ validate: ["nonEmpty"] }) */
  locality: string;
  administrativeAreaLevel3: string | null;
  administrativeAreaLevel2: string | null;
  /** @serde({ validate: ["nonEmpty"] }) */
  administrativeAreaLevel1: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  country: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  postalCode: string;
  postalCodeSuffix: string | null;
  coordinates: Coordinates;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Metadata {
  createdAt: string;
  lastLogin: string | null;
  isActive: boolean;
  roles: string[];
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface ColumnConfig {
  /** @serde({ validate: ["nonEmpty"] }) */
  heading: string;
  dataPath: DataPath;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface PhoneNumber {
  main: boolean;
  /** @serde({ validate: ["nonEmpty"] }) */
  phoneType: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  number: string;
  canText: boolean;
  canCall: boolean;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Gradient {
  startHue: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Product {
  id: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  quickCode: string;
  group: string | null;
  subgroup: string | null;
  unit: string | null;
  active: boolean;
  commission: boolean;
  favorite: boolean;
  defaults: ProductDefaults;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface YearlyRecurrenceRule {
  quantityOfYears: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface AppointmentNotifications {
  /** @serde({ validate: ["nonEmpty"] }) */
  personalScheduleChangeNotifications: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  allScheduleChangeNotifications: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface DirectionHue {
  bearing: number;
  hue: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface MonthlyRecurrenceRule {
  quantityOfMonths: number;
  day: number;
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Represents {
  /** @default("") */
  in: string | Employee;
  /** @default("") */
  out: string | Account;
  id: string;
  dateStarted: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Payment {
  id: string;
  date: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Settings {
  appointmentNotifications: AppointmentNotifications | null;
  commissions: Commissions | null;
  scheduleSettings: ScheduleSettings;
  accountOverviewSettings: OverviewSettings;
  serviceOverviewSettings: OverviewSettings;
  appointmentOverviewSettings: OverviewSettings;
  leadOverviewSettings: OverviewSettings;
  packageOverviewSettings: OverviewSettings;
  productOverviewSettings: OverviewSettings;
  orderOverviewSettings: OverviewSettings;
  taxRateOverviewSettings: OverviewSettings;
  /** @default("UserHome") */
  homePage: Page;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Color {
  red: number;
  green: number;
  blue: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface CompanyName {
  /** @serde({ validate: ["nonEmpty"] }) */
  companyName: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Appointment {
  id: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  title: string;
  /** @default("Scheduled") */
  status: Status;
  begins: string;
  duration: number;
  timeZone: string;
  offsetMs: number;
  allDay: boolean;
  multiDay: boolean;
  employees: (string | Employee)[];
  /** @default("") */
  location: string | Site;
  description: string | null;
  /** @default({ main: "#000000", hover: "#333333", text: "#ffffff" }) */
  colors: Colors;
  recurrenceRule: RecurrenceRule | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Package {
  id: string;
  date: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface ScheduleSettings {
  daysPerWeek: number;
  /** @default("Medium") */
  rowHeight: RowHeight;
  visibleRoutes: string[];
  detailedCards: boolean;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface DailyRecurrenceRule {
  quantityOfDays: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface SignUpCredentials {
  firstName: FirstName;
  lastName: LastName;
  email: EmailParts;
  password: Password;
  rememberMe: boolean;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface OverviewSettings {
  /** @default("Medium") */
  rowHeight: RowHeight;
  /** @default("Table") */
  cardOrRow: OverviewDisplay;
  perPage: number;
  columnConfigs: ColumnConfig[];
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface FirstName {
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Account {
  id: string;
  /** @default("") */
  taxRate: string | TaxRate;
  /** @default("") */
  site: string | Site;
  salesRep: Represents[] | null;
  orders: Ordered[];
  activity: Did[];
  customFields: [string, string][];
  accountName: AccountName;
  /** @default("Residential") */
  sector: Sector;
  memo: string | null;
  phones: PhoneNumber[];
  email: Email;
  /** @serde({ validate: ["nonEmpty"] }) */
  leadSource: string;
  colors: Colors;
  needsReview: boolean;
  hasAlert: boolean;
  /** @serde({ validate: ["nonEmpty"] }) */
  accountType: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  subtype: string;
  isTaxExempt: boolean;
  /** @serde({ validate: ["nonEmpty"] }) */
  paymentTerms: string;
  tags: string[];
  dateAdded: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Edited {
  /** @serde({ validate: ["nonEmpty"] }) */
  fieldName: string;
  oldValue: string | null;
  newValue: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Order {
  id: string;
  /** @default("") */
  account: string | Account;
  /** @default("Estimate") */
  stage: OrderStage;
  number: number;
  payments: (string | Payment)[];
  /** @serde({ validate: ["nonEmpty"] }) */
  opportunity: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  reference: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  leadSource: string;
  /** @default("") */
  salesRep: string | Employee;
  /** @serde({ validate: ["nonEmpty"] }) */
  group: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  subgroup: string;
  isPosted: boolean;
  needsReview: boolean;
  /** @serde({ validate: ["nonEmpty"] }) */
  actionItem: string;
  upsale: number;
  dateCreated: string;
  /** @default("") */
  appointment: string | Appointment;
  lastTechs: (string | Employee)[];
  package: (string | Package)[] | null;
  promotion: (string | Promotion)[] | null;
  balance: number;
  due: string;
  total: number;
  /** @default("") */
  site: string | Site;
  billedItems: BilledItem[];
  /** @serde({ validate: ["nonEmpty"] }) */
  memo: string;
  discount: number;
  tip: number;
  commissions: number[];
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Commented {
  /** @serde({ validate: ["nonEmpty"] }) */
  comment: string;
  replyTo: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Custom {
  mappings: DirectionHue[];
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Colors {
  /** @serde({ validate: ["nonEmpty"] }) */
  main: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  hover: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  active: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface ProductDefaults {
  price: number;
  /** @serde({ validate: ["nonEmpty"] }) */
  description: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Viewed {
  durationSeconds: number | null;
  source: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface WeeklyRecurrenceRule {
  quantityOfWeeks: number;
  weekdays: Weekday[];
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Paid {
  amount: number | null;
  currency: string | null;
  paymentMethod: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface TaxRate {
  id: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  taxAgency: string;
  zip: number;
  /** @serde({ validate: ["nonEmpty"] }) */
  city: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  county: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  state: string;
  isActive: boolean;
  /** @serde({ validate: ["nonEmpty"] }) */
  description: string;
  /** @default({}) */
  taxComponents: { [key: string]: number };
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Address {
  /** @serde({ validate: ["nonEmpty"] }) */
  street: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  city: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  state: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  zipcode: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Lead {
  id: string;
  number: number | null;
  accepted: boolean;
  probability: number;
  /** @default("Medium") */
  priority: Priority;
  dueDate: string | null;
  closeDate: string | null;
  value: number;
  /** @default("Open") */
  stage: LeadStage;
  /** @serde({ validate: ["nonEmpty"] }) */
  status: string;
  description: string | null;
  /** @default("InitialContact") */
  nextStep: NextStep;
  favorite: boolean;
  dateAdded: string | null;
  taxRate: (string | TaxRate) | null;
  /** @default("Residential") */
  sector: Sector;
  leadName: AccountName;
  phones: PhoneNumber[];
  email: Email;
  leadSource: string | null;
  /** @default("") */
  site: string | Site;
  /** @serde({ validate: ["nonEmpty"] }) */
  memo: string;
  needsReview: boolean;
  hasAlert: boolean;
  salesRep: Represents[] | null;
  color: string | null;
  /** @serde({ validate: ["nonEmpty"] }) */
  accountType: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  subtype: string;
  isTaxExempt: boolean;
  /** @serde({ validate: ["nonEmpty"] }) */
  paymentTerms: string;
  tags: string[];
  customFields: [string, string][];
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface AppPermissions {
  applications: Applications[];
  pages: Page[];
  data: Table[];
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Company {
  id: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  legalName: string;
  /** @default("") */
  headquarters: string | Site;
  phones: PhoneNumber[];
  /** @serde({ validate: ["nonEmpty"] }) */
  fax: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  email: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  website: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  taxId: string;
  referenceNumber: number;
  /** @serde({ validate: ["nonEmpty"] }) */
  postalCodeLookup: string;
  timeZone: string;
  /** @default("") */
  defaultTax: string | TaxRate;
  /** @serde({ validate: ["nonEmpty"] }) */
  defaultTaxLocation: string;
  defaultAreaCode: number;
  /** @serde({ validate: ["nonEmpty"] }) */
  defaultAccountType: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  lookupFormatting: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  accountNameFormat: string;
  merchantServiceProvider: string | null;
  /** @serde({ validate: ["nonEmpty"] }) */
  dateDisplayStyle: string;
  hasAutoCommission: boolean;
  hasAutoDaylightSavings: boolean;
  hasAutoFmsTracking: boolean;
  hasNotifications: boolean;
  hasRequiredLeadSource: boolean;
  hasRequiredEmail: boolean;
  hasSortServiceItemsAlphabetically: boolean;
  hasAttachOrderToAppointmentEmails: boolean;
  scheduleInterval: number;
  colorsConfig: ColorsConfig;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Ordinal {
  north: number;
  northeast: number;
  east: number;
  southeast: number;
  south: number;
  southwest: number;
  west: number;
  northwest: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Password {
  /** @serde({ validate: ["nonEmpty"] }) */
  password: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Created {
  initialData: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Employee {
  id: string;
  imageUrl: string | null;
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;
  phones: PhoneNumber[];
  /** @serde({ validate: ["nonEmpty"] }) */
  role: string;
  /** @default("Technician") */
  title: JobTitle;
  email: Email;
  /** @serde({ validate: ["nonEmpty"] }) */
  address: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  username: string;
  /** @default("") */
  route: string | Route;
  ratePerHour: number;
  active: boolean;
  isTechnician: boolean;
  isSalesRep: boolean;
  description: string | null;
  linkedinUrl: string | null;
  attendance: string[];
  settings: Settings;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Commissions {
  /** @serde({ validate: ["nonEmpty"] }) */
  technician: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  salesRep: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Number {
  /** @serde({ validate: ["nonEmpty"] }) */
  countryCode: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  areaCode: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  localNumber: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface DataPath {
  path: string[];
  formatter: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Route {
  id: string;
  techs: (string | Employee)[] | null;
  active: boolean;
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  phone: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  position: string;
  serviceRoute: boolean;
  defaultDurationHours: number;
  tags: string[];
  icon: string | null;
  color: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface EmailParts {
  /** @serde({ validate: ["nonEmpty"] }) */
  local: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  domainName: string;
  /** @serde({ validate: ["nonEmpty"] }) */
  topLevelDomain: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Sent {
  recipient: string | null;
  method: string | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface BilledItem {
  /** @default("") */
  item: Item;
  quantity: number;
  taxed: boolean;
  upsale: boolean;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Coordinates {
  lat: number;
  lng: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Ordered {
  id: string;
  /** @default("") */
  in: string | Account;
  /** @default("") */
  out: string | Order;
  date: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Email {
  canEmail: boolean;
  /** @serde({ validate: ["nonEmpty"] }) */
  emailString: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface RecurrenceRule {
  interval: Interval;
  recurrenceBegins: string;
  recurrenceEnds: RecurrenceEnd | null;
  cancelledInstances: string[] | null;
  additionalInstances: string[] | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface LastName {
  /** @serde({ validate: ["nonEmpty"] }) */
  name: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Cardinal {
  north: number;
  east: number;
  south: number;
  west: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Interval =
  | /** @default */ DailyRecurrenceRule
  | WeeklyRecurrenceRule
  | MonthlyRecurrenceRule
  | YearlyRecurrenceRule;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Page =
  | /** @default */ "SalesHomeDashboard"
  | "SalesHomeProducts"
  | "SalesHomeServices"
  | "SalesHomePackages"
  | "SalesHomeTaxRates"
  | "SalesLeadsOverview"
  | "SalesLeadsActivities"
  | "SalesLeadsCampaigns"
  | "SalesLeadsDripCampaigns"
  | "SalesLeadsOpportunities"
  | "SalesLeadsPromotions"
  | "SalesAccountsOverview"
  | "SalesAccountsActivities"
  | "SalesAccountsBilling"
  | "SalesAccountsContracts"
  | "SalesOrdersOverview"
  | "SalesOrdersActivities"
  | "SalesOrdersPayments"
  | "SalesOrdersCommissions"
  | "SalesSchedulingSchedule"
  | "SalesSchedulingAppointments"
  | "SalesSchedulingRecurring"
  | "SalesSchedulingRoutes"
  | "SalesSchedulingReminders"
  | "UserHome";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type UserRole =
  | /** @default */ "Administrator"
  | "SalesRepresentative"
  | "Technician"
  | "HumanResources"
  | "InformationTechnology";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Target =
  | /** @default */ Account
  | User
  | Employee
  | Appointment
  | Lead
  | TaxRate
  | Site
  | Route
  | Company
  | Product
  | Service
  | Order
  | Payment
  | Package
  | Promotion
  | Represents
  | Ordered;

/** @derive(Default, Serialize, Deserialize, Gigaform)
 *  @default(0)
 */
export type RecurrenceEnd = number | string;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type OverviewDisplay = /** @default */ "Card" | "Table";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type IntervalUnit = /** @default */ "Day" | "Week" | "Month" | "Year";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Sector = /** @default */ "Residential" | "Commercial";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Weekday =
  | /** @default */ "Monday"
  | "Tuesday"
  | "Wednesday"
  | "Thursday"
  | "Friday"
  | "Saturday"
  | "Sunday";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Status = /** @default */ "Scheduled" | "OnDeck" | "Waiting";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type NextStep =
  | /** @default */ "InitialContact"
  | "Qualified"
  | "Estimate"
  | "Negotiation";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type LeadStage =
  | /** @default */ "Open"
  | "InitialContact"
  | "Qualified"
  | "Estimate"
  | "Negotiation";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type AccountName = /** @default */ CompanyName | PersonName;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Priority = /** @default */ "Medium" | "High" | "Low";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Applications =
  | /** @default */ "Sales"
  | "Accounting"
  | "Errand"
  | "HumanResources"
  | "Logistics"
  | "Marketing"
  | "Website";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type JobTitle =
  | /** @default */ "Technician"
  | "SalesRepresentative"
  | "HumanResources"
  | "InformationTechnology";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type ColorsConfig =
  | Cardinal
  | Ordinal
  | Custom
  | /** @default */ Gradient;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type WeekOfMonth =
  | /** @default */ "First"
  | "Second"
  | "Third"
  | "Fourth"
  | "Last";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type ActivityType =
  | /** @default */ Created
  | Edited
  | Sent
  | Viewed
  | Commented
  | Paid;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type RowHeight =
  | "ExtraSmall"
  | "Small"
  | /** @default */ "Medium"
  | "Large";

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type OrderStage = /** @default */ "Estimate" | "Active" | "Invoice";

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

/** @derive(Default, Serialize, Deserialize, Gigaform)
 * @default("")
 */
export type Item = (string | Product) | (string | Service);

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Actor = /** @default */ User | Employee | Account;
