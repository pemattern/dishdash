import { ensureAuthenticated } from '#lib/api/auth.ts';
import { routes } from '#lib/const/routes.ts';
import { resolve } from '$app/paths';
import { goto } from '$app/navigation';

export const load = async () => {
	try {
		await ensureAuthenticated();
	} catch {
		await goto(resolve(routes.login));
		return;
	}
};
