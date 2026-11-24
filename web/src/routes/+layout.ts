import type { LayoutLoad } from './$types';
import { setFetch } from '#lib/api/client.ts';
import { setLocale } from '#lib/paraglide/runtime.js';

export const ssr = false;
export const csr = true;

export const load: LayoutLoad = async ({ fetch }) => {
	setFetch(fetch);
	setLocale('de');
};
