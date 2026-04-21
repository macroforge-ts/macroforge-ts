// Consumer for `attributes-demo.ts`. Lives in a separate file because
// `@cfg`-stripped declarations vanish from their owning module — direct
// references by name would TDZ at runtime. A namespace import (`import *`)
// returns `undefined` for missing exports without throwing.

import * as attrs from './attributes-demo';

export type AttributesDemoResults = {
    keptByFeature: string | null;
    strippedByFeature: string | null;
    keptByTarget: string | null;
    strippedByTarget: string | null;
    deprecatedCall: string;
    nonExhaustiveValue: string;
};

export function collectAttributesDemo(): AttributesDemoResults {
    const dyn = attrs as Partial<typeof attrs>;
    return {
        keptByFeature: dyn.keptByPlayground?.() ?? null,
        strippedByFeature: dyn.strippedByMissingFeature?.() ?? null,
        keptByTarget: dyn.keptByWebTarget?.() ?? null,
        strippedByTarget: dyn.strippedByNodeTarget?.() ?? null,
        deprecatedCall: attrs.renderV1(),
        nonExhaustiveValue: attrs.exampleStatus
    };
}
