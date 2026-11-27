import { appState } from '#lib/state.svelte.ts';
import type { LoginSchema, RefreshSchema } from '$bindings';
import { fetch } from './transport';

let refreshPromise: Promise<void> | null = null;

export async function ensureAuthenticated() {
	if (!appState.accessToken) {
		refreshPromise ??= auth.refresh().finally(() => {
			refreshPromise = null;
		});
		await refreshPromise;
	}

	if (!appState.accessToken) {
		throw new Error('Unauthorized');
	}
}

export const auth = {
	async login(email: string, password: string) {
		try {
			const credentials = btoa(`${email}:${password}`);
			const response = await fetch<LoginSchema>('/auth/login', {
				method: 'POST',
				headers: {
					Authorization: `Basic ${credentials}`
				}
			});
			appState.accessToken = response.accessToken;
		} catch (error) {
			appState.accessToken = null;
			throw error;
		}
	},
	async refresh() {
		try {
			const response = await fetch<RefreshSchema>(`/auth/refresh`, {
				method: 'POST'
			});
			appState.accessToken = response.accessToken;
		} catch (error) {
			appState.accessToken = null;
			throw error;
		}
	},
	async logout() {
		try {
			await fetch(`/auth/logout`, {
				method: 'POST'
			});
		} catch (error) {
			appState.accessToken = null;
			throw error;
		}
	}
};
