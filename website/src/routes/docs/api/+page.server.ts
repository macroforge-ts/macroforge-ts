import { loadRustCrate, getVersion } from '$lib/server/api-docs';

export async function load() {
	const crate = loadRustCrate('macroforge_ts');

	return {
		version: getVersion('rust', 'macroforge_ts'),
		// Get counts for display
		stats: {
			functions: crate?.items.filter((i) => i.kind === 'function').length ?? 0,
			structs: crate?.items.filter((i) => i.kind === 'struct').length ?? 0,
			total: crate?.items.length ?? 0
		}
	};
}
