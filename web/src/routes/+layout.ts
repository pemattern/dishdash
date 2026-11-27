import type { LayoutLoad } from './$types';
import { setFetch } from '#lib/api/transport.ts';

export const ssr = false;
export const csr = true;

export const load: LayoutLoad = async ({ fetch }) => {
	setFetch(fetch);
};
