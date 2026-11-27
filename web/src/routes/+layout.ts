import type { LayoutLoad } from './$types';
import { setFetch } from '#lib/api/client.ts';
import { appState } from '#lib/state.svelte.ts';

export const ssr = false;
export const csr = true;

export const load: LayoutLoad = async ({ fetch }) => {
	setFetch(fetch);
	await appState.init();
};
