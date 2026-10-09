/** import macro {Gigaform} from "@testground/macro"; */

/** @derive(Default, Encode, Decode, Gigaform) */
export interface User {
    id: string;
    email: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @endec({ validate: ["nonEmpty"] }) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Service {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Quick Code" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export interface ServiceDefaults {
    /** @numberController({ label: "Price", min: 0, step: 0.01 }) */
    price: number;
    /** @textAreaController({ label: "Description" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    description: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export interface PersonName {
    /** @textController({ label: "First Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    firstName: string;
    /** @textController({ label: "Last Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    lastName: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Promotion {
    id: string;
    date: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Site {
    id: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    addressLine1: string;
    addressLine2: string | null;
    sublocalityLevel1: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
    locality: string;
    administrativeAreaLevel3: string | null;
    administrativeAreaLevel2: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
    administrativeAreaLevel1: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    country: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    postalCode: string;
    postalCodeSuffix: string | null;
    coordinates: Coordinates;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Metadata {
    createdAt: string;
    lastLogin: string | null;
    isActive: boolean;
    roles: Array<string>;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface ColumnConfig {
    /** @endec({ validate: ["nonEmpty"] }) */
    heading: string;
    dataPath: DataPath;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface PhoneNumber {
    /** @switchController({ label: "Main" }) */
    main: boolean;
    /** @comboboxController({ label: "Phone Type", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    phoneType: string;
    /** @textController({ label: "Number" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    number: string;
    /** @switchController({ label: "Can Text" }) */
    canText: boolean;
    /** @switchController({ label: "Can Call" }) */
    canCall: boolean;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Gradient {
    startHue: number;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Product {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Quick Code" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export interface YearlyRecurrenceRule {
    quantityOfYears: number;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface AppointmentNotifications {
    /** @endec({ validate: ["nonEmpty"] }) */
    personalScheduleChangeNotifications: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    allScheduleChangeNotifications: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface DirectionHue {
    bearing: number;
    hue: number;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface MonthlyRecurrenceRule {
    quantityOfMonths: number;
    day: number;
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Represents {
    /** @default("") */
    in: string | Employee;
    /** @default("") */
    out: string | Account;
    id: string;
    dateStarted: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Payment {
    id: string;
    date: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Color {
    red: number;
    green: number;
    blue: number;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface CompanyName {
    /** @textController({ label: "Company Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    companyName: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Appointment {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Title" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
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
    employees: Array<string | Employee>;
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

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Package {
    /** @hiddenController({}) */
    id: string;
    /** @dateTimeController({ label: "Date" }) */
    date: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface ScheduleSettings {
    daysPerWeek: number;
    /** @default("Medium") */
    rowHeight: RowHeight;
    visibleRoutes: Array<string>;
    detailedCards: boolean;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface DailyRecurrenceRule {
    quantityOfDays: number;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface SignUpCredentials {
    firstName: FirstName;
    lastName: LastName;
    email: EmailParts;
    password: Password;
    rememberMe: boolean;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface OverviewSettings {
    /** @default("Medium") */
    rowHeight: RowHeight;
    /** @default("Table") */
    cardOrRow: OverviewDisplay;
    perPage: number;
    columnConfigs: Array<ColumnConfig>;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface FirstName {
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Account {
    /** @hiddenController({}) */
    id: string;
    /** @comboboxController({ label: "Tax Rate", allowCustom: false, fetchUrls: ["/api/tax_rates"] }) */
    /** @default("") */
    taxRate: string | TaxRate;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @comboboxMultipleController({ label: "Sales Rep", fetchUrls: ["/api/employees"] }) */
    salesRep: Array<Represents> | null;
    /** @hiddenController({}) */
    orders: Array<Ordered>;
    /** @hiddenController({}) */
    activity: Array<Did>;
    /** @arrayFieldsetController({ legend: "Custom Fields" }) */
    customFields: Array<[string, string]>;
    /** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
    accountName: AccountName;
    /** @radioGroupController({ label: "Sector", options: [{ label: "Residential", value: "Residential" }, { label: "Commercial", value: "Commercial" }], orientation: "horizontal" }) */
    /** @default("Residential") */
    sector: Sector;
    /** @textAreaController({ label: "Memo" }) */
    memo: string | null;
    /** @arrayFieldsetController({ legend: "Phones" }) */
    phones: Array<PhoneNumber>;
    /** @emailFieldController({ label: "Email" }) */
    email: Email;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    leadSource: string;
    /** @hiddenController({}) */
    colors: Colors;
    /** @toggleController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @toggleController({ label: "Has Alert" }) */
    hasAlert: boolean;
    /** @comboboxController({ label: "Account Type", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    accountType: string;
    /** @comboboxController({ label: "Subtype", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    subtype: string;
    /** @toggleController({ label: "Tax Exempt" }) */
    isTaxExempt: boolean;
    /** @comboboxController({ label: "Payment Terms", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    paymentTerms: string;
    /** @tagsController({ label: "Tags" }) */
    tags: Array<string>;
    /** @hiddenController({}) */
    dateAdded: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Edited {
    /** @endec({ validate: ["nonEmpty"] }) */
    fieldName: string;
    oldValue: string | null;
    newValue: string | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
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
    payments: Array<string | Payment>;
    /** @textController({ label: "Opportunity" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    opportunity: string;
    /** @textController({ label: "Reference" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    reference: string;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    leadSource: string;
    /** @comboboxController({ label: "Sales Rep", allowCustom: false, fetchUrls: ["/api/employees"] }) */
    /** @default("") */
    salesRep: string | Employee;
    /** @comboboxController({ label: "Group", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    group: string;
    /** @comboboxController({ label: "Subgroup", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    subgroup: string;
    /** @switchController({ label: "Posted" }) */
    isPosted: boolean;
    /** @switchController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @textController({ label: "Action Item" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    actionItem: string;
    /** @hiddenController({}) */
    upsale: number;
    /** @hiddenController({}) */
    dateCreated: string;
    /** @comboboxController({ label: "Appointment", allowCustom: false, fetchUrls: ["/api/appointments"] }) */
    /** @default("") */
    appointment: string | Appointment;
    /** @comboboxMultipleController({ label: "Technicians", fetchUrls: ["/api/employees"] }) */
    lastTechs: Array<string | Employee>;
    /** @hiddenController({}) */
    package: Array<string | Package> | null;
    /** @hiddenController({}) */
    promotion: Array<string | Promotion> | null;
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
    billedItems: Array<BilledItem>;
    /** @textAreaController({ label: "Memo" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    memo: string;
    /** @hiddenController({}) */
    discount: number;
    /** @hiddenController({}) */
    tip: number;
    /** @hiddenController({}) */
    commissions: Array<number>;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Commented {
    /** @endec({ validate: ["nonEmpty"] }) */
    comment: string;
    replyTo: string | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Custom {
    mappings: Array<DirectionHue>;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Colors {
    /** @endec({ validate: ["nonEmpty"] }) */
    main: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    hover: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    active: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface ProductDefaults {
    /** @numberController({ label: "Price", min: 0, step: 0.01 }) */
    price: number;
    /** @textAreaController({ label: "Description" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    description: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Viewed {
    durationSeconds: number | null;
    source: string | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface WeeklyRecurrenceRule {
    quantityOfWeeks: number;
    weekdays: Array<Weekday>;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Paid {
    amount: number | null;
    currency: string | null;
    paymentMethod: string | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface TaxRate {
    /** @hiddenController({}) */
    id: string;
    /** @textController({ label: "Name" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    /** @textController({ label: "Tax Agency" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    taxAgency: string;
    /** @numberController({ label: "Zip", min: 0 }) */
    zip: number;
    /** @textController({ label: "City" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    city: string;
    /** @textController({ label: "County" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    county: string;
    /** @textController({ label: "State" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    state: string;
    /** @switchController({ label: "Active" }) */
    isActive: boolean;
    /** @textAreaController({ label: "Description" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    description: string;
    /** @hiddenController({}) */
    /** @default({}) */
    taxComponents: { [key: string]: number };
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Address {
    /** @endec({ validate: ["nonEmpty"] }) */
    street: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    city: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    state: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    zipcode: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
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
    /** @endec({ validate: ["nonEmpty"] }) */
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
    /** @comboboxController({ label: "Tax Rate", allowCustom: false, fetchUrls: ["/api/tax_rates"] }) */
    taxRate: (string | TaxRate) | null;
    /** @radioGroupController({ label: "Sector", options: [{ label: "Residential", value: "Residential" }, { label: "Commercial", value: "Commercial" }], orientation: "horizontal" }) */
    /** @default("Residential") */
    sector: Sector;
    /** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
    leadName: AccountName;
    /** @arrayFieldsetController({ legend: "Phones" }) */
    phones: Array<PhoneNumber>;
    /** @emailFieldController({ label: "Email" }) */
    email: Email;
    /** @comboboxController({ label: "Lead Source", allowCustom: true }) */
    leadSource: string | null;
    /** @siteFieldsetController({ label: "Site" }) */
    /** @default("") */
    site: string | Site;
    /** @textAreaController({ label: "Memo" }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    memo: string;
    /** @toggleController({ label: "Needs Review" }) */
    needsReview: boolean;
    /** @toggleController({ label: "Has Alert" }) */
    hasAlert: boolean;
    /** @comboboxMultipleController({ label: "Sales Rep", fetchUrls: ["/api/employees"] }) */
    salesRep: Array<Represents> | null;
    /** @hiddenController({}) */
    color: string | null;
    /** @comboboxController({ label: "Account Type", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    accountType: string;
    /** @comboboxController({ label: "Subtype", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    subtype: string;
    /** @toggleController({ label: "Tax Exempt" }) */
    isTaxExempt: boolean;
    /** @comboboxController({ label: "Payment Terms", allowCustom: true }) */
    /** @endec({ validate: ["nonEmpty"] }) */
    paymentTerms: string;
    /** @tagsController({ label: "Tags" }) */
    tags: Array<string>;
    /** @arrayFieldsetController({ legend: "Custom Fields" }) */
    customFields: Array<[string, string]>;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface AppPermissions {
    applications: Array<Applications>;
    pages: Array<Page>;
    data: Array<Table>;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Company {
    id: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    legalName: string;
    /** @default("") */
    headquarters: string | Site;
    phones: Array<PhoneNumber>;
    /** @endec({ validate: ["nonEmpty"] }) */
    fax: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    email: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    website: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    taxId: string;
    referenceNumber: number;
    /** @endec({ validate: ["nonEmpty"] }) */
    postalCodeLookup: string;
    timeZone: string;
    /** @default("") */
    defaultTax: string | TaxRate;
    /** @endec({ validate: ["nonEmpty"] }) */
    defaultTaxLocation: string;
    defaultAreaCode: number;
    /** @endec({ validate: ["nonEmpty"] }) */
    defaultAccountType: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    lookupFormatting: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    accountNameFormat: string;
    merchantServiceProvider: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Password {
    /** @endec({ validate: ["nonEmpty"] }) */
    password: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Created {
    initialData: string | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Employee {
    id: string;
    imageUrl: string | null;
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    phones: Array<PhoneNumber>;
    /** @endec({ validate: ["nonEmpty"] }) */
    role: string;
    /** @default("Technician") */
    title: JobTitle;
    email: Email;
    /** @endec({ validate: ["nonEmpty"] }) */
    address: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    username: string;
    /** @default("") */
    route: string | Route;
    ratePerHour: number;
    active: boolean;
    isTechnician: boolean;
    isSalesRep: boolean;
    description: string | null;
    linkedinUrl: string | null;
    attendance: Array<string>;
    settings: Settings;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Commissions {
    /** @endec({ validate: ["nonEmpty"] }) */
    technician: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    salesRep: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Number {
    /** @endec({ validate: ["nonEmpty"] }) */
    countryCode: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    areaCode: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    localNumber: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface DataPath {
    path: Array<string>;
    formatter: string | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Route {
    id: string;
    techs: Array<string | Employee> | null;
    active: boolean;
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    phone: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    position: string;
    serviceRoute: boolean;
    defaultDurationHours: number;
    tags: Array<string>;
    icon: string | null;
    color: string | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface EmailParts {
    /** @endec({ validate: ["nonEmpty"] }) */
    local: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    domainName: string;
    /** @endec({ validate: ["nonEmpty"] }) */
    topLevelDomain: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Sent {
    recipient: string | null;
    method: string | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Coordinates {
    lat: number;
    lng: number;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Ordered {
    id: string;
    /** @default("") */
    in: string | Account;
    /** @default("") */
    out: string | Order;
    date: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Email {
    /** @switchController({ label: "Can Email" }) */
    canEmail: boolean;
    /** @textController({ label: "Email" }) */
    /** @endec({ validate: ["nonEmpty", "email"] }) */
    emailString: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface RecurrenceRule {
    interval: Interval;
    recurrenceBegins: string;
    recurrenceEnds: RecurrenceEnd | null;
    cancelledInstances: Array<string> | null;
    additionalInstances: Array<string> | null;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface LastName {
    /** @endec({ validate: ["nonEmpty"] }) */
    name: string;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export interface Cardinal {
    north: number;
    east: number;
    south: number;
    west: number;
}

/** @derive(Default, Encode, Decode, Gigaform) */
export type Interval =
    | /** @default */ DailyRecurrenceRule
    | WeeklyRecurrenceRule
    | MonthlyRecurrenceRule
    | YearlyRecurrenceRule;

/** @derive(Default, Encode, Decode, Gigaform) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export type UserRole =
    | /** @default */ 'Administrator'
    | 'SalesRepresentative'
    | 'Technician'
    | 'HumanResources'
    | 'InformationTechnology';

/** @derive(Default, Encode, Decode, Gigaform) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export type RecurrenceEnd = /** @default(0) */ number | string;

/** @derive(Default, Encode, Decode, Gigaform) */
export type OverviewDisplay = /** @default */ 'Card' | 'Table';

/** @derive(Default, Encode, Decode, Gigaform) */
export type IntervalUnit = /** @default */ 'Day' | 'Week' | 'Month' | 'Year';

/** @derive(Default, Encode, Decode, Gigaform) */
export type Sector = /** @default */ 'Residential' | 'Commercial';

/** @derive(Default, Encode, Decode, Gigaform) */
export type Weekday =
    | /** @default */ 'Monday'
    | 'Tuesday'
    | 'Wednesday'
    | 'Thursday'
    | 'Friday'
    | 'Saturday'
    | 'Sunday';

/** @derive(Default, Encode, Decode, Gigaform) */
export type Status = /** @default */ 'Scheduled' | 'OnDeck' | 'Waiting';

/** @derive(Default, Encode, Decode, Gigaform) */
export type NextStep =
    | /** @default */ 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';

/** @derive(Default, Encode, Decode, Gigaform) */
export type LeadStage =
    | /** @default */ 'Open'
    | 'InitialContact'
    | 'Qualified'
    | 'Estimate'
    | 'Negotiation';

/** @derive(Default, Encode, Decode, Gigaform) */
/** @enumFieldsetController({ legend: "Name", variants: { CompanyName: { label: "Company" }, PersonName: { label: "Person" } } }) */
export type AccountName = /** @default */ CompanyName | PersonName;

/** @derive(Default, Encode, Decode, Gigaform) */
export type Priority = /** @default */ 'Medium' | 'High' | 'Low';

/** @derive(Default, Encode, Decode, Gigaform) */
export type Applications =
    | /** @default */ 'Sales'
    | 'Accounting'
    | 'Errand'
    | 'HumanResources'
    | 'Logistics'
    | 'Marketing'
    | 'Website';

/** @derive(Default, Encode, Decode, Gigaform) */
export type JobTitle =
    | /** @default */ 'Technician'
    | 'SalesRepresentative'
    | 'HumanResources'
    | 'InformationTechnology';

/** @derive(Default, Encode, Decode, Gigaform) */
export type ColorsConfig =
    | Cardinal
    | Ordinal
    | Custom
    | /** @default */ Gradient;

/** @derive(Default, Encode, Decode, Gigaform) */
export type WeekOfMonth =
    | /** @default */ 'First'
    | 'Second'
    | 'Third'
    | 'Fourth'
    | 'Last';

/** @derive(Default, Encode, Decode, Gigaform) */
export type ActivityType =
    | /** @default */ Created
    | Edited
    | Sent
    | Viewed
    | Commented
    | Paid;

/** @derive(Default, Encode, Decode, Gigaform) */
export type RowHeight =
    | 'ExtraSmall'
    | 'Small'
    | /** @default */ 'Medium'
    | 'Large';

/** @derive(Default, Encode, Decode, Gigaform) */
export type OrderStage = /** @default */ 'Estimate' | 'Active' | 'Invoice';

/** @derive(Default, Encode, Decode, Gigaform) */
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

/** @derive(Default, Encode, Decode, Gigaform) */
export type Item = RecordLink<Product> | /** @default */ RecordLink<Service>;

/** @derive(Default, Encode, Decode) */
export type RecordLink<T> = /** @default */ string | T;

/** @derive(Default, Encode, Decode, Gigaform) */
export type Actor = /** @default */ User | Employee | Account;
