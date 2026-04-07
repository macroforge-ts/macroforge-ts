import { getCliDocs, getVersion } from '$lib/server/api-docs';

export async function load() {
    return {
        version: getVersion('rust', 'macroforge_ts'),
        cli: getCliDocs()
    };
}
