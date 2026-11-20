import { appState } from '#lib/state.svelte.ts';
import type { LoginSchema, RefreshSchema } from '$bindings';
import { request } from './client';

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
			const response = await request<LoginSchema>(
				'/auth/login',
				{
					method: 'POST',
					headers: {
						Authorization: `Basic ${credentials}`
					}
				},
				true
			);
			appState.accessToken = response.accessToken;
		} catch (error) {
			appState.accessToken = null;
			throw error;
		}
	},

	async refresh() {
		try {
			const response = await request<RefreshSchema>(
				`/auth/refresh`,
				{
					method: 'POST'
				},
				true
			);
			appState.accessToken = response.accessToken;
		} catch (error) {
			appState.accessToken = null;
			throw error;
		}
	},

	async logout() {
		try {
			await request(
				`/auth/logout`,
				{
					method: 'POST'
				},
				true
			);
		} catch (error) {
			appState.accessToken = null;
			throw error;
		}
	}
};
