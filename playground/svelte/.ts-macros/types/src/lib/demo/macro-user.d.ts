import { Schema } from "effect";
import { TaxRate, Site, Represents, Ordered, Did, PhoneNumber, Email, Colors } from "../types/bindings";
export declare class MacroUser {
    id: string;
    name: string;
    role: string;
    favoriteMacro: "Derive" | "JsonNative";
    since: string;
    apiToken: string;
    constructor(id: string, name: string, role: string, favoriteMacro: "Derive" | "JsonNative", since: string, apiToken: string);
    toString(): string;
    toJSON(): Record<string, unknown>;
}
export declare const showcaseUserSummary: string;
export declare const showcaseUserJson: Record<string, unknown>;
declare const Account_base: Schema.Class<Account, {
    id: Schema.propertySignature<typeof Schema.String>;
    taxRate: Schema.propertySignature<Schema.Union<[Schema.filter<typeof Schema.String>, typeof TaxRate]>>;
    site: Schema.propertySignature<Schema.Union<[Schema.filter<typeof Schema.String>, typeof Site]>>;
    salesRep: Schema.OptionFromNullishOr<Schema.Array$<typeof Represents>>;
    orders: Schema.propertySignature<Schema.Array$<Schema.SchemaClass<Ordered, {
        readonly id: string;
        readonly date: string;
        readonly in: string | {
            readonly id: string;
            readonly salesRep: readonly import("../types/bindings").RepresentsEncoded[] | null | undefined;
            readonly phones: readonly {
                readonly number: string;
                readonly main: boolean;
                readonly phoneType: string;
                readonly canText: boolean;
                readonly canCall: boolean;
            }[];
            readonly email: {
                readonly canEmail: boolean;
                readonly emailString: string;
            };
            readonly tags: readonly string[];
            readonly colors: {
                readonly main: string;
                readonly hover: string;
                readonly active: string;
            };
            readonly taxRate: string | {
                readonly name: string;
                readonly id: string;
                readonly description: string;
                readonly isActive: boolean;
                readonly taxAgency: string;
                readonly zip: number;
                readonly city: string;
                readonly county: string;
                readonly state: string;
                readonly taxComponents: {
                    readonly [x: string]: number;
                };
            };
            readonly site: string | {
                readonly id: string;
                readonly addressLine1: string;
                readonly addressLine2: string | null | undefined;
                readonly sublocalityLevel1: string | null | undefined;
                readonly locality: string;
                readonly administrativeAreaLevel3: string | null | undefined;
                readonly administrativeAreaLevel2: string | null | undefined;
                readonly administrativeAreaLevel1: string;
                readonly country: string;
                readonly postalCode: string;
                readonly postalCodeSuffix: string | null | undefined;
                readonly coordinates: {
                    readonly lat: number;
                    readonly lng: number;
                };
            };
            readonly orders: readonly import("../types/bindings").OrderedEncoded[];
            readonly activity: readonly import("../types/bindings").DidEncoded[];
            readonly customFields: readonly (readonly [string, string])[];
            readonly accountName: {
                readonly firstName: string;
                readonly lastName: string;
            } | {
                readonly companyName: string;
            };
            readonly sector: "Residential" | "Commercial";
            readonly memo: string | null | undefined;
            readonly leadSource: string;
            readonly needsReview: boolean;
            readonly hasAlert: boolean;
            readonly accountType: string;
            readonly subtype: string;
            readonly isTaxExempt: boolean;
            readonly paymentTerms: string;
            readonly dateAdded: string;
        };
        readonly out: string | {
            readonly number: number;
            readonly id: string;
            readonly salesRep: string | {
                readonly active: boolean;
                readonly name: string;
                readonly id: string;
                readonly imageUrl: string | null | undefined;
                readonly phones: readonly {
                    readonly number: string;
                    readonly main: boolean;
                    readonly phoneType: string;
                    readonly canText: boolean;
                    readonly canCall: boolean;
                }[];
                readonly role: string;
                readonly title: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
                readonly email: {
                    readonly canEmail: boolean;
                    readonly emailString: string;
                };
                readonly address: string;
                readonly username: string;
                readonly route: string | import("../types/bindings").RouteEncoded;
                readonly ratePerHour: number;
                readonly isTechnician: boolean;
                readonly isSalesRep: boolean;
                readonly description: string | null | undefined;
                readonly linkedinUrl: string | null | undefined;
                readonly attendance: readonly string[];
                readonly settings: {
                    readonly appointmentNotifications: {
                        readonly personalScheduleChangeNotifications: string;
                        readonly allScheduleChangeNotifications: string;
                    } | null | undefined;
                    readonly commissions: {
                        readonly technician: string;
                        readonly salesRep: string;
                    } | null | undefined;
                    readonly scheduleSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly daysPerWeek: number;
                        readonly visibleRoutes: readonly string[];
                        readonly detailedCards: boolean;
                    };
                    readonly accountOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly serviceOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly appointmentOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly leadOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly packageOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly productOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly orderOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly taxRateOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
                };
            };
            readonly commissions: readonly string[];
            readonly group: string;
            readonly subgroup: string;
            readonly upsale: string;
            readonly site: string | {
                readonly id: string;
                readonly addressLine1: string;
                readonly addressLine2: string | null | undefined;
                readonly sublocalityLevel1: string | null | undefined;
                readonly locality: string;
                readonly administrativeAreaLevel3: string | null | undefined;
                readonly administrativeAreaLevel2: string | null | undefined;
                readonly administrativeAreaLevel1: string;
                readonly country: string;
                readonly postalCode: string;
                readonly postalCodeSuffix: string | null | undefined;
                readonly coordinates: {
                    readonly lat: number;
                    readonly lng: number;
                };
            };
            readonly memo: string;
            readonly leadSource: string;
            readonly needsReview: boolean;
            readonly stage: "Estimate" | "Active" | "Invoice";
            readonly account: string | {
                readonly id: string;
                readonly salesRep: readonly import("../types/bindings").RepresentsEncoded[] | null | undefined;
                readonly phones: readonly {
                    readonly number: string;
                    readonly main: boolean;
                    readonly phoneType: string;
                    readonly canText: boolean;
                    readonly canCall: boolean;
                }[];
                readonly email: {
                    readonly canEmail: boolean;
                    readonly emailString: string;
                };
                readonly tags: readonly string[];
                readonly colors: {
                    readonly main: string;
                    readonly hover: string;
                    readonly active: string;
                };
                readonly taxRate: string | {
                    readonly name: string;
                    readonly id: string;
                    readonly description: string;
                    readonly isActive: boolean;
                    readonly taxAgency: string;
                    readonly zip: number;
                    readonly city: string;
                    readonly county: string;
                    readonly state: string;
                    readonly taxComponents: {
                        readonly [x: string]: number;
                    };
                };
                readonly site: string | {
                    readonly id: string;
                    readonly addressLine1: string;
                    readonly addressLine2: string | null | undefined;
                    readonly sublocalityLevel1: string | null | undefined;
                    readonly locality: string;
                    readonly administrativeAreaLevel3: string | null | undefined;
                    readonly administrativeAreaLevel2: string | null | undefined;
                    readonly administrativeAreaLevel1: string;
                    readonly country: string;
                    readonly postalCode: string;
                    readonly postalCodeSuffix: string | null | undefined;
                    readonly coordinates: {
                        readonly lat: number;
                        readonly lng: number;
                    };
                };
                readonly orders: readonly import("../types/bindings").OrderedEncoded[];
                readonly activity: readonly import("../types/bindings").DidEncoded[];
                readonly customFields: readonly (readonly [string, string])[];
                readonly accountName: {
                    readonly firstName: string;
                    readonly lastName: string;
                } | {
                    readonly companyName: string;
                };
                readonly sector: "Residential" | "Commercial";
                readonly memo: string | null | undefined;
                readonly leadSource: string;
                readonly needsReview: boolean;
                readonly hasAlert: boolean;
                readonly accountType: string;
                readonly subtype: string;
                readonly isTaxExempt: boolean;
                readonly paymentTerms: string;
                readonly dateAdded: string;
            };
            readonly payments: readonly (string | {
                readonly id: string;
                readonly date: string;
            })[];
            readonly opportunity: string;
            readonly reference: string;
            readonly isPosted: boolean;
            readonly actionItem: string;
            readonly dateCreated: string;
            readonly appointment: string | {
                readonly id: string;
                readonly title: string;
                readonly description: string | null | undefined;
                readonly status: "Scheduled" | "OnDeck" | "Waiting";
                readonly begins: string;
                readonly duration: Schema.DurationEncoded | readonly [seconds: number, nanos: number];
                readonly timeZone: string;
                readonly offsetMs: number;
                readonly allDay: boolean;
                readonly multiDay: boolean;
                readonly employees: readonly (string | {
                    readonly active: boolean;
                    readonly name: string;
                    readonly id: string;
                    readonly imageUrl: string | null | undefined;
                    readonly phones: readonly {
                        readonly number: string;
                        readonly main: boolean;
                        readonly phoneType: string;
                        readonly canText: boolean;
                        readonly canCall: boolean;
                    }[];
                    readonly role: string;
                    readonly title: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
                    readonly email: {
                        readonly canEmail: boolean;
                        readonly emailString: string;
                    };
                    readonly address: string;
                    readonly username: string;
                    readonly route: string | import("../types/bindings").RouteEncoded;
                    readonly ratePerHour: number;
                    readonly isTechnician: boolean;
                    readonly isSalesRep: boolean;
                    readonly description: string | null | undefined;
                    readonly linkedinUrl: string | null | undefined;
                    readonly attendance: readonly string[];
                    readonly settings: {
                        readonly appointmentNotifications: {
                            readonly personalScheduleChangeNotifications: string;
                            readonly allScheduleChangeNotifications: string;
                        } | null | undefined;
                        readonly commissions: {
                            readonly technician: string;
                            readonly salesRep: string;
                        } | null | undefined;
                        readonly scheduleSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly daysPerWeek: number;
                            readonly visibleRoutes: readonly string[];
                            readonly detailedCards: boolean;
                        };
                        readonly accountOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly serviceOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly appointmentOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly leadOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly packageOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly productOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly orderOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly taxRateOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
                    };
                })[];
                readonly location: string | {
                    readonly id: string;
                    readonly addressLine1: string;
                    readonly addressLine2: string | null | undefined;
                    readonly sublocalityLevel1: string | null | undefined;
                    readonly locality: string;
                    readonly administrativeAreaLevel3: string | null | undefined;
                    readonly administrativeAreaLevel2: string | null | undefined;
                    readonly administrativeAreaLevel1: string;
                    readonly country: string;
                    readonly postalCode: string;
                    readonly postalCodeSuffix: string | null | undefined;
                    readonly coordinates: {
                        readonly lat: number;
                        readonly lng: number;
                    };
                };
                readonly colors: {
                    readonly main: string;
                    readonly hover: string;
                    readonly active: string;
                };
                readonly recurrenceRule: {
                    readonly interval: {
                        readonly quantityOfMonths: number;
                        readonly day: number;
                        readonly name: string;
                    } | {
                        readonly quantityOfYears: number;
                    } | {
                        readonly quantityOfWeeks: number;
                        readonly weekdays: readonly ("Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday")[];
                    } | {
                        readonly quantityOfDays: number;
                    };
                    readonly recurrenceBegins: string;
                    readonly recurrenceEnds: string | number | null | undefined;
                    readonly cancelledInstances: readonly string[] | null | undefined;
                    readonly additionalInstances: readonly string[] | null | undefined;
                } | null | undefined;
            };
            readonly lastTechs: readonly (string | {
                readonly active: boolean;
                readonly name: string;
                readonly id: string;
                readonly imageUrl: string | null | undefined;
                readonly phones: readonly {
                    readonly number: string;
                    readonly main: boolean;
                    readonly phoneType: string;
                    readonly canText: boolean;
                    readonly canCall: boolean;
                }[];
                readonly role: string;
                readonly title: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
                readonly email: {
                    readonly canEmail: boolean;
                    readonly emailString: string;
                };
                readonly address: string;
                readonly username: string;
                readonly route: string | import("../types/bindings").RouteEncoded;
                readonly ratePerHour: number;
                readonly isTechnician: boolean;
                readonly isSalesRep: boolean;
                readonly description: string | null | undefined;
                readonly linkedinUrl: string | null | undefined;
                readonly attendance: readonly string[];
                readonly settings: {
                    readonly appointmentNotifications: {
                        readonly personalScheduleChangeNotifications: string;
                        readonly allScheduleChangeNotifications: string;
                    } | null | undefined;
                    readonly commissions: {
                        readonly technician: string;
                        readonly salesRep: string;
                    } | null | undefined;
                    readonly scheduleSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly daysPerWeek: number;
                        readonly visibleRoutes: readonly string[];
                        readonly detailedCards: boolean;
                    };
                    readonly accountOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly serviceOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly appointmentOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly leadOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly packageOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly productOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly orderOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly taxRateOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
                };
            })[];
            readonly package: readonly (string | {
                readonly id: string;
                readonly date: string;
            })[] | null | undefined;
            readonly promotion: readonly (string | {
                readonly id: string;
                readonly date: string;
            })[] | null | undefined;
            readonly balance: string;
            readonly due: string;
            readonly total: string;
            readonly billedItems: readonly {
                readonly item: string | {
                    readonly active: boolean;
                    readonly name: string;
                    readonly id: string;
                    readonly quickCode: string;
                    readonly group: string | null | undefined;
                    readonly subgroup: string | null | undefined;
                    readonly unit: string | null | undefined;
                    readonly commission: boolean;
                    readonly favorite: boolean;
                    readonly defaults: {
                        readonly description: string;
                        readonly price: number;
                    };
                } | {
                    readonly active: boolean;
                    readonly name: string;
                    readonly id: string;
                    readonly quickCode: string;
                    readonly group: string | null | undefined;
                    readonly subgroup: string | null | undefined;
                    readonly unit: string | null | undefined;
                    readonly commission: boolean;
                    readonly favorite: boolean;
                    readonly defaults: {
                        readonly description: string;
                        readonly price: number;
                    };
                    readonly averageTime: string | null | undefined;
                };
                readonly quantity: number;
                readonly taxed: boolean;
                readonly upsale: boolean;
            }[];
            readonly discount: string;
            readonly tip: string;
        };
    }, never>>>;
    activity: Schema.propertySignature<Schema.Array$<Schema.SchemaClass<Did, {
        readonly id: string;
        readonly createdAt: string;
        readonly metadata: string | null | undefined;
        readonly in: string | {
            readonly active: boolean;
            readonly name: string;
            readonly id: string;
            readonly imageUrl: string | null | undefined;
            readonly phones: readonly {
                readonly number: string;
                readonly main: boolean;
                readonly phoneType: string;
                readonly canText: boolean;
                readonly canCall: boolean;
            }[];
            readonly role: string;
            readonly title: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
            readonly email: {
                readonly canEmail: boolean;
                readonly emailString: string;
            };
            readonly address: string;
            readonly username: string;
            readonly route: string | import("../types/bindings").RouteEncoded;
            readonly ratePerHour: number;
            readonly isTechnician: boolean;
            readonly isSalesRep: boolean;
            readonly description: string | null | undefined;
            readonly linkedinUrl: string | null | undefined;
            readonly attendance: readonly string[];
            readonly settings: {
                readonly appointmentNotifications: {
                    readonly personalScheduleChangeNotifications: string;
                    readonly allScheduleChangeNotifications: string;
                } | null | undefined;
                readonly commissions: {
                    readonly technician: string;
                    readonly salesRep: string;
                } | null | undefined;
                readonly scheduleSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly daysPerWeek: number;
                    readonly visibleRoutes: readonly string[];
                    readonly detailedCards: boolean;
                };
                readonly accountOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly serviceOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly appointmentOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly leadOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly packageOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly productOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly orderOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly taxRateOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
            };
        } | {
            readonly id: string;
            readonly role: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology" | "Administrator";
            readonly email: string | null | undefined;
            readonly settings: {
                readonly appointmentNotifications: {
                    readonly personalScheduleChangeNotifications: string;
                    readonly allScheduleChangeNotifications: string;
                } | null | undefined;
                readonly commissions: {
                    readonly technician: string;
                    readonly salesRep: string;
                } | null | undefined;
                readonly scheduleSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly daysPerWeek: number;
                    readonly visibleRoutes: readonly string[];
                    readonly detailedCards: boolean;
                };
                readonly accountOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly serviceOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly appointmentOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly leadOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly packageOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly productOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly orderOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly taxRateOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
            };
            readonly password: string | null | undefined;
            readonly firstName: string;
            readonly lastName: string;
            readonly metadata: {
                readonly createdAt: string;
                readonly lastLogin: string | null | undefined;
                readonly isActive: boolean;
                readonly roles: readonly string[];
            } | null | undefined;
            readonly emailVerified: boolean;
            readonly verificationToken: string | null | undefined;
            readonly verificationExpires: string | null | undefined;
            readonly passwordResetToken: string | null | undefined;
            readonly passwordResetExpires: string | null | undefined;
            readonly permissions: {
                readonly applications: readonly ("HumanResources" | "Sales" | "Accounting" | "Errand" | "Logistics" | "Marketing" | "Website")[];
                readonly pages: readonly ("SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome")[];
                readonly data: readonly ("Site" | "Employee" | "Route" | "Appointment" | "Product" | "Service" | "Account" | "Did" | "Lead" | "TaxRate" | "Company" | "User" | "Order" | "Payment" | "Package" | "Promotion" | "Represents" | "Ordered")[];
            };
        } | {
            readonly id: string;
            readonly salesRep: readonly import("../types/bindings").RepresentsEncoded[] | null | undefined;
            readonly phones: readonly {
                readonly number: string;
                readonly main: boolean;
                readonly phoneType: string;
                readonly canText: boolean;
                readonly canCall: boolean;
            }[];
            readonly email: {
                readonly canEmail: boolean;
                readonly emailString: string;
            };
            readonly tags: readonly string[];
            readonly colors: {
                readonly main: string;
                readonly hover: string;
                readonly active: string;
            };
            readonly taxRate: string | {
                readonly name: string;
                readonly id: string;
                readonly description: string;
                readonly isActive: boolean;
                readonly taxAgency: string;
                readonly zip: number;
                readonly city: string;
                readonly county: string;
                readonly state: string;
                readonly taxComponents: {
                    readonly [x: string]: number;
                };
            };
            readonly site: string | {
                readonly id: string;
                readonly addressLine1: string;
                readonly addressLine2: string | null | undefined;
                readonly sublocalityLevel1: string | null | undefined;
                readonly locality: string;
                readonly administrativeAreaLevel3: string | null | undefined;
                readonly administrativeAreaLevel2: string | null | undefined;
                readonly administrativeAreaLevel1: string;
                readonly country: string;
                readonly postalCode: string;
                readonly postalCodeSuffix: string | null | undefined;
                readonly coordinates: {
                    readonly lat: number;
                    readonly lng: number;
                };
            };
            readonly orders: readonly import("../types/bindings").OrderedEncoded[];
            readonly activity: readonly import("../types/bindings").DidEncoded[];
            readonly customFields: readonly (readonly [string, string])[];
            readonly accountName: {
                readonly firstName: string;
                readonly lastName: string;
            } | {
                readonly companyName: string;
            };
            readonly sector: "Residential" | "Commercial";
            readonly memo: string | null | undefined;
            readonly leadSource: string;
            readonly needsReview: boolean;
            readonly hasAlert: boolean;
            readonly accountType: string;
            readonly subtype: string;
            readonly isTaxExempt: boolean;
            readonly paymentTerms: string;
            readonly dateAdded: string;
        };
        readonly out: string | import("../types/bindings").TargetEncoded;
        readonly activityType: {
            readonly durationSeconds: number | null | undefined;
            readonly source: string | null | undefined;
        } | {
            readonly recipient: string | null | undefined;
            readonly method: string | null | undefined;
        } | {
            readonly amount: string | null | undefined;
            readonly currency: string | null | undefined;
            readonly paymentMethod: string | null | undefined;
        } | {
            readonly comment: string;
            readonly replyTo: string | null | undefined;
        } | {
            readonly fieldName: string;
            readonly oldValue: string | null | undefined;
            readonly newValue: string | null | undefined;
        } | {
            readonly initialData: string | null | undefined;
        };
    }, never>>>;
    customFields: Schema.propertySignature<Schema.Array$<Schema.Tuple2<Schema.filter<typeof Schema.String>, Schema.filter<typeof Schema.String>>>>;
    accountName: Schema.propertySignature<Schema.Union<[typeof import("../types/bindings").CompanyName, typeof import("../types/bindings").PersonName]>>;
    sector: Schema.propertySignature<Schema.Union<[Schema.Literal<["Residential"]>, Schema.Literal<["Commercial"]>]>>;
    memo: Schema.OptionFromNullishOr<Schema.filter<typeof Schema.String>>;
    phones: Schema.propertySignature<Schema.Array$<typeof PhoneNumber>>;
    email: Schema.propertySignature<typeof Email>;
    leadSource: Schema.propertySignature<Schema.filter<typeof Schema.String>>;
    colors: Schema.propertySignature<typeof Colors>;
    needsReview: Schema.propertySignature<typeof Schema.Boolean>;
    hasAlert: Schema.propertySignature<typeof Schema.Boolean>;
    accountType: Schema.propertySignature<Schema.filter<typeof Schema.String>>;
    subtype: Schema.propertySignature<Schema.filter<typeof Schema.String>>;
    isTaxExempt: Schema.propertySignature<typeof Schema.Boolean>;
    paymentTerms: Schema.propertySignature<Schema.filter<typeof Schema.String>>;
    tags: Schema.propertySignature<Schema.Array$<Schema.filter<typeof Schema.String>>>;
    dateAdded: Schema.propertySignature<typeof Schema.DateTimeUtc>;
}, Schema.Struct.Encoded<{
    id: Schema.propertySignature<typeof Schema.String>;
    taxRate: Schema.propertySignature<Schema.Union<[Schema.filter<typeof Schema.String>, typeof TaxRate]>>;
    site: Schema.propertySignature<Schema.Union<[Schema.filter<typeof Schema.String>, typeof Site]>>;
    salesRep: Schema.OptionFromNullishOr<Schema.Array$<typeof Represents>>;
    orders: Schema.propertySignature<Schema.Array$<Schema.SchemaClass<Ordered, {
        readonly id: string;
        readonly date: string;
        readonly in: string | {
            readonly id: string;
            readonly salesRep: readonly import("../types/bindings").RepresentsEncoded[] | null | undefined;
            readonly phones: readonly {
                readonly number: string;
                readonly main: boolean;
                readonly phoneType: string;
                readonly canText: boolean;
                readonly canCall: boolean;
            }[];
            readonly email: {
                readonly canEmail: boolean;
                readonly emailString: string;
            };
            readonly tags: readonly string[];
            readonly colors: {
                readonly main: string;
                readonly hover: string;
                readonly active: string;
            };
            readonly taxRate: string | {
                readonly name: string;
                readonly id: string;
                readonly description: string;
                readonly isActive: boolean;
                readonly taxAgency: string;
                readonly zip: number;
                readonly city: string;
                readonly county: string;
                readonly state: string;
                readonly taxComponents: {
                    readonly [x: string]: number;
                };
            };
            readonly site: string | {
                readonly id: string;
                readonly addressLine1: string;
                readonly addressLine2: string | null | undefined;
                readonly sublocalityLevel1: string | null | undefined;
                readonly locality: string;
                readonly administrativeAreaLevel3: string | null | undefined;
                readonly administrativeAreaLevel2: string | null | undefined;
                readonly administrativeAreaLevel1: string;
                readonly country: string;
                readonly postalCode: string;
                readonly postalCodeSuffix: string | null | undefined;
                readonly coordinates: {
                    readonly lat: number;
                    readonly lng: number;
                };
            };
            readonly orders: readonly import("../types/bindings").OrderedEncoded[];
            readonly activity: readonly import("../types/bindings").DidEncoded[];
            readonly customFields: readonly (readonly [string, string])[];
            readonly accountName: {
                readonly firstName: string;
                readonly lastName: string;
            } | {
                readonly companyName: string;
            };
            readonly sector: "Residential" | "Commercial";
            readonly memo: string | null | undefined;
            readonly leadSource: string;
            readonly needsReview: boolean;
            readonly hasAlert: boolean;
            readonly accountType: string;
            readonly subtype: string;
            readonly isTaxExempt: boolean;
            readonly paymentTerms: string;
            readonly dateAdded: string;
        };
        readonly out: string | {
            readonly number: number;
            readonly id: string;
            readonly salesRep: string | {
                readonly active: boolean;
                readonly name: string;
                readonly id: string;
                readonly imageUrl: string | null | undefined;
                readonly phones: readonly {
                    readonly number: string;
                    readonly main: boolean;
                    readonly phoneType: string;
                    readonly canText: boolean;
                    readonly canCall: boolean;
                }[];
                readonly role: string;
                readonly title: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
                readonly email: {
                    readonly canEmail: boolean;
                    readonly emailString: string;
                };
                readonly address: string;
                readonly username: string;
                readonly route: string | import("../types/bindings").RouteEncoded;
                readonly ratePerHour: number;
                readonly isTechnician: boolean;
                readonly isSalesRep: boolean;
                readonly description: string | null | undefined;
                readonly linkedinUrl: string | null | undefined;
                readonly attendance: readonly string[];
                readonly settings: {
                    readonly appointmentNotifications: {
                        readonly personalScheduleChangeNotifications: string;
                        readonly allScheduleChangeNotifications: string;
                    } | null | undefined;
                    readonly commissions: {
                        readonly technician: string;
                        readonly salesRep: string;
                    } | null | undefined;
                    readonly scheduleSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly daysPerWeek: number;
                        readonly visibleRoutes: readonly string[];
                        readonly detailedCards: boolean;
                    };
                    readonly accountOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly serviceOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly appointmentOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly leadOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly packageOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly productOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly orderOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly taxRateOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
                };
            };
            readonly commissions: readonly string[];
            readonly group: string;
            readonly subgroup: string;
            readonly upsale: string;
            readonly site: string | {
                readonly id: string;
                readonly addressLine1: string;
                readonly addressLine2: string | null | undefined;
                readonly sublocalityLevel1: string | null | undefined;
                readonly locality: string;
                readonly administrativeAreaLevel3: string | null | undefined;
                readonly administrativeAreaLevel2: string | null | undefined;
                readonly administrativeAreaLevel1: string;
                readonly country: string;
                readonly postalCode: string;
                readonly postalCodeSuffix: string | null | undefined;
                readonly coordinates: {
                    readonly lat: number;
                    readonly lng: number;
                };
            };
            readonly memo: string;
            readonly leadSource: string;
            readonly needsReview: boolean;
            readonly stage: "Estimate" | "Active" | "Invoice";
            readonly account: string | {
                readonly id: string;
                readonly salesRep: readonly import("../types/bindings").RepresentsEncoded[] | null | undefined;
                readonly phones: readonly {
                    readonly number: string;
                    readonly main: boolean;
                    readonly phoneType: string;
                    readonly canText: boolean;
                    readonly canCall: boolean;
                }[];
                readonly email: {
                    readonly canEmail: boolean;
                    readonly emailString: string;
                };
                readonly tags: readonly string[];
                readonly colors: {
                    readonly main: string;
                    readonly hover: string;
                    readonly active: string;
                };
                readonly taxRate: string | {
                    readonly name: string;
                    readonly id: string;
                    readonly description: string;
                    readonly isActive: boolean;
                    readonly taxAgency: string;
                    readonly zip: number;
                    readonly city: string;
                    readonly county: string;
                    readonly state: string;
                    readonly taxComponents: {
                        readonly [x: string]: number;
                    };
                };
                readonly site: string | {
                    readonly id: string;
                    readonly addressLine1: string;
                    readonly addressLine2: string | null | undefined;
                    readonly sublocalityLevel1: string | null | undefined;
                    readonly locality: string;
                    readonly administrativeAreaLevel3: string | null | undefined;
                    readonly administrativeAreaLevel2: string | null | undefined;
                    readonly administrativeAreaLevel1: string;
                    readonly country: string;
                    readonly postalCode: string;
                    readonly postalCodeSuffix: string | null | undefined;
                    readonly coordinates: {
                        readonly lat: number;
                        readonly lng: number;
                    };
                };
                readonly orders: readonly import("../types/bindings").OrderedEncoded[];
                readonly activity: readonly import("../types/bindings").DidEncoded[];
                readonly customFields: readonly (readonly [string, string])[];
                readonly accountName: {
                    readonly firstName: string;
                    readonly lastName: string;
                } | {
                    readonly companyName: string;
                };
                readonly sector: "Residential" | "Commercial";
                readonly memo: string | null | undefined;
                readonly leadSource: string;
                readonly needsReview: boolean;
                readonly hasAlert: boolean;
                readonly accountType: string;
                readonly subtype: string;
                readonly isTaxExempt: boolean;
                readonly paymentTerms: string;
                readonly dateAdded: string;
            };
            readonly payments: readonly (string | {
                readonly id: string;
                readonly date: string;
            })[];
            readonly opportunity: string;
            readonly reference: string;
            readonly isPosted: boolean;
            readonly actionItem: string;
            readonly dateCreated: string;
            readonly appointment: string | {
                readonly id: string;
                readonly title: string;
                readonly description: string | null | undefined;
                readonly status: "Scheduled" | "OnDeck" | "Waiting";
                readonly begins: string;
                readonly duration: Schema.DurationEncoded | readonly [seconds: number, nanos: number];
                readonly timeZone: string;
                readonly offsetMs: number;
                readonly allDay: boolean;
                readonly multiDay: boolean;
                readonly employees: readonly (string | {
                    readonly active: boolean;
                    readonly name: string;
                    readonly id: string;
                    readonly imageUrl: string | null | undefined;
                    readonly phones: readonly {
                        readonly number: string;
                        readonly main: boolean;
                        readonly phoneType: string;
                        readonly canText: boolean;
                        readonly canCall: boolean;
                    }[];
                    readonly role: string;
                    readonly title: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
                    readonly email: {
                        readonly canEmail: boolean;
                        readonly emailString: string;
                    };
                    readonly address: string;
                    readonly username: string;
                    readonly route: string | import("../types/bindings").RouteEncoded;
                    readonly ratePerHour: number;
                    readonly isTechnician: boolean;
                    readonly isSalesRep: boolean;
                    readonly description: string | null | undefined;
                    readonly linkedinUrl: string | null | undefined;
                    readonly attendance: readonly string[];
                    readonly settings: {
                        readonly appointmentNotifications: {
                            readonly personalScheduleChangeNotifications: string;
                            readonly allScheduleChangeNotifications: string;
                        } | null | undefined;
                        readonly commissions: {
                            readonly technician: string;
                            readonly salesRep: string;
                        } | null | undefined;
                        readonly scheduleSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly daysPerWeek: number;
                            readonly visibleRoutes: readonly string[];
                            readonly detailedCards: boolean;
                        };
                        readonly accountOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly serviceOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly appointmentOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly leadOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly packageOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly productOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly orderOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly taxRateOverviewSettings: {
                            readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                            readonly cardOrRow: "Card" | "Table";
                            readonly perPage: number;
                            readonly columnConfigs: readonly {
                                readonly heading: string;
                                readonly dataPath: {
                                    readonly path: readonly string[];
                                    readonly formatter: string | null | undefined;
                                };
                            }[];
                        };
                        readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
                    };
                })[];
                readonly location: string | {
                    readonly id: string;
                    readonly addressLine1: string;
                    readonly addressLine2: string | null | undefined;
                    readonly sublocalityLevel1: string | null | undefined;
                    readonly locality: string;
                    readonly administrativeAreaLevel3: string | null | undefined;
                    readonly administrativeAreaLevel2: string | null | undefined;
                    readonly administrativeAreaLevel1: string;
                    readonly country: string;
                    readonly postalCode: string;
                    readonly postalCodeSuffix: string | null | undefined;
                    readonly coordinates: {
                        readonly lat: number;
                        readonly lng: number;
                    };
                };
                readonly colors: {
                    readonly main: string;
                    readonly hover: string;
                    readonly active: string;
                };
                readonly recurrenceRule: {
                    readonly interval: {
                        readonly quantityOfMonths: number;
                        readonly day: number;
                        readonly name: string;
                    } | {
                        readonly quantityOfYears: number;
                    } | {
                        readonly quantityOfWeeks: number;
                        readonly weekdays: readonly ("Monday" | "Tuesday" | "Wednesday" | "Thursday" | "Friday" | "Saturday" | "Sunday")[];
                    } | {
                        readonly quantityOfDays: number;
                    };
                    readonly recurrenceBegins: string;
                    readonly recurrenceEnds: string | number | null | undefined;
                    readonly cancelledInstances: readonly string[] | null | undefined;
                    readonly additionalInstances: readonly string[] | null | undefined;
                } | null | undefined;
            };
            readonly lastTechs: readonly (string | {
                readonly active: boolean;
                readonly name: string;
                readonly id: string;
                readonly imageUrl: string | null | undefined;
                readonly phones: readonly {
                    readonly number: string;
                    readonly main: boolean;
                    readonly phoneType: string;
                    readonly canText: boolean;
                    readonly canCall: boolean;
                }[];
                readonly role: string;
                readonly title: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
                readonly email: {
                    readonly canEmail: boolean;
                    readonly emailString: string;
                };
                readonly address: string;
                readonly username: string;
                readonly route: string | import("../types/bindings").RouteEncoded;
                readonly ratePerHour: number;
                readonly isTechnician: boolean;
                readonly isSalesRep: boolean;
                readonly description: string | null | undefined;
                readonly linkedinUrl: string | null | undefined;
                readonly attendance: readonly string[];
                readonly settings: {
                    readonly appointmentNotifications: {
                        readonly personalScheduleChangeNotifications: string;
                        readonly allScheduleChangeNotifications: string;
                    } | null | undefined;
                    readonly commissions: {
                        readonly technician: string;
                        readonly salesRep: string;
                    } | null | undefined;
                    readonly scheduleSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly daysPerWeek: number;
                        readonly visibleRoutes: readonly string[];
                        readonly detailedCards: boolean;
                    };
                    readonly accountOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly serviceOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly appointmentOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly leadOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly packageOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly productOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly orderOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly taxRateOverviewSettings: {
                        readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                        readonly cardOrRow: "Card" | "Table";
                        readonly perPage: number;
                        readonly columnConfigs: readonly {
                            readonly heading: string;
                            readonly dataPath: {
                                readonly path: readonly string[];
                                readonly formatter: string | null | undefined;
                            };
                        }[];
                    };
                    readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
                };
            })[];
            readonly package: readonly (string | {
                readonly id: string;
                readonly date: string;
            })[] | null | undefined;
            readonly promotion: readonly (string | {
                readonly id: string;
                readonly date: string;
            })[] | null | undefined;
            readonly balance: string;
            readonly due: string;
            readonly total: string;
            readonly billedItems: readonly {
                readonly item: string | {
                    readonly active: boolean;
                    readonly name: string;
                    readonly id: string;
                    readonly quickCode: string;
                    readonly group: string | null | undefined;
                    readonly subgroup: string | null | undefined;
                    readonly unit: string | null | undefined;
                    readonly commission: boolean;
                    readonly favorite: boolean;
                    readonly defaults: {
                        readonly description: string;
                        readonly price: number;
                    };
                } | {
                    readonly active: boolean;
                    readonly name: string;
                    readonly id: string;
                    readonly quickCode: string;
                    readonly group: string | null | undefined;
                    readonly subgroup: string | null | undefined;
                    readonly unit: string | null | undefined;
                    readonly commission: boolean;
                    readonly favorite: boolean;
                    readonly defaults: {
                        readonly description: string;
                        readonly price: number;
                    };
                    readonly averageTime: string | null | undefined;
                };
                readonly quantity: number;
                readonly taxed: boolean;
                readonly upsale: boolean;
            }[];
            readonly discount: string;
            readonly tip: string;
        };
    }, never>>>;
    activity: Schema.propertySignature<Schema.Array$<Schema.SchemaClass<Did, {
        readonly id: string;
        readonly createdAt: string;
        readonly metadata: string | null | undefined;
        readonly in: string | {
            readonly active: boolean;
            readonly name: string;
            readonly id: string;
            readonly imageUrl: string | null | undefined;
            readonly phones: readonly {
                readonly number: string;
                readonly main: boolean;
                readonly phoneType: string;
                readonly canText: boolean;
                readonly canCall: boolean;
            }[];
            readonly role: string;
            readonly title: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology";
            readonly email: {
                readonly canEmail: boolean;
                readonly emailString: string;
            };
            readonly address: string;
            readonly username: string;
            readonly route: string | import("../types/bindings").RouteEncoded;
            readonly ratePerHour: number;
            readonly isTechnician: boolean;
            readonly isSalesRep: boolean;
            readonly description: string | null | undefined;
            readonly linkedinUrl: string | null | undefined;
            readonly attendance: readonly string[];
            readonly settings: {
                readonly appointmentNotifications: {
                    readonly personalScheduleChangeNotifications: string;
                    readonly allScheduleChangeNotifications: string;
                } | null | undefined;
                readonly commissions: {
                    readonly technician: string;
                    readonly salesRep: string;
                } | null | undefined;
                readonly scheduleSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly daysPerWeek: number;
                    readonly visibleRoutes: readonly string[];
                    readonly detailedCards: boolean;
                };
                readonly accountOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly serviceOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly appointmentOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly leadOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly packageOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly productOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly orderOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly taxRateOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
            };
        } | {
            readonly id: string;
            readonly role: "Technician" | "SalesRepresentative" | "HumanResources" | "InformationTechnology" | "Administrator";
            readonly email: string | null | undefined;
            readonly settings: {
                readonly appointmentNotifications: {
                    readonly personalScheduleChangeNotifications: string;
                    readonly allScheduleChangeNotifications: string;
                } | null | undefined;
                readonly commissions: {
                    readonly technician: string;
                    readonly salesRep: string;
                } | null | undefined;
                readonly scheduleSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly daysPerWeek: number;
                    readonly visibleRoutes: readonly string[];
                    readonly detailedCards: boolean;
                };
                readonly accountOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly serviceOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly appointmentOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly leadOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly packageOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly productOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly orderOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly taxRateOverviewSettings: {
                    readonly rowHeight: "ExtraSmall" | "Small" | "Medium" | "Large";
                    readonly cardOrRow: "Card" | "Table";
                    readonly perPage: number;
                    readonly columnConfigs: readonly {
                        readonly heading: string;
                        readonly dataPath: {
                            readonly path: readonly string[];
                            readonly formatter: string | null | undefined;
                        };
                    }[];
                };
                readonly homePage: "SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome";
            };
            readonly password: string | null | undefined;
            readonly firstName: string;
            readonly lastName: string;
            readonly metadata: {
                readonly createdAt: string;
                readonly lastLogin: string | null | undefined;
                readonly isActive: boolean;
                readonly roles: readonly string[];
            } | null | undefined;
            readonly emailVerified: boolean;
            readonly verificationToken: string | null | undefined;
            readonly verificationExpires: string | null | undefined;
            readonly passwordResetToken: string | null | undefined;
            readonly passwordResetExpires: string | null | undefined;
            readonly permissions: {
                readonly applications: readonly ("HumanResources" | "Sales" | "Accounting" | "Errand" | "Logistics" | "Marketing" | "Website")[];
                readonly pages: readonly ("SalesHomeDashboard" | "SalesHomeProducts" | "SalesHomeServices" | "SalesHomePackages" | "SalesHomeTaxRates" | "SalesLeadsOverview" | "SalesLeadsActivities" | "SalesLeadsCampaigns" | "SalesLeadsDripCampaigns" | "SalesLeadsOpportunities" | "SalesLeadsPromotions" | "SalesAccountsOverview" | "SalesAccountsActivities" | "SalesAccountsBilling" | "SalesAccountsContracts" | "SalesOrdersOverview" | "SalesOrdersActivities" | "SalesOrdersPayments" | "SalesOrdersCommissions" | "SalesSchedulingSchedule" | "SalesSchedulingAppointments" | "SalesSchedulingRecurring" | "SalesSchedulingRoutes" | "SalesSchedulingReminders" | "UserHome")[];
                readonly data: readonly ("Site" | "Employee" | "Route" | "Appointment" | "Product" | "Service" | "Account" | "Did" | "Lead" | "TaxRate" | "Company" | "User" | "Order" | "Payment" | "Package" | "Promotion" | "Represents" | "Ordered")[];
            };
        } | {
            readonly id: string;
            readonly salesRep: readonly import("../types/bindings").RepresentsEncoded[] | null | undefined;
            readonly phones: readonly {
                readonly number: string;
                readonly main: boolean;
                readonly phoneType: string;
                readonly canText: boolean;
                readonly canCall: boolean;
            }[];
            readonly email: {
                readonly canEmail: boolean;
                readonly emailString: string;
            };
            readonly tags: readonly string[];
            readonly colors: {
                readonly main: string;
                readonly hover: string;
                readonly active: string;
            };
            readonly taxRate: string | {
                readonly name: string;
                readonly id: string;
                readonly description: string;
                readonly isActive: boolean;
                readonly taxAgency: string;
                readonly zip: number;
                readonly city: string;
                readonly county: string;
                readonly state: string;
                readonly taxComponents: {
                    readonly [x: string]: number;
                };
            };
            readonly site: string | {
                readonly id: string;
                readonly addressLine1: string;
                readonly addressLine2: string | null | undefined;
                readonly sublocalityLevel1: string | null | undefined;
                readonly locality: string;
                readonly administrativeAreaLevel3: string | null | undefined;
                readonly administrativeAreaLevel2: string | null | undefined;
                readonly administrativeAreaLevel1: string;
                readonly country: string;
                readonly postalCode: string;
                readonly postalCodeSuffix: string | null | undefined;
                readonly coordinates: {
                    readonly lat: number;
                    readonly lng: number;
                };
            };
            readonly orders: readonly import("../types/bindings").OrderedEncoded[];
            readonly activity: readonly import("../types/bindings").DidEncoded[];
            readonly customFields: readonly (readonly [string, string])[];
            readonly accountName: {
                readonly firstName: string;
                readonly lastName: string;
            } | {
                readonly companyName: string;
            };
            readonly sector: "Residential" | "Commercial";
            readonly memo: string | null | undefined;
            readonly leadSource: string;
            readonly needsReview: boolean;
            readonly hasAlert: boolean;
            readonly accountType: string;
            readonly subtype: string;
            readonly isTaxExempt: boolean;
            readonly paymentTerms: string;
            readonly dateAdded: string;
        };
        readonly out: string | import("../types/bindings").TargetEncoded;
        readonly activityType: {
            readonly durationSeconds: number | null | undefined;
            readonly source: string | null | undefined;
        } | {
            readonly recipient: string | null | undefined;
            readonly method: string | null | undefined;
        } | {
            readonly amount: string | null | undefined;
            readonly currency: string | null | undefined;
            readonly paymentMethod: string | null | undefined;
        } | {
            readonly comment: string;
            readonly replyTo: string | null | undefined;
        } | {
            readonly fieldName: string;
            readonly oldValue: string | null | undefined;
            readonly newValue: string | null | undefined;
        } | {
            readonly initialData: string | null | undefined;
        };
    }, never>>>;
    customFields: Schema.propertySignature<Schema.Array$<Schema.Tuple2<Schema.filter<typeof Schema.String>, Schema.filter<typeof Schema.String>>>>;
    accountName: Schema.propertySignature<Schema.Union<[typeof import("../types/bindings").CompanyName, typeof import("../types/bindings").PersonName]>>;
    sector: Schema.propertySignature<Schema.Union<[Schema.Literal<["Residential"]>, Schema.Literal<["Commercial"]>]>>;
    memo: Schema.OptionFromNullishOr<Schema.filter<typeof Schema.String>>;
    phones: Schema.propertySignature<Schema.Array$<typeof PhoneNumber>>;
    email: Schema.propertySignature<typeof Email>;
    leadSource: Schema.propertySignature<Schema.filter<typeof Schema.String>>;
    colors: Schema.propertySignature<typeof Colors>;
    needsReview: Schema.propertySignature<typeof Schema.Boolean>;
    hasAlert: Schema.propertySignature<typeof Schema.Boolean>;
    accountType: Schema.propertySignature<Schema.filter<typeof Schema.String>>;
    subtype: Schema.propertySignature<Schema.filter<typeof Schema.String>>;
    isTaxExempt: Schema.propertySignature<typeof Schema.Boolean>;
    paymentTerms: Schema.propertySignature<Schema.filter<typeof Schema.String>>;
    tags: Schema.propertySignature<Schema.Array$<Schema.filter<typeof Schema.String>>>;
    dateAdded: Schema.propertySignature<typeof Schema.DateTimeUtc>;
}>, never, {
    readonly id: string;
} & {
    readonly salesRep: import("effect/Option").Option<readonly Represents[]>;
} & {
    readonly phones: readonly PhoneNumber[];
} & {
    readonly email: Email;
} & {
    readonly tags: readonly string[];
} & {
    readonly colors: Colors;
} & {
    readonly taxRate: string | TaxRate;
} & {
    readonly site: string | Site;
} & {
    readonly orders: readonly Ordered[];
} & {
    readonly activity: readonly Did[];
} & {
    readonly customFields: readonly (readonly [string, string])[];
} & {
    readonly accountName: import("../types/bindings").PersonName | import("../types/bindings").CompanyName;
} & {
    readonly sector: "Residential" | "Commercial";
} & {
    readonly memo: import("effect/Option").Option<string>;
} & {
    readonly leadSource: string;
} & {
    readonly needsReview: boolean;
} & {
    readonly hasAlert: boolean;
} & {
    readonly accountType: string;
} & {
    readonly subtype: string;
} & {
    readonly isTaxExempt: boolean;
} & {
    readonly paymentTerms: string;
} & {
    readonly dateAdded: import("effect/DateTime").Utc;
}, {}, {}>;
export declare class Account extends Account_base {
}
export {};
