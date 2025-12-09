import { redirect } from '@sveltejs/kit';
import { base } from '$app/paths';

export function load() {
	redirect(302, `${base}/docs/getting-started`);
}
