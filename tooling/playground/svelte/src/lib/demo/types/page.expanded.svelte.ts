import { SerializeContext as __mf_SerializeContext } from "macroforge/serde";
import { DeserializeContext as __mf_DeserializeContext } from "macroforge/serde";
import { DeserializeError as __mf_DeserializeError } from "macroforge/serde";
import type { DeserializeOptions as __mf_DeserializeOptions } from "macroforge/serde";
import { PendingRef as __mf_PendingRef } from "macroforge/serde";
import type { Exit } from "@playground/macro/gigaform";
import { toExit } from "@playground/macro/gigaform";
import type { Option as __gf_Option } from "@playground/macro/gigaform";
import { optionNone } from "@playground/macro/gigaform";
import type { FieldController } from "@playground/macro/gigaform";

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

export function pageDefaultValue#0#0(): Page {
    return 'SalesHomeDashboard';
}

export function pageSerialize(value: Page): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(pageSerializeWithContext(value, ctx));
}
export function pageSerializeWithContext(value: Page, ctx: __mf_SerializeContext): unknown {
    if (typeof (value as any)?.serializeWithContext === "function") {
        return (value as any).serializeWithContext(ctx);
    }
    return value;
}

export function pageDeserialize(input: unknown, opts: __mf_DeserializeOptions): { success: true; value: Page } | { success: false; errors: Array<{ field: string; message: string }> } {
    try {
        const data = typeof input === "string" ? JSON.parse(input) : input;
        const ctx = __mf_DeserializeContext.create();
        const resultOrRef = pageDeserializeWithContext(data, ctx);
        if (__mf_PendingRef.is(resultOrRef)) {
            return {
                success: false,
                errors: [
                    {
                        field: "_root",
                        message: "Page.deserialize: root cannot be a forward reference"
                    }
                ]
            };
        }
        ctx.applyPatches();
        if (opts?.freeze) {
            ctx.freezeAll();
        }
        return {
            success: true,
            value: resultOrRef
        };
    } catch (e) {
        if (e instanceof __mf_DeserializeError) {
            return {
                success: false,
                errors: e.errors
            };
        }
        const message = e instanceof Error ? e.message : String(e);
        return {
            success: false,
            errors: [
                {
                    field: "_root",
                    message
                }
            ]
        };
    }
}
export function pageDeserializeWithContext(value: any, ctx: __mf_DeserializeContext): Page | __mf_PendingRef {
    if (value?.__ref !== undefined) {
        return ctx.getOrDefer(value.__ref) as Page | __mf_PendingRef;
    }
    const allowedValues = [
        "'SalesHomeDashboard', 'SalesHomeProducts', 'SalesHomeServices', 'SalesHomePackages', 'SalesHomeTaxRates', 'SalesLeadsOverview', 'SalesLeadsActivities', 'SalesLeadsCampaigns', 'SalesLeadsDripCampaigns', 'SalesLeadsOpportunities', 'SalesLeadsPromotions', 'SalesAccountsOverview', 'SalesAccountsActivities', 'SalesAccountsBilling', 'SalesAccountsContracts', 'SalesOrdersOverview', 'SalesOrdersActivities', 'SalesOrdersPayments', 'SalesOrdersCommissions', 'SalesSchedulingSchedule', 'SalesSchedulingAppointments', 'SalesSchedulingRecurring', 'SalesSchedulingRoutes', 'SalesSchedulingReminders', 'UserHome'"
    ] as const;
    if (!allowedValues.includes(value)) {
        throw new __mf_DeserializeError([
            {
                field: "_root",
                message: `Invalid value for ${"Page"}: expected one of ` + allowedValues.map((v)=>JSON.stringify(v)).join(", ") + ", got " + JSON.stringify(value)
            }
        ]);
    }
    return value as Page;
}
export function pageIs(value: unknown): value is Page {
    const allowedValues = [
        "'SalesHomeDashboard', 'SalesHomeProducts', 'SalesHomeServices', 'SalesHomePackages', 'SalesHomeTaxRates', 'SalesLeadsOverview', 'SalesLeadsActivities', 'SalesLeadsCampaigns', 'SalesLeadsDripCampaigns', 'SalesLeadsOpportunities', 'SalesLeadsPromotions', 'SalesAccountsOverview', 'SalesAccountsActivities', 'SalesAccountsBilling', 'SalesAccountsContracts', 'SalesOrdersOverview', 'SalesOrdersActivities', 'SalesOrdersPayments', 'SalesOrdersCommissions', 'SalesSchedulingSchedule', 'SalesSchedulingAppointments', 'SalesSchedulingRecurring', 'SalesSchedulingRoutes', 'SalesSchedulingReminders', 'UserHome'"
    ] as const;
    return allowedValues.includes(value as any);
}

export type PageSalesHomeDashboardErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesHomeProductsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesHomeServicesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesHomePackagesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesHomeTaxRatesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesLeadsOverviewErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesLeadsActivitiesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesLeadsCampaignsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesLeadsDripCampaignsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesLeadsOpportunitiesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesLeadsPromotionsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesAccountsOverviewErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesAccountsActivitiesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesAccountsBillingErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesAccountsContractsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesOrdersOverviewErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesOrdersActivitiesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesOrdersPaymentsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesOrdersCommissionsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesSchedulingScheduleErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesSchedulingAppointmentsErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesSchedulingRecurringErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesSchedulingRoutesErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesSchedulingRemindersErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageUserHomeErrors = {
    _errors: __gf_Option<Array<string>>;
};
export type PageSalesHomeDashboardTainted = {
};
export type PageSalesHomeProductsTainted = {
};
export type PageSalesHomeServicesTainted = {
};
export type PageSalesHomePackagesTainted = {
};
export type PageSalesHomeTaxRatesTainted = {
};
export type PageSalesLeadsOverviewTainted = {
};
export type PageSalesLeadsActivitiesTainted = {
};
export type PageSalesLeadsCampaignsTainted = {
};
export type PageSalesLeadsDripCampaignsTainted = {
};
export type PageSalesLeadsOpportunitiesTainted = {
};
export type PageSalesLeadsPromotionsTainted = {
};
export type PageSalesAccountsOverviewTainted = {
};
export type PageSalesAccountsActivitiesTainted = {
};
export type PageSalesAccountsBillingTainted = {
};
export type PageSalesAccountsContractsTainted = {
};
export type PageSalesOrdersOverviewTainted = {
};
export type PageSalesOrdersActivitiesTainted = {
};
export type PageSalesOrdersPaymentsTainted = {
};
export type PageSalesOrdersCommissionsTainted = {
};
export type PageSalesSchedulingScheduleTainted = {
};
export type PageSalesSchedulingAppointmentsTainted = {
};
export type PageSalesSchedulingRecurringTainted = {
};
export type PageSalesSchedulingRoutesTainted = {
};
export type PageSalesSchedulingRemindersTainted = {
};
export type PageUserHomeTainted = {
};
export type PageErrors = ({
    _value: "SalesHomeDashboard";
} & PageSalesHomeDashboardErrors) | ({
    _value: "SalesHomeProducts";
} & PageSalesHomeProductsErrors) | ({
    _value: "SalesHomeServices";
} & PageSalesHomeServicesErrors) | ({
    _value: "SalesHomePackages";
} & PageSalesHomePackagesErrors) | ({
    _value: "SalesHomeTaxRates";
} & PageSalesHomeTaxRatesErrors) | ({
    _value: "SalesLeadsOverview";
} & PageSalesLeadsOverviewErrors) | ({
    _value: "SalesLeadsActivities";
} & PageSalesLeadsActivitiesErrors) | ({
    _value: "SalesLeadsCampaigns";
} & PageSalesLeadsCampaignsErrors) | ({
    _value: "SalesLeadsDripCampaigns";
} & PageSalesLeadsDripCampaignsErrors) | ({
    _value: "SalesLeadsOpportunities";
} & PageSalesLeadsOpportunitiesErrors) | ({
    _value: "SalesLeadsPromotions";
} & PageSalesLeadsPromotionsErrors) | ({
    _value: "SalesAccountsOverview";
} & PageSalesAccountsOverviewErrors) | ({
    _value: "SalesAccountsActivities";
} & PageSalesAccountsActivitiesErrors) | ({
    _value: "SalesAccountsBilling";
} & PageSalesAccountsBillingErrors) | ({
    _value: "SalesAccountsContracts";
} & PageSalesAccountsContractsErrors) | ({
    _value: "SalesOrdersOverview";
} & PageSalesOrdersOverviewErrors) | ({
    _value: "SalesOrdersActivities";
} & PageSalesOrdersActivitiesErrors) | ({
    _value: "SalesOrdersPayments";
} & PageSalesOrdersPaymentsErrors) | ({
    _value: "SalesOrdersCommissions";
} & PageSalesOrdersCommissionsErrors) | ({
    _value: "SalesSchedulingSchedule";
} & PageSalesSchedulingScheduleErrors) | ({
    _value: "SalesSchedulingAppointments";
} & PageSalesSchedulingAppointmentsErrors) | ({
    _value: "SalesSchedulingRecurring";
} & PageSalesSchedulingRecurringErrors) | ({
    _value: "SalesSchedulingRoutes";
} & PageSalesSchedulingRoutesErrors) | ({
    _value: "SalesSchedulingReminders";
} & PageSalesSchedulingRemindersErrors) | ({
    _value: "UserHome";
} & PageUserHomeErrors);
export type PageTainted = ({
    _value: "SalesHomeDashboard";
} & PageSalesHomeDashboardTainted) | ({
    _value: "SalesHomeProducts";
} & PageSalesHomeProductsTainted) | ({
    _value: "SalesHomeServices";
} & PageSalesHomeServicesTainted) | ({
    _value: "SalesHomePackages";
} & PageSalesHomePackagesTainted) | ({
    _value: "SalesHomeTaxRates";
} & PageSalesHomeTaxRatesTainted) | ({
    _value: "SalesLeadsOverview";
} & PageSalesLeadsOverviewTainted) | ({
    _value: "SalesLeadsActivities";
} & PageSalesLeadsActivitiesTainted) | ({
    _value: "SalesLeadsCampaigns";
} & PageSalesLeadsCampaignsTainted) | ({
    _value: "SalesLeadsDripCampaigns";
} & PageSalesLeadsDripCampaignsTainted) | ({
    _value: "SalesLeadsOpportunities";
} & PageSalesLeadsOpportunitiesTainted) | ({
    _value: "SalesLeadsPromotions";
} & PageSalesLeadsPromotionsTainted) | ({
    _value: "SalesAccountsOverview";
} & PageSalesAccountsOverviewTainted) | ({
    _value: "SalesAccountsActivities";
} & PageSalesAccountsActivitiesTainted) | ({
    _value: "SalesAccountsBilling";
} & PageSalesAccountsBillingTainted) | ({
    _value: "SalesAccountsContracts";
} & PageSalesAccountsContractsTainted) | ({
    _value: "SalesOrdersOverview";
} & PageSalesOrdersOverviewTainted) | ({
    _value: "SalesOrdersActivities";
} & PageSalesOrdersActivitiesTainted) | ({
    _value: "SalesOrdersPayments";
} & PageSalesOrdersPaymentsTainted) | ({
    _value: "SalesOrdersCommissions";
} & PageSalesOrdersCommissionsTainted) | ({
    _value: "SalesSchedulingSchedule";
} & PageSalesSchedulingScheduleTainted) | ({
    _value: "SalesSchedulingAppointments";
} & PageSalesSchedulingAppointmentsTainted) | ({
    _value: "SalesSchedulingRecurring";
} & PageSalesSchedulingRecurringTainted) | ({
    _value: "SalesSchedulingRoutes";
} & PageSalesSchedulingRoutesTainted) | ({
    _value: "SalesSchedulingReminders";
} & PageSalesSchedulingRemindersTainted) | ({
    _value: "UserHome";
} & PageUserHomeTainted);
export interface PageSalesHomeDashboardFieldControllers {
}
export interface PageSalesHomeProductsFieldControllers {
}
export interface PageSalesHomeServicesFieldControllers {
}
export interface PageSalesHomePackagesFieldControllers {
}
export interface PageSalesHomeTaxRatesFieldControllers {
}
export interface PageSalesLeadsOverviewFieldControllers {
}
export interface PageSalesLeadsActivitiesFieldControllers {
}
export interface PageSalesLeadsCampaignsFieldControllers {
}
export interface PageSalesLeadsDripCampaignsFieldControllers {
}
export interface PageSalesLeadsOpportunitiesFieldControllers {
}
export interface PageSalesLeadsPromotionsFieldControllers {
}
export interface PageSalesAccountsOverviewFieldControllers {
}
export interface PageSalesAccountsActivitiesFieldControllers {
}
export interface PageSalesAccountsBillingFieldControllers {
}
export interface PageSalesAccountsContractsFieldControllers {
}
export interface PageSalesOrdersOverviewFieldControllers {
}
export interface PageSalesOrdersActivitiesFieldControllers {
}
export interface PageSalesOrdersPaymentsFieldControllers {
}
export interface PageSalesOrdersCommissionsFieldControllers {
}
export interface PageSalesSchedulingScheduleFieldControllers {
}
export interface PageSalesSchedulingAppointmentsFieldControllers {
}
export interface PageSalesSchedulingRecurringFieldControllers {
}
export interface PageSalesSchedulingRoutesFieldControllers {
}
export interface PageSalesSchedulingRemindersFieldControllers {
}
export interface PageUserHomeFieldControllers {
}
export interface PageGigaform {
    readonly currentVariant: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
    readonly data: Page;
    readonly errors: PageErrors;
    readonly tainted: PageTainted;
    readonly variants: PageVariantFields;
    switchVariant(variant: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome"): void;
    validate(): Exit<Page, Array<{
        field: string;
        message: string;
    }>>;
    reset(overrides?: Partial<Page>): void;
}
export interface PageVariantFields {
    readonly SalesHomeDashboard: {
        readonly fields: PageSalesHomeDashboardFieldControllers;
    };
    readonly SalesHomeProducts: {
        readonly fields: PageSalesHomeProductsFieldControllers;
    };
    readonly SalesHomeServices: {
        readonly fields: PageSalesHomeServicesFieldControllers;
    };
    readonly SalesHomePackages: {
        readonly fields: PageSalesHomePackagesFieldControllers;
    };
    readonly SalesHomeTaxRates: {
        readonly fields: PageSalesHomeTaxRatesFieldControllers;
    };
    readonly SalesLeadsOverview: {
        readonly fields: PageSalesLeadsOverviewFieldControllers;
    };
    readonly SalesLeadsActivities: {
        readonly fields: PageSalesLeadsActivitiesFieldControllers;
    };
    readonly SalesLeadsCampaigns: {
        readonly fields: PageSalesLeadsCampaignsFieldControllers;
    };
    readonly SalesLeadsDripCampaigns: {
        readonly fields: PageSalesLeadsDripCampaignsFieldControllers;
    };
    readonly SalesLeadsOpportunities: {
        readonly fields: PageSalesLeadsOpportunitiesFieldControllers;
    };
    readonly SalesLeadsPromotions: {
        readonly fields: PageSalesLeadsPromotionsFieldControllers;
    };
    readonly SalesAccountsOverview: {
        readonly fields: PageSalesAccountsOverviewFieldControllers;
    };
    readonly SalesAccountsActivities: {
        readonly fields: PageSalesAccountsActivitiesFieldControllers;
    };
    readonly SalesAccountsBilling: {
        readonly fields: PageSalesAccountsBillingFieldControllers;
    };
    readonly SalesAccountsContracts: {
        readonly fields: PageSalesAccountsContractsFieldControllers;
    };
    readonly SalesOrdersOverview: {
        readonly fields: PageSalesOrdersOverviewFieldControllers;
    };
    readonly SalesOrdersActivities: {
        readonly fields: PageSalesOrdersActivitiesFieldControllers;
    };
    readonly SalesOrdersPayments: {
        readonly fields: PageSalesOrdersPaymentsFieldControllers;
    };
    readonly SalesOrdersCommissions: {
        readonly fields: PageSalesOrdersCommissionsFieldControllers;
    };
    readonly SalesSchedulingSchedule: {
        readonly fields: PageSalesSchedulingScheduleFieldControllers;
    };
    readonly SalesSchedulingAppointments: {
        readonly fields: PageSalesSchedulingAppointmentsFieldControllers;
    };
    readonly SalesSchedulingRecurring: {
        readonly fields: PageSalesSchedulingRecurringFieldControllers;
    };
    readonly SalesSchedulingRoutes: {
        readonly fields: PageSalesSchedulingRoutesFieldControllers;
    };
    readonly SalesSchedulingReminders: {
        readonly fields: PageSalesSchedulingRemindersFieldControllers;
    };
    readonly UserHome: {
        readonly fields: PageUserHomeFieldControllers;
    };
}
function pageGetDefaultForVariant(variant: string): Page {
    if (variant === "SalesHomeDashboard") {
        return "SalesHomeDashboard" as Page;
    }
    if (variant === "SalesHomeProducts") {
        return "SalesHomeProducts" as Page;
    }
    if (variant === "SalesHomeServices") {
        return "SalesHomeServices" as Page;
    }
    if (variant === "SalesHomePackages") {
        return "SalesHomePackages" as Page;
    }
    if (variant === "SalesHomeTaxRates") {
        return "SalesHomeTaxRates" as Page;
    }
    if (variant === "SalesLeadsOverview") {
        return "SalesLeadsOverview" as Page;
    }
    if (variant === "SalesLeadsActivities") {
        return "SalesLeadsActivities" as Page;
    }
    if (variant === "SalesLeadsCampaigns") {
        return "SalesLeadsCampaigns" as Page;
    }
    if (variant === "SalesLeadsDripCampaigns") {
        return "SalesLeadsDripCampaigns" as Page;
    }
    if (variant === "SalesLeadsOpportunities") {
        return "SalesLeadsOpportunities" as Page;
    }
    if (variant === "SalesLeadsPromotions") {
        return "SalesLeadsPromotions" as Page;
    }
    if (variant === "SalesAccountsOverview") {
        return "SalesAccountsOverview" as Page;
    }
    if (variant === "SalesAccountsActivities") {
        return "SalesAccountsActivities" as Page;
    }
    if (variant === "SalesAccountsBilling") {
        return "SalesAccountsBilling" as Page;
    }
    if (variant === "SalesAccountsContracts") {
        return "SalesAccountsContracts" as Page;
    }
    if (variant === "SalesOrdersOverview") {
        return "SalesOrdersOverview" as Page;
    }
    if (variant === "SalesOrdersActivities") {
        return "SalesOrdersActivities" as Page;
    }
    if (variant === "SalesOrdersPayments") {
        return "SalesOrdersPayments" as Page;
    }
    if (variant === "SalesOrdersCommissions") {
        return "SalesOrdersCommissions" as Page;
    }
    if (variant === "SalesSchedulingSchedule") {
        return "SalesSchedulingSchedule" as Page;
    }
    if (variant === "SalesSchedulingAppointments") {
        return "SalesSchedulingAppointments" as Page;
    }
    if (variant === "SalesSchedulingRecurring") {
        return "SalesSchedulingRecurring" as Page;
    }
    if (variant === "SalesSchedulingRoutes") {
        return "SalesSchedulingRoutes" as Page;
    }
    if (variant === "SalesSchedulingReminders") {
        return "SalesSchedulingReminders" as Page;
    }
    if (variant === "UserHome") {
        return "UserHome" as Page;
    }
    return "SalesHomeDashboard" as Page;
}
export function pageCreateForm(initial: Page): PageGigaform {
    const initialVariant: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome" = (initial as "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome") ?? "SalesHomeDashboard";
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "pageGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as PageErrors);
    let tainted = $state<$MfPh10>({} as PageTainted);
    const variants = {} as PageVariantFields;
    variants[__expr__] = {
        fields: {} as PageSalesHomeDashboardFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesHomeProductsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesHomeServicesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesHomePackagesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesHomeTaxRatesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesLeadsOverviewFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesLeadsActivitiesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesLeadsCampaignsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesLeadsDripCampaignsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesLeadsOpportunitiesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesLeadsPromotionsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesAccountsOverviewFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesAccountsActivitiesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesAccountsBillingFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesAccountsContractsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesOrdersOverviewFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesOrdersActivitiesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesOrdersPaymentsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesOrdersCommissionsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesSchedulingScheduleFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesSchedulingAppointmentsFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesSchedulingRecurringFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesSchedulingRoutesFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageSalesSchedulingRemindersFieldControllers
    };
    variants[__expr__] = {
        fields: {} as PageUserHomeFieldControllers
    };
    function switchVariant(variant: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome"): void {
        currentVariant = variant;
        data = "pageGetDefaultForVariant"(variant);
        errors = {} as PageErrors;
        tainted = {} as PageTainted;
    }
    function validate(): Exit<Page, Array<{
        field: string;
        message: string;
    }>> {
        return toExit(pageDeserialize(data));
    }
    function reset(overrides: Partial<Page>): void {
        data = overrides ? overrides as typeof data : pageGetDefaultForVariant(currentVariant);
        errors = {} as PageErrors;
        tainted = {} as PageTainted;
    }
    return {
        get currentVariant () {
            return currentVariant;
        },
        get data () {
            return data;
        },
        set data (v){
            data = v;
        },
        get errors () {
            return errors;
        },
        set errors (v){
            errors = v;
        },
        get tainted () {
            return tainted;
        },
        set tainted (v){
            tainted = v;
        },
        variants,
        switchVariant,
        validate,
        reset
    };
}
export function pageFromFormData(formData: FormData): Exit<Page, Array<{
    field: string;
    message: string;
}>> {
    const discriminant = formData.get(`${"_value"}`) as "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome" | null;
    if (!discriminant) {
        return toExit({
            success: false,
            errors: [
                {
                    field: `${"_value"}`,
                    message: "Missing discriminant field"
                }
            ]
        });
    }
    const obj: Record<string, unknown> = {};
    obj._value = discriminant;
    return toExit(pageDeserialize(obj));
}

export const Page = {
  serialize: pageSerialize,
  serializeWithContext: pageSerializeWithContext,
  deserialize: pageDeserialize,
  deserializeWithContext: pageDeserializeWithContext,
  is: pageIs,
  createForm: pageCreateForm,
  fromFormData: pageFromFormData
} as const;