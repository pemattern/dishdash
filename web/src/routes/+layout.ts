import type { LayoutLoad } from './$types';
import { setFetch } from '#lib/api/client.ts';
import { ensureAuthenticated } from '#lib/api/auth.ts';
import { routes } from '#lib/const/routes.ts';
import { resolve } from '$app/paths';
import { goto } from '$app/navigation';

export const ssr = false;
export const csr = true;

export const load: LayoutLoad = async ({ fetch, url }) => {
	setFetch(fetch);
	if (url.pathname === '/login') {
		return;
	}
	try {
		await ensureAuthenticated();
	} catch {
		await goto(resolve(routes.login));
		return;
	}
};
