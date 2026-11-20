import type { LayoutLoad } from './$types';
import { setFetch } from '#lib/api/client.ts';

export const ssr = false;
export const csr = true;

export const load: LayoutLoad = async ({ fetch }) => {
	setFetch(fetch);
};
