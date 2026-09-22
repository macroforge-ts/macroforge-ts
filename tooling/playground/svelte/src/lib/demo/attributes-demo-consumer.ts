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

/** Calls an export by name, or `null` when `@cfg` stripped it. */
function probeExport(name: string): string | null {
    const value: unknown = Reflect.get(attrs, name);
    return typeof value === 'function' ? String(value()) : null;
}

export function collectAttributesDemo(): AttributesDemoResults {
    return {
        keptByFeature: probeExport('keptByPlayground'),
        strippedByFeature: probeExport('strippedByMissingFeature'),
        keptByTarget: probeExport('keptByWebTarget'),
        strippedByTarget: probeExport('strippedByNodeTarget'),
        deprecatedCall: attrs.renderV1(),
        nonExhaustiveValue: attrs.exampleStatus
    };
}
