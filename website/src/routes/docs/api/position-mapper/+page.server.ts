import { getRustItems, getVersion } from '$lib/server/api-docs';

export async function load() {
    return {
        version: getVersion('rust', 'macroforge_ts'),
        api: getRustItems('macroforge_ts', [
            'NativePositionMapper',
            'NativeMapper',
            'SpanResult',
            'is_empty',
            'original_to_expanded',
            'expanded_to_original',
            'is_in_generated',
            'generated_by',
            'map_span_to_original',
            'map_span_to_expanded'
        ])
    };
}
