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
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Quick Code" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    quickCode: string;
    /** @comboboxController({ label: "Group", allowCustom: true }) */
    group: string | null;
    /** @comboboxController({ label: "Subgroup", allowCustom: true }) */
    subgroup: string | null;
    /** @comboboxController({ label: "Unit", allowCustom: true }) */
    unit: string | null;
    /** @switchController({ label: "Active" }) */
    active: boolean;
    /** @switchController({ label: "Commission" }) */
    commission: boolean;
    /** @switchController({ label: "Favorite" }) */
    favorite: boolean;
    /** @textController({ label: "Average Time" }) */
    averageTime: string | null;
    defaults: ServiceDefaults;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface ServiceDefaults {
    /** @numberController({ label: "Price", min: 0, step: 0.01 }) */
    price: number;
    /** @textAreaController({ label: "Description" }) */
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
    /** @textController({ label: "First Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @textController({ label: "Last Name" }) */
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
    /** @switchController({ label: "Main" }) */
    main: boolean;
    /** @comboboxController({ label: "Phone Type", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    phoneType: string;
    /** @textController({ label: "Number" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    number: string;
    /** @switchController({ label: "Can Text" }) */
    canText: boolean;
    /** @switchController({ label: "Can Call" }) */
    canCall: boolean;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Gradient {
    startHue: number;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Product {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Quick Code" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    quickCode: string;
    /** @comboboxController({ label: "Group", allowCustom: true }) */
    group: string | null;
    /** @comboboxController({ label: "Subgroup", allowCustom: true }) */
    subgroup: string | null;
    /** @comboboxController({ label: "Unit", allowCustom: true }) */
    unit: string | null;
    /** @switchController({ label: "Active" }) */
    active: boolean;
    /** @switchController({ label: "Commission" }) */
    commission: boolean;
    /** @switchController({ label: "Favorite" }) */
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
    /** @textController({ label: "Company Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    companyName: string;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Appointment {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Title" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    title: string;
    /** @selectController({ label: "Status", options: [{ label: "Scheduled", value: "Scheduled" }, { label: "On Deck", value: "OnDeck" }, { label: "Waiting", value: "Waiting" }] }) */
    /** @default("Scheduled") */
    status: Status;
    /** @dateTimeController({ label: "Begins" }) */
    begins: string;
    /** @numberController({ label: "Duration", min: 0 }) */
    duration: number;
    /** @textController({ label: "Time Zone" }) */
    timeZone: string;
    /** @hiddenController({}) */
    offsetMs: number;
    /** @switchController({ label: "All Day" }) */
    allDay: boolean;
    /** @switchController({ label: "Multi Day" }) */
    multiDay: boolean;
    /** @comboboxMultipleController({ label: "Employees", fetchUrls: ["/api/employees"] }) */
    employees: (string | Employee)[];
    /** @siteFieldsetController({ label: "Location" }) */
    /** @default("") */
    location: string | Site;
    /** @textAreaController({ label: "Description" }) */
    description: string | null;
    /** @hiddenController({}) */
    /** @default({ main: "#000000", hover: "#333333", active: "#666666" }) */
    colors: Colors;
    /** @hiddenController({}) */
    recurrenceRule: RecurrenceRule | null;
}

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export interface Package {
    /** @hiddenController({}) */
    id: string;
    /** @dateTimeController({ label: "Date" }) */
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
    /** @hiddenController({}) */
    id: string;
    /** @comboboxController({ label: "Tax Rate", allowCustom: false, fetchUrls: ["/api/tax-rates"] }) */
    /** @default("") */
    taxRate: string | TaxRate;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @comboboxMultipleController({ label: "Sales Rep", fetchUrls: ["/api/employees"] }) */
    salesRep: Represents[] | null;
    /** @hiddenController({}) */
    orders: Ordered[];
    /** @hiddenController({}) */
    activity: Did[];
    /** @arrayFieldsetController({ legend: "Custom Fields" }) */
    customFields: [string, string][];
    /** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
    accountName: AccountName;
    /** @radioGroupController({ label: "Sector", options: [{ label: "Residential", value: "Residential" }, { label: "Commercial", value: "Commercial" }], orientation: "horizontal" }) */
    /** @default("Residential") */
    sector: Sector;
    /** @textAreaController({ label: "Memo" }) */
    memo: string | null;
    /** @arrayFieldsetController({ legend: "Phones" }) */
    phones: PhoneNumber[];
    /** @emailFieldController({ label: "Email" }) */
    email: Email;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    leadSource: string;
    /** @hiddenController({}) */
    colors: Colors;
    /** @toggleController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @toggleController({ label: "Has Alert" }) */
    hasAlert: boolean;
    /** @comboboxController({ label: "Account Type", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    accountType: string;
    /** @comboboxController({ label: "Subtype", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    subtype: string;
    /** @toggleController({ label: "Tax Exempt" }) */
    isTaxExempt: boolean;
    /** @comboboxController({ label: "Payment Terms", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    paymentTerms: string;
    /** @tagsController({ label: "Tags" }) */
    tags: string[];
    /** @hiddenController({}) */
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
    /** @hiddenController({}) */
    id: string;
    /** @comboboxController({ label: "Account", allowCustom: false, fetchUrls: ["/api/accounts"] }) */
    /** @default("") */
    account: string | Account;
    /** @selectController({ label: "Stage", options: [{ label: "Estimate", value: "Estimate" }, { label: "Active", value: "Active" }, { label: "Invoice", value: "Invoice" }] }) */
    /** @default("Estimate") */
    stage: OrderStage;
    /** @hiddenController({}) */
    number: number;
    /** @hiddenController({}) */
    payments: (string | Payment)[];
    /** @textController({ label: "Opportunity" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    opportunity: string;
    /** @textController({ label: "Reference" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    reference: string;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    leadSource: string;
    /** @comboboxController({ label: "Sales Rep", allowCustom: false, fetchUrls: ["/api/employees"] }) */
    /** @default("") */
    salesRep: string | Employee;
    /** @comboboxController({ label: "Group", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    group: string;
    /** @comboboxController({ label: "Subgroup", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    subgroup: string;
    /** @switchController({ label: "Posted" }) */
    isPosted: boolean;
    /** @switchController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @textController({ label: "Action Item" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    actionItem: string;
    /** @hiddenController({}) */
    upsale: number;
    /** @hiddenController({}) */
    dateCreated: string;
    /** @comboboxController({ label: "Appointment", allowCustom: false, fetchUrls: ["/api/appointments"] }) */
    /** @default("") */
    appointment: string | Appointment;
    /** @comboboxMultipleController({ label: "Technicians", fetchUrls: ["/api/employees"] }) */
    lastTechs: (string | Employee)[];
    /** @hiddenController({}) */
    package: (string | Package)[] | null;
    /** @hiddenController({}) */
    promotion: (string | Promotion)[] | null;
    /** @hiddenController({}) */
    balance: number;
    /** @dateTimeController({ label: "Due" }) */
    due: string;
    /** @hiddenController({}) */
    total: number;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @arrayFieldsetController({ legend: "Billed Items" }) */
    billedItems: BilledItem[];
    /** @textAreaController({ label: "Memo" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    memo: string;
    /** @hiddenController({}) */
    discount: number;
    /** @hiddenController({}) */
    tip: number;
    /** @hiddenController({}) */
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
    /** @numberController({ label: "Price", min: 0, step: 0.01 }) */
    price: number;
    /** @textAreaController({ label: "Description" }) */
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
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Tax Agency" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    taxAgency: string;
    /** @numberController({ label: "Zip", min: 0 }) */
    zip: number;
    /** @textController({ label: "City" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    city: string;
    /** @textController({ label: "County" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    county: string;
    /** @textController({ label: "State" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    state: string;
    /** @switchController({ label: "Active" }) */
    isActive: boolean;
    /** @textAreaController({ label: "Description" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    description: string;
    /** @hiddenController({}) */
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
    /** @hiddenController({}) */
    id: string;
    /** @hiddenController({}) */
    number: number | null;
    /** @hiddenController({}) */
    accepted: boolean;
    /** @numberController({ label: "Probability", min: 0, max: 100 }) */
    probability: number;
    /** @radioGroupController({ label: "Priority", options: [{ label: "High", value: "High" }, { label: "Medium", value: "Medium" }, { label: "Low", value: "Low" }], orientation: "horizontal" }) */
    /** @default("Medium") */
    priority: Priority;
    /** @dateTimeController({ label: "Due Date" }) */
    dueDate: string | null;
    /** @dateTimeController({ label: "Close Date" }) */
    closeDate: string | null;
    /** @numberController({ label: "Value", min: 0, step: 0.01 }) */
    value: number;
    /** @selectController({ label: "Stage", options: [{ label: "Open", value: "Open" }, { label: "Working", value: "Working" }, { label: "Lost", value: "Lost" }, { label: "Qualified", value: "Qualified" }, { label: "Estimate", value: "Estimate" }, { label: "Negotiation", value: "Negotiation" }] }) */
    /** @default("Open") */
    stage: LeadStage;
    /** @textController({ label: "Status" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    status: string;
    /** @textAreaController({ label: "Description" }) */
    description: string | null;
    /** @hiddenController({}) */
    /** @default("InitialContact") */
    nextStep: NextStep;
    /** @switchController({ label: "Favorite" }) */
    favorite: boolean;
    /** @hiddenController({}) */
    dateAdded: string | null;
    /** @comboboxController({ label: "Tax Rate", allowCustom: false, fetchUrls: ["/api/tax-rates"] }) */
    taxRate: (string | TaxRate) | null;
    /** @radioGroupController({ label: "Sector", options: [{ label: "Residential", value: "Residential" }, { label: "Commercial", value: "Commercial" }], orientation: "horizontal" }) */
    /** @default("Residential") */
    sector: Sector;
    /** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
    leadName: AccountName;
    /** @arrayFieldsetController({ legend: "Phones" }) */
    phones: PhoneNumber[];
    /** @emailFieldController({ label: "Email" }) */
    email: Email;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    leadSource: string | null;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @textAreaController({ label: "Memo" }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    memo: string;
    /** @toggleController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @toggleController({ label: "Has Alert" }) */
    hasAlert: boolean;
    /** @comboboxMultipleController({ label: "Sales Rep", fetchUrls: ["/api/employees"] }) */
    salesRep: Represents[] | null;
    /** @hiddenController({}) */
    color: string | null;
    /** @comboboxController({ label: "Account Type", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    accountType: string;
    /** @comboboxController({ label: "Subtype", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    subtype: string;
    /** @toggleController({ label: "Tax Exempt" }) */
    isTaxExempt: boolean;
    /** @comboboxController({ label: "Payment Terms", allowCustom: true }) */
    /** @serde({ validate: ["nonEmpty"] }) */
    paymentTerms: string;
    /** @tagsController({ label: "Tags" }) */
    tags: string[];
    /** @arrayFieldsetController({ legend: "Custom Fields" }) */
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
    /** @comboboxController({ label: "Item", allowCustom: true, fetchUrls: ["/api/products", "/api/services"] }) */
    /** @default("") */
    item: Item;
    /** @numberController({ label: "Quantity", min: 0, step: 1 }) */
    quantity: number;
    /** @switchController({ label: "Taxed" }) */
    taxed: boolean;
    /** @switchController({ label: "Upsale" }) */
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
    /** @switchController({ label: "Can Email" }) */
    canEmail: boolean;
    /** @textController({ label: "Email" }) */
    /** @serde({ validate: ["nonEmpty", "email"] }) */
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
    | /** @default */ 'SalesHomeDashboard'
    | 'SalesHomeProducts'
    | 'SalesHomeServices'
    | 'SalesHomePackages'
    | 'SalesHomeTaxRates'
    | 'SalesLeadsOverview'
    | 'SalesLeadsActivities'
    | 'SalesLeadsCampaigns'
    | 'SalesLeadsDripCampaigns'
    | 'SalesLeadsOpportunities'
    | 'SalesLeadsPromotions'
    | 'SalesAccountsOverview'
    | 'SalesAccountsActivities'
    | 'SalesAccountsBilling'
    | 'SalesAccountsContracts'
    | 'SalesOrdersOverview'
    | 'SalesOrdersActivities'
    | 'SalesOrdersPayments'
    | 'SalesOrdersCommissions'
    | 'SalesSchedulingSchedule'
    | 'SalesSchedulingAppointments'
    | 'SalesSchedulingRecurring'
    | 'SalesSchedulingRoutes'
    | 'SalesSchedulingReminders'
    | 'UserHome';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type UserRole =
    | /** @default */ 'Administrator'
    | 'SalesRepresentative'
    | 'Technician'
    | 'HumanResources'
    | 'InformationTechnology';

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
export type OverviewDisplay = /** @default */ 'Card' | 'Table';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type IntervalUnit = /** @default */ 'Day' | 'Week' | 'Month' | 'Year';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Sector = /** @default */ 'Residential' | 'Commercial';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Weekday =
    | /** @default */ 'Monday'
    | 'Tuesday'
    | 'Wednesday'
    | 'Thursday'
    | 'Friday'
    | 'Saturday'
    | 'Sunday';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type NextStep = /** @default */ 'InitialContact' | 'Qualified' | 'Estimate' | 'Negotiation';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type LeadStage =
    | /** @default */ 'Open'
    | 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type AccountName = /** @default */ CompanyName | PersonName;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Priority = /** @default */ 'Medium' | 'High' | 'Low';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type ColorsConfig = Cardinal | Ordinal | Custom | /** @default */ Gradient;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type WeekOfMonth = /** @default */ 'First' | 'Second' | 'Third' | 'Fourth' | 'Last';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type ActivityType = /** @default */ Created | Edited | Sent | Viewed | Commented | Paid;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type RowHeight = 'ExtraSmall' | 'Small' | /** @default */ 'Medium' | 'Large';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Table =
    | /** @default */ 'Account'
    | 'Did'
    | 'Appointment'
    | 'Lead'
    | 'TaxRate'
    | 'Site'
    | 'Employee'
    | 'Route'
    | 'Company'
    | 'Product'
    | 'Service'
    | 'User'
    | 'Order'
    | 'Payment'
    | 'Package'
    | 'Promotion'
    | 'Represents'
    | 'Ordered';

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;

/** @derive(Default, Serialize, Deserialize) */
export type RecordLink<T> = /** @default */ string | T;

/** @derive(Default, Serialize, Deserialize, Gigaform) */
export type Actor = /** @default */ User | Employee | Account;
