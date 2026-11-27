import { ensureAuthenticated } from '#lib/api/auth.ts';
import { routes } from '#lib/const/routes.ts';
import { resolve } from '$app/paths';
import { goto } from '$app/navigation';
import { appState } from '#lib/state.svelte.ts';

export const load = async () => {
	try {
		await ensureAuthenticated();
		await appState.init();
	} catch {
		await goto(resolve(routes.login));
		return;
	}
};
