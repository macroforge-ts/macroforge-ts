import { getRustItems, getVersion } from '$lib/server/api-docs';

export async function load() {
	return {
		version: getVersion('rust', 'macroforge_ts'),
		api: getRustItems('macroforge_ts', [
			'NativePlugin',
			'process_file',
			'get_mapper',
			'map_diagnostics',
			'log',
			'set_log_file'
		])
	};
}
