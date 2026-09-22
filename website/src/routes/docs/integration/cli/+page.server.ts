import { getCliDocs, getVersion } from '$lib/server/api-docs.ts';

export async function load() {
    return {
        version: getVersion('rust', 'macroforge_ts'),
        cli: getCliDocs()
    };
}
