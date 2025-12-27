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

export function pageSerialize#0#0(value: Page): string {
    const ctx = __mf_SerializeContext.create();
    return JSON.stringify(pageSerializeWithContext(value, ctx));
}
export function pageSerializeWithContext#0#0(value: Page, ctx: __mf_SerializeContext): unknown {
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

function pageGetDefaultForVariant(variant: string): Page {
    if (variant === `${"SalesHomeDashboard"}`) {
        return `${"SalesHomeDashboard"}` as Page;
    }
    if (variant === `${"SalesHomeProducts"}`) {
        return `${"SalesHomeProducts"}` as Page;
    }
    if (variant === `${"SalesHomeServices"}`) {
        return `${"SalesHomeServices"}` as Page;
    }
    if (variant === `${"SalesHomePackages"}`) {
        return `${"SalesHomePackages"}` as Page;
    }
    if (variant === `${"SalesHomeTaxRates"}`) {
        return `${"SalesHomeTaxRates"}` as Page;
    }
    if (variant === `${"SalesLeadsOverview"}`) {
        return `${"SalesLeadsOverview"}` as Page;
    }
    if (variant === `${"SalesLeadsActivities"}`) {
        return `${"SalesLeadsActivities"}` as Page;
    }
    if (variant === `${"SalesLeadsCampaigns"}`) {
        return `${"SalesLeadsCampaigns"}` as Page;
    }
    if (variant === `${"SalesLeadsDripCampaigns"}`) {
        return `${"SalesLeadsDripCampaigns"}` as Page;
    }
    if (variant === `${"SalesLeadsOpportunities"}`) {
        return `${"SalesLeadsOpportunities"}` as Page;
    }
    if (variant === `${"SalesLeadsPromotions"}`) {
        return `${"SalesLeadsPromotions"}` as Page;
    }
    if (variant === `${"SalesAccountsOverview"}`) {
        return `${"SalesAccountsOverview"}` as Page;
    }
    if (variant === `${"SalesAccountsActivities"}`) {
        return `${"SalesAccountsActivities"}` as Page;
    }
    if (variant === `${"SalesAccountsBilling"}`) {
        return `${"SalesAccountsBilling"}` as Page;
    }
    if (variant === `${"SalesAccountsContracts"}`) {
        return `${"SalesAccountsContracts"}` as Page;
    }
    if (variant === `${"SalesOrdersOverview"}`) {
        return `${"SalesOrdersOverview"}` as Page;
    }
    if (variant === `${"SalesOrdersActivities"}`) {
        return `${"SalesOrdersActivities"}` as Page;
    }
    if (variant === `${"SalesOrdersPayments"}`) {
        return `${"SalesOrdersPayments"}` as Page;
    }
    if (variant === `${"SalesOrdersCommissions"}`) {
        return `${"SalesOrdersCommissions"}` as Page;
    }
    if (variant === `${"SalesSchedulingSchedule"}`) {
        return `${"SalesSchedulingSchedule"}` as Page;
    }
    if (variant === `${"SalesSchedulingAppointments"}`) {
        return `${"SalesSchedulingAppointments"}` as Page;
    }
    if (variant === `${"SalesSchedulingRecurring"}`) {
        return `${"SalesSchedulingRecurring"}` as Page;
    }
    if (variant === `${"SalesSchedulingRoutes"}`) {
        return `${"SalesSchedulingRoutes"}` as Page;
    }
    if (variant === `${"SalesSchedulingReminders"}`) {
        return `${"SalesSchedulingReminders"}` as Page;
    }
    if (variant === `${"UserHome"}`) {
        return `${"UserHome"}` as Page;
    }
    return `${"SalesHomeDashboard"}` as Page;
}
export function pageCreateForm(initial: Page): PageGigaform {
    const initialVariant: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome" = '(initial as "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome") ?? "SalesHomeDashboard"';
    let currentVariant = $state<$MfPh5>(initialVariant);
    let data = $state<$MfPh6>(initial ?? "pageGetDefaultForVariant"(initialVariant));
    let errors = $state<$MfPh8>({} as PageErrors);
    let tainted = $state<$MfPh10>({} as PageTainted);
    const variants = {} as PageVariantFields;
    variants[SalesHomeDashboard] = {
        fields: {} as PageSalesHomeDashboardFieldControllers
    };
    variants[SalesHomeProducts] = {
        fields: {} as PageSalesHomeProductsFieldControllers
    };
    variants[SalesHomeServices] = {
        fields: {} as PageSalesHomeServicesFieldControllers
    };
    variants[SalesHomePackages] = {
        fields: {} as PageSalesHomePackagesFieldControllers
    };
    variants[SalesHomeTaxRates] = {
        fields: {} as PageSalesHomeTaxRatesFieldControllers
    };
    variants[SalesLeadsOverview] = {
        fields: {} as PageSalesLeadsOverviewFieldControllers
    };
    variants[SalesLeadsActivities] = {
        fields: {} as PageSalesLeadsActivitiesFieldControllers
    };
    variants[SalesLeadsCampaigns] = {
        fields: {} as PageSalesLeadsCampaignsFieldControllers
    };
    variants[SalesLeadsDripCampaigns] = {
        fields: {} as PageSalesLeadsDripCampaignsFieldControllers
    };
    variants[SalesLeadsOpportunities] = {
        fields: {} as PageSalesLeadsOpportunitiesFieldControllers
    };
    variants[SalesLeadsPromotions] = {
        fields: {} as PageSalesLeadsPromotionsFieldControllers
    };
    variants[SalesAccountsOverview] = {
        fields: {} as PageSalesAccountsOverviewFieldControllers
    };
    variants[SalesAccountsActivities] = {
        fields: {} as PageSalesAccountsActivitiesFieldControllers
    };
    variants[SalesAccountsBilling] = {
        fields: {} as PageSalesAccountsBillingFieldControllers
    };
    variants[SalesAccountsContracts] = {
        fields: {} as PageSalesAccountsContractsFieldControllers
    };
    variants[SalesOrdersOverview] = {
        fields: {} as PageSalesOrdersOverviewFieldControllers
    };
    variants[SalesOrdersActivities] = {
        fields: {} as PageSalesOrdersActivitiesFieldControllers
    };
    variants[SalesOrdersPayments] = {
        fields: {} as PageSalesOrdersPaymentsFieldControllers
    };
    variants[SalesOrdersCommissions] = {
        fields: {} as PageSalesOrdersCommissionsFieldControllers
    };
    variants[SalesSchedulingSchedule] = {
        fields: {} as PageSalesSchedulingScheduleFieldControllers
    };
    variants[SalesSchedulingAppointments] = {
        fields: {} as PageSalesSchedulingAppointmentsFieldControllers
    };
    variants[SalesSchedulingRecurring] = {
        fields: {} as PageSalesSchedulingRecurringFieldControllers
    };
    variants[SalesSchedulingRoutes] = {
        fields: {} as PageSalesSchedulingRoutesFieldControllers
    };
    variants[SalesSchedulingReminders] = {
        fields: {} as PageSalesSchedulingRemindersFieldControllers
    };
    variants[UserHome] = {
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
        return toExit("pageDeserialize(data)");
    }
    function reset(overrides: Partial<Page>): void {
        data = "overrides ? overrides as typeof data : pageGetDefaultForVariant(currentVariant)";
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
    return toExit("pageDeserialize(obj)");
}
export type $MfPh0 = $MfPh1;
export type $MfPh2 = $MfPh3;
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
export interface $MfPh4 {
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
    reset(overrides: Partial<Page>): void;
}
export interface $MfPh13 {
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
 };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  }; export type PageSalesHomeDashboardTainted = {
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
 };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };  };

export const Page = {
  deserialize: pageDeserialize,
  deserializeWithContext: pageDeserializeWithContext,
  is: pageIs,
  createForm: pageCreateForm,
  fromFormData: pageFromFormData
} as const;