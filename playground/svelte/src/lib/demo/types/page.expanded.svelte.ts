import { SerializeContext } from 'macroforge/serde';
import { Result } from 'macroforge/utils';
import { DeserializeContext } from 'macroforge/serde';
import { DeserializeError } from 'macroforge/serde';
import type { DeserializeOptions } from 'macroforge/serde';
import { PendingRef } from 'macroforge/serde';
import { Option } from 'macroforge/utils';
import type { FieldController } from '@playground/macro/gigaform';

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

export function defaultValuePage(): Page {
    return 'SalesHomeDashboard';
}

export function toStringifiedJSONPage(value: Page): string {
    const ctx = SerializeContext.create();
    return JSON.stringify(__serializePage(value, ctx));
}
export function toObjectPage(value: Page): unknown {
    const ctx = SerializeContext.create();
    return __serializePage(value, ctx);
}
export function __serializePage(value: Page, ctx: SerializeContext): unknown {
    if (typeof (value as any)?.__serialize === 'function') {
        return (value as any).__serialize(ctx);
    }
    return value;
}

export function fromStringifiedJSONPage(
    json: string,
    opts?: DeserializeOptions
): Result<Page, Array<{ field: string; message: string }>> {
    try {
        const raw = JSON.parse(json);
        return fromObjectPage(raw, opts);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function fromObjectPage(
    obj: unknown,
    opts?: DeserializeOptions
): Result<Page, Array<{ field: string; message: string }>> {
    try {
        const ctx = DeserializeContext.create();
        const resultOrRef = __deserializePage(obj, ctx);
        if (PendingRef.is(resultOrRef)) {
            return Result.err([
                { field: '_root', message: 'Page.fromObject: root cannot be a forward reference' }
            ]);
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return Result.ok(resultOrRef);
    } catch (e) {
        if (e instanceof DeserializeError) {
            return Result.err(e.errors);
        }
        const message = e instanceof Error ? e.message : String(e);
        return Result.err([{ field: '_root', message }]);
    }
}
export function __deserializePage(value: any, ctx: DeserializeContext): Page | PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Page | PendingRef;
    }
    const allowedValues = [
        'SalesHomeDashboard',
        'SalesHomeProducts',
        'SalesHomeServices',
        'SalesHomePackages',
        'SalesHomeTaxRates',
        'SalesLeadsOverview',
        'SalesLeadsActivities',
        'SalesLeadsCampaigns',
        'SalesLeadsDripCampaigns',
        'SalesLeadsOpportunities',
        'SalesLeadsPromotions',
        'SalesAccountsOverview',
        'SalesAccountsActivities',
        'SalesAccountsBilling',
        'SalesAccountsContracts',
        'SalesOrdersOverview',
        'SalesOrdersActivities',
        'SalesOrdersPayments',
        'SalesOrdersCommissions',
        'SalesSchedulingSchedule',
        'SalesSchedulingAppointments',
        'SalesSchedulingRecurring',
        'SalesSchedulingRoutes',
        'SalesSchedulingReminders',
        'UserHome'
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new DeserializeError([
            {
                field: '_root',
                message:
                    'Invalid value for Page: expected one of ' +
                    allowedValues.map((v) => JSON.stringify(v)).join(', ') +
                    ', got ' +
                    JSON.stringify(value)
            }
        ]);
    }
    return value as Page;
}
export function isPage(value: unknown): value is Page {
    const allowedValues = [
        'SalesHomeDashboard',
        'SalesHomeProducts',
        'SalesHomeServices',
        'SalesHomePackages',
        'SalesHomeTaxRates',
        'SalesLeadsOverview',
        'SalesLeadsActivities',
        'SalesLeadsCampaigns',
        'SalesLeadsDripCampaigns',
        'SalesLeadsOpportunities',
        'SalesLeadsPromotions',
        'SalesAccountsOverview',
        'SalesAccountsActivities',
        'SalesAccountsBilling',
        'SalesAccountsContracts',
        'SalesOrdersOverview',
        'SalesOrdersActivities',
        'SalesOrdersPayments',
        'SalesOrdersCommissions',
        'SalesSchedulingSchedule',
        'SalesSchedulingAppointments',
        'SalesSchedulingRecurring',
        'SalesSchedulingRoutes',
        'SalesSchedulingReminders',
        'UserHome'
    ] as const;
    return allowedValues.includes(value as any);
}

/** Per-variant error types */ export type SalesHomeDashboardErrorsPage = {
    _errors: Option<Array<string>>;
};
export type SalesHomeProductsErrorsPage = { _errors: Option<Array<string>> };
export type SalesHomeServicesErrorsPage = { _errors: Option<Array<string>> };
export type SalesHomePackagesErrorsPage = { _errors: Option<Array<string>> };
export type SalesHomeTaxRatesErrorsPage = { _errors: Option<Array<string>> };
export type SalesLeadsOverviewErrorsPage = { _errors: Option<Array<string>> };
export type SalesLeadsActivitiesErrorsPage = { _errors: Option<Array<string>> };
export type SalesLeadsCampaignsErrorsPage = { _errors: Option<Array<string>> };
export type SalesLeadsDripCampaignsErrorsPage = { _errors: Option<Array<string>> };
export type SalesLeadsOpportunitiesErrorsPage = { _errors: Option<Array<string>> };
export type SalesLeadsPromotionsErrorsPage = { _errors: Option<Array<string>> };
export type SalesAccountsOverviewErrorsPage = { _errors: Option<Array<string>> };
export type SalesAccountsActivitiesErrorsPage = { _errors: Option<Array<string>> };
export type SalesAccountsBillingErrorsPage = { _errors: Option<Array<string>> };
export type SalesAccountsContractsErrorsPage = { _errors: Option<Array<string>> };
export type SalesOrdersOverviewErrorsPage = { _errors: Option<Array<string>> };
export type SalesOrdersActivitiesErrorsPage = { _errors: Option<Array<string>> };
export type SalesOrdersPaymentsErrorsPage = { _errors: Option<Array<string>> };
export type SalesOrdersCommissionsErrorsPage = { _errors: Option<Array<string>> };
export type SalesSchedulingScheduleErrorsPage = { _errors: Option<Array<string>> };
export type SalesSchedulingAppointmentsErrorsPage = { _errors: Option<Array<string>> };
export type SalesSchedulingRecurringErrorsPage = { _errors: Option<Array<string>> };
export type SalesSchedulingRoutesErrorsPage = { _errors: Option<Array<string>> };
export type SalesSchedulingRemindersErrorsPage = { _errors: Option<Array<string>> };
export type UserHomeErrorsPage = {
    _errors: Option<Array<string>>;
}; /** Per-variant tainted types */
export type SalesHomeDashboardTaintedPage = {};
export type SalesHomeProductsTaintedPage = {};
export type SalesHomeServicesTaintedPage = {};
export type SalesHomePackagesTaintedPage = {};
export type SalesHomeTaxRatesTaintedPage = {};
export type SalesLeadsOverviewTaintedPage = {};
export type SalesLeadsActivitiesTaintedPage = {};
export type SalesLeadsCampaignsTaintedPage = {};
export type SalesLeadsDripCampaignsTaintedPage = {};
export type SalesLeadsOpportunitiesTaintedPage = {};
export type SalesLeadsPromotionsTaintedPage = {};
export type SalesAccountsOverviewTaintedPage = {};
export type SalesAccountsActivitiesTaintedPage = {};
export type SalesAccountsBillingTaintedPage = {};
export type SalesAccountsContractsTaintedPage = {};
export type SalesOrdersOverviewTaintedPage = {};
export type SalesOrdersActivitiesTaintedPage = {};
export type SalesOrdersPaymentsTaintedPage = {};
export type SalesOrdersCommissionsTaintedPage = {};
export type SalesSchedulingScheduleTaintedPage = {};
export type SalesSchedulingAppointmentsTaintedPage = {};
export type SalesSchedulingRecurringTaintedPage = {};
export type SalesSchedulingRoutesTaintedPage = {};
export type SalesSchedulingRemindersTaintedPage = {};
export type UserHomeTaintedPage = {}; /** Union error type */
export type ErrorsPage =
    | ({ _value: 'SalesHomeDashboard' } & SalesHomeDashboardErrorsPage)
    | ({ _value: 'SalesHomeProducts' } & SalesHomeProductsErrorsPage)
    | ({ _value: 'SalesHomeServices' } & SalesHomeServicesErrorsPage)
    | ({ _value: 'SalesHomePackages' } & SalesHomePackagesErrorsPage)
    | ({ _value: 'SalesHomeTaxRates' } & SalesHomeTaxRatesErrorsPage)
    | ({ _value: 'SalesLeadsOverview' } & SalesLeadsOverviewErrorsPage)
    | ({ _value: 'SalesLeadsActivities' } & SalesLeadsActivitiesErrorsPage)
    | ({ _value: 'SalesLeadsCampaigns' } & SalesLeadsCampaignsErrorsPage)
    | ({ _value: 'SalesLeadsDripCampaigns' } & SalesLeadsDripCampaignsErrorsPage)
    | ({ _value: 'SalesLeadsOpportunities' } & SalesLeadsOpportunitiesErrorsPage)
    | ({ _value: 'SalesLeadsPromotions' } & SalesLeadsPromotionsErrorsPage)
    | ({ _value: 'SalesAccountsOverview' } & SalesAccountsOverviewErrorsPage)
    | ({ _value: 'SalesAccountsActivities' } & SalesAccountsActivitiesErrorsPage)
    | ({ _value: 'SalesAccountsBilling' } & SalesAccountsBillingErrorsPage)
    | ({ _value: 'SalesAccountsContracts' } & SalesAccountsContractsErrorsPage)
    | ({ _value: 'SalesOrdersOverview' } & SalesOrdersOverviewErrorsPage)
    | ({ _value: 'SalesOrdersActivities' } & SalesOrdersActivitiesErrorsPage)
    | ({ _value: 'SalesOrdersPayments' } & SalesOrdersPaymentsErrorsPage)
    | ({ _value: 'SalesOrdersCommissions' } & SalesOrdersCommissionsErrorsPage)
    | ({ _value: 'SalesSchedulingSchedule' } & SalesSchedulingScheduleErrorsPage)
    | ({ _value: 'SalesSchedulingAppointments' } & SalesSchedulingAppointmentsErrorsPage)
    | ({ _value: 'SalesSchedulingRecurring' } & SalesSchedulingRecurringErrorsPage)
    | ({ _value: 'SalesSchedulingRoutes' } & SalesSchedulingRoutesErrorsPage)
    | ({ _value: 'SalesSchedulingReminders' } & SalesSchedulingRemindersErrorsPage)
    | ({ _value: 'UserHome' } & UserHomeErrorsPage); /** Union tainted type */
export type TaintedPage =
    | ({ _value: 'SalesHomeDashboard' } & SalesHomeDashboardTaintedPage)
    | ({ _value: 'SalesHomeProducts' } & SalesHomeProductsTaintedPage)
    | ({ _value: 'SalesHomeServices' } & SalesHomeServicesTaintedPage)
    | ({ _value: 'SalesHomePackages' } & SalesHomePackagesTaintedPage)
    | ({ _value: 'SalesHomeTaxRates' } & SalesHomeTaxRatesTaintedPage)
    | ({ _value: 'SalesLeadsOverview' } & SalesLeadsOverviewTaintedPage)
    | ({ _value: 'SalesLeadsActivities' } & SalesLeadsActivitiesTaintedPage)
    | ({ _value: 'SalesLeadsCampaigns' } & SalesLeadsCampaignsTaintedPage)
    | ({ _value: 'SalesLeadsDripCampaigns' } & SalesLeadsDripCampaignsTaintedPage)
    | ({ _value: 'SalesLeadsOpportunities' } & SalesLeadsOpportunitiesTaintedPage)
    | ({ _value: 'SalesLeadsPromotions' } & SalesLeadsPromotionsTaintedPage)
    | ({ _value: 'SalesAccountsOverview' } & SalesAccountsOverviewTaintedPage)
    | ({ _value: 'SalesAccountsActivities' } & SalesAccountsActivitiesTaintedPage)
    | ({ _value: 'SalesAccountsBilling' } & SalesAccountsBillingTaintedPage)
    | ({ _value: 'SalesAccountsContracts' } & SalesAccountsContractsTaintedPage)
    | ({ _value: 'SalesOrdersOverview' } & SalesOrdersOverviewTaintedPage)
    | ({ _value: 'SalesOrdersActivities' } & SalesOrdersActivitiesTaintedPage)
    | ({ _value: 'SalesOrdersPayments' } & SalesOrdersPaymentsTaintedPage)
    | ({ _value: 'SalesOrdersCommissions' } & SalesOrdersCommissionsTaintedPage)
    | ({ _value: 'SalesSchedulingSchedule' } & SalesSchedulingScheduleTaintedPage)
    | ({ _value: 'SalesSchedulingAppointments' } & SalesSchedulingAppointmentsTaintedPage)
    | ({ _value: 'SalesSchedulingRecurring' } & SalesSchedulingRecurringTaintedPage)
    | ({ _value: 'SalesSchedulingRoutes' } & SalesSchedulingRoutesTaintedPage)
    | ({ _value: 'SalesSchedulingReminders' } & SalesSchedulingRemindersTaintedPage)
    | ({ _value: 'UserHome' } & UserHomeTaintedPage); /** Per-variant field controller types */
export interface SalesHomeDashboardFieldControllersPage {}
export interface SalesHomeProductsFieldControllersPage {}
export interface SalesHomeServicesFieldControllersPage {}
export interface SalesHomePackagesFieldControllersPage {}
export interface SalesHomeTaxRatesFieldControllersPage {}
export interface SalesLeadsOverviewFieldControllersPage {}
export interface SalesLeadsActivitiesFieldControllersPage {}
export interface SalesLeadsCampaignsFieldControllersPage {}
export interface SalesLeadsDripCampaignsFieldControllersPage {}
export interface SalesLeadsOpportunitiesFieldControllersPage {}
export interface SalesLeadsPromotionsFieldControllersPage {}
export interface SalesAccountsOverviewFieldControllersPage {}
export interface SalesAccountsActivitiesFieldControllersPage {}
export interface SalesAccountsBillingFieldControllersPage {}
export interface SalesAccountsContractsFieldControllersPage {}
export interface SalesOrdersOverviewFieldControllersPage {}
export interface SalesOrdersActivitiesFieldControllersPage {}
export interface SalesOrdersPaymentsFieldControllersPage {}
export interface SalesOrdersCommissionsFieldControllersPage {}
export interface SalesSchedulingScheduleFieldControllersPage {}
export interface SalesSchedulingAppointmentsFieldControllersPage {}
export interface SalesSchedulingRecurringFieldControllersPage {}
export interface SalesSchedulingRoutesFieldControllersPage {}
export interface SalesSchedulingRemindersFieldControllersPage {}
export interface UserHomeFieldControllersPage {} /** Union Gigaform interface with variant switching */
export interface GigaformPage {
    readonly currentVariant:
        | 'SalesHomeDashboard'
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
    readonly data: Page;
    readonly errors: ErrorsPage;
    readonly tainted: TaintedPage;
    readonly variants: VariantFieldsPage;
    switchVariant(
        variant:
            | 'SalesHomeDashboard'
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
            | 'UserHome'
    ): void;
    validate(): Result<Page, Array<{ field: string; message: string }>>;
    reset(overrides?: Partial<Page>): void;
} /** Variant fields container */
export interface VariantFieldsPage {
    readonly SalesHomeDashboard: { readonly fields: SalesHomeDashboardFieldControllersPage };
    readonly SalesHomeProducts: { readonly fields: SalesHomeProductsFieldControllersPage };
    readonly SalesHomeServices: { readonly fields: SalesHomeServicesFieldControllersPage };
    readonly SalesHomePackages: { readonly fields: SalesHomePackagesFieldControllersPage };
    readonly SalesHomeTaxRates: { readonly fields: SalesHomeTaxRatesFieldControllersPage };
    readonly SalesLeadsOverview: { readonly fields: SalesLeadsOverviewFieldControllersPage };
    readonly SalesLeadsActivities: { readonly fields: SalesLeadsActivitiesFieldControllersPage };
    readonly SalesLeadsCampaigns: { readonly fields: SalesLeadsCampaignsFieldControllersPage };
    readonly SalesLeadsDripCampaigns: {
        readonly fields: SalesLeadsDripCampaignsFieldControllersPage;
    };
    readonly SalesLeadsOpportunities: {
        readonly fields: SalesLeadsOpportunitiesFieldControllersPage;
    };
    readonly SalesLeadsPromotions: { readonly fields: SalesLeadsPromotionsFieldControllersPage };
    readonly SalesAccountsOverview: { readonly fields: SalesAccountsOverviewFieldControllersPage };
    readonly SalesAccountsActivities: {
        readonly fields: SalesAccountsActivitiesFieldControllersPage;
    };
    readonly SalesAccountsBilling: { readonly fields: SalesAccountsBillingFieldControllersPage };
    readonly SalesAccountsContracts: {
        readonly fields: SalesAccountsContractsFieldControllersPage;
    };
    readonly SalesOrdersOverview: { readonly fields: SalesOrdersOverviewFieldControllersPage };
    readonly SalesOrdersActivities: { readonly fields: SalesOrdersActivitiesFieldControllersPage };
    readonly SalesOrdersPayments: { readonly fields: SalesOrdersPaymentsFieldControllersPage };
    readonly SalesOrdersCommissions: {
        readonly fields: SalesOrdersCommissionsFieldControllersPage;
    };
    readonly SalesSchedulingSchedule: {
        readonly fields: SalesSchedulingScheduleFieldControllersPage;
    };
    readonly SalesSchedulingAppointments: {
        readonly fields: SalesSchedulingAppointmentsFieldControllersPage;
    };
    readonly SalesSchedulingRecurring: {
        readonly fields: SalesSchedulingRecurringFieldControllersPage;
    };
    readonly SalesSchedulingRoutes: { readonly fields: SalesSchedulingRoutesFieldControllersPage };
    readonly SalesSchedulingReminders: {
        readonly fields: SalesSchedulingRemindersFieldControllersPage;
    };
    readonly UserHome: { readonly fields: UserHomeFieldControllersPage };
} /** Gets default value for a specific variant */
function getDefaultForVariantPage(variant: string): Page {
    switch (variant) {
        case 'SalesHomeDashboard':
            return 'SalesHomeDashboard' as Page;
        case 'SalesHomeProducts':
            return 'SalesHomeProducts' as Page;
        case 'SalesHomeServices':
            return 'SalesHomeServices' as Page;
        case 'SalesHomePackages':
            return 'SalesHomePackages' as Page;
        case 'SalesHomeTaxRates':
            return 'SalesHomeTaxRates' as Page;
        case 'SalesLeadsOverview':
            return 'SalesLeadsOverview' as Page;
        case 'SalesLeadsActivities':
            return 'SalesLeadsActivities' as Page;
        case 'SalesLeadsCampaigns':
            return 'SalesLeadsCampaigns' as Page;
        case 'SalesLeadsDripCampaigns':
            return 'SalesLeadsDripCampaigns' as Page;
        case 'SalesLeadsOpportunities':
            return 'SalesLeadsOpportunities' as Page;
        case 'SalesLeadsPromotions':
            return 'SalesLeadsPromotions' as Page;
        case 'SalesAccountsOverview':
            return 'SalesAccountsOverview' as Page;
        case 'SalesAccountsActivities':
            return 'SalesAccountsActivities' as Page;
        case 'SalesAccountsBilling':
            return 'SalesAccountsBilling' as Page;
        case 'SalesAccountsContracts':
            return 'SalesAccountsContracts' as Page;
        case 'SalesOrdersOverview':
            return 'SalesOrdersOverview' as Page;
        case 'SalesOrdersActivities':
            return 'SalesOrdersActivities' as Page;
        case 'SalesOrdersPayments':
            return 'SalesOrdersPayments' as Page;
        case 'SalesOrdersCommissions':
            return 'SalesOrdersCommissions' as Page;
        case 'SalesSchedulingSchedule':
            return 'SalesSchedulingSchedule' as Page;
        case 'SalesSchedulingAppointments':
            return 'SalesSchedulingAppointments' as Page;
        case 'SalesSchedulingRecurring':
            return 'SalesSchedulingRecurring' as Page;
        case 'SalesSchedulingRoutes':
            return 'SalesSchedulingRoutes' as Page;
        case 'SalesSchedulingReminders':
            return 'SalesSchedulingReminders' as Page;
        case 'UserHome':
            return 'UserHome' as Page;
        default:
            return 'SalesHomeDashboard' as Page;
    }
} /** Creates a new discriminated union Gigaform with variant switching */
export function createFormPage(initial?: Page): GigaformPage {
    const initialVariant:
        | 'SalesHomeDashboard'
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
        | 'UserHome' =
        (initial as
            | 'SalesHomeDashboard'
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
            | 'UserHome') ?? 'SalesHomeDashboard';
    let currentVariant = $state<
        | 'SalesHomeDashboard'
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
        | 'UserHome'
    >(initialVariant);
    let data = $state<Page>(initial ?? getDefaultForVariantPage(initialVariant));
    let errors = $state<ErrorsPage>({} as ErrorsPage);
    let tainted = $state<TaintedPage>({} as TaintedPage);
    const variants: VariantFieldsPage = {
        SalesHomeDashboard: {
            fields: {} as SalesHomeDashboardFieldControllersPage
        },
        SalesHomeProducts: {
            fields: {} as SalesHomeProductsFieldControllersPage
        },
        SalesHomeServices: {
            fields: {} as SalesHomeServicesFieldControllersPage
        },
        SalesHomePackages: {
            fields: {} as SalesHomePackagesFieldControllersPage
        },
        SalesHomeTaxRates: {
            fields: {} as SalesHomeTaxRatesFieldControllersPage
        },
        SalesLeadsOverview: {
            fields: {} as SalesLeadsOverviewFieldControllersPage
        },
        SalesLeadsActivities: {
            fields: {} as SalesLeadsActivitiesFieldControllersPage
        },
        SalesLeadsCampaigns: {
            fields: {} as SalesLeadsCampaignsFieldControllersPage
        },
        SalesLeadsDripCampaigns: {
            fields: {} as SalesLeadsDripCampaignsFieldControllersPage
        },
        SalesLeadsOpportunities: {
            fields: {} as SalesLeadsOpportunitiesFieldControllersPage
        },
        SalesLeadsPromotions: {
            fields: {} as SalesLeadsPromotionsFieldControllersPage
        },
        SalesAccountsOverview: {
            fields: {} as SalesAccountsOverviewFieldControllersPage
        },
        SalesAccountsActivities: {
            fields: {} as SalesAccountsActivitiesFieldControllersPage
        },
        SalesAccountsBilling: {
            fields: {} as SalesAccountsBillingFieldControllersPage
        },
        SalesAccountsContracts: {
            fields: {} as SalesAccountsContractsFieldControllersPage
        },
        SalesOrdersOverview: {
            fields: {} as SalesOrdersOverviewFieldControllersPage
        },
        SalesOrdersActivities: {
            fields: {} as SalesOrdersActivitiesFieldControllersPage
        },
        SalesOrdersPayments: {
            fields: {} as SalesOrdersPaymentsFieldControllersPage
        },
        SalesOrdersCommissions: {
            fields: {} as SalesOrdersCommissionsFieldControllersPage
        },
        SalesSchedulingSchedule: {
            fields: {} as SalesSchedulingScheduleFieldControllersPage
        },
        SalesSchedulingAppointments: {
            fields: {} as SalesSchedulingAppointmentsFieldControllersPage
        },
        SalesSchedulingRecurring: {
            fields: {} as SalesSchedulingRecurringFieldControllersPage
        },
        SalesSchedulingRoutes: {
            fields: {} as SalesSchedulingRoutesFieldControllersPage
        },
        SalesSchedulingReminders: {
            fields: {} as SalesSchedulingRemindersFieldControllersPage
        },
        UserHome: {
            fields: {} as UserHomeFieldControllersPage
        }
    };
    function switchVariant(
        variant:
            | 'SalesHomeDashboard'
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
            | 'UserHome'
    ): void {
        currentVariant = variant;
        data = getDefaultForVariantPage(variant);
        errors = {} as ErrorsPage;
        tainted = {} as TaintedPage;
    }
    function validate(): Result<Page, Array<{ field: string; message: string }>> {
        return fromObjectPage(data);
    }
    function reset(overrides?: Partial<Page>): void {
        data = overrides ? (overrides as typeof data) : getDefaultForVariantPage(currentVariant);
        errors = {} as ErrorsPage;
        tainted = {} as TaintedPage;
    }
    return {
        get currentVariant() {
            return currentVariant;
        },
        get data() {
            return data;
        },
        set data(v) {
            data = v;
        },
        get errors() {
            return errors;
        },
        set errors(v) {
            errors = v;
        },
        get tainted() {
            return tainted;
        },
        set tainted(v) {
            tainted = v;
        },
        variants,
        switchVariant,
        validate,
        reset
    };
} /** Parses FormData for union type, determining variant from discriminant field */
export function fromFormDataPage(
    formData: FormData
): Result<Page, Array<{ field: string; message: string }>> {
    const discriminant = formData.get('_value') as
        | 'SalesHomeDashboard'
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
        | 'UserHome'
        | null;
    if (!discriminant) {
        return Result.err([{ field: '_value', message: 'Missing discriminant field' }]);
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    if (discriminant === 'SalesHomeDashboard') {
    } else if (discriminant === 'SalesHomeProducts') {
    } else if (discriminant === 'SalesHomeServices') {
    } else if (discriminant === 'SalesHomePackages') {
    } else if (discriminant === 'SalesHomeTaxRates') {
    } else if (discriminant === 'SalesLeadsOverview') {
    } else if (discriminant === 'SalesLeadsActivities') {
    } else if (discriminant === 'SalesLeadsCampaigns') {
    } else if (discriminant === 'SalesLeadsDripCampaigns') {
    } else if (discriminant === 'SalesLeadsOpportunities') {
    } else if (discriminant === 'SalesLeadsPromotions') {
    } else if (discriminant === 'SalesAccountsOverview') {
    } else if (discriminant === 'SalesAccountsActivities') {
    } else if (discriminant === 'SalesAccountsBilling') {
    } else if (discriminant === 'SalesAccountsContracts') {
    } else if (discriminant === 'SalesOrdersOverview') {
    } else if (discriminant === 'SalesOrdersActivities') {
    } else if (discriminant === 'SalesOrdersPayments') {
    } else if (discriminant === 'SalesOrdersCommissions') {
    } else if (discriminant === 'SalesSchedulingSchedule') {
    } else if (discriminant === 'SalesSchedulingAppointments') {
    } else if (discriminant === 'SalesSchedulingRecurring') {
    } else if (discriminant === 'SalesSchedulingRoutes') {
    } else if (discriminant === 'SalesSchedulingReminders') {
    } else if (discriminant === 'UserHome') {
    }
    return fromStringifiedJSONPage(JSON.stringify(obj));
}
