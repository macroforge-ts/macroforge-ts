import { getRustItems, getVersion } from '$lib/server/api-docs';

export async function load() {
	// Pull specific API items from generated docs
	const items = getRustItems('macroforge_ts', [
		'expand_sync',
		'ExpandResult',
		'MacroDiagnostic',
		'SourceMappingResult'
	]);

	return {
		version: getVersion('rust', 'macroforge_ts'),
		// Items from doc comments - can be used for signatures, params, etc.
		api: items
	};
}
