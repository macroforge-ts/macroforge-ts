import { getRustItems, getVersion } from '$lib/server/api-docs.ts';

export async function load() {
    return {
        version: getVersion('rust', 'macroforge_ts'),
        api: getRustItems('macroforge_ts', ['transform_sync', 'TransformResult'])
    };
}
