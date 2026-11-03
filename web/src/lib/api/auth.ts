import { appState } from '#lib/state.svelte.ts';
import type { LoginSchema, RefreshSchema } from '$bindings';

export const auth = {
	async login(email: string, password: string) {
		try {
			const credentials = btoa(`${email}:${password}`);
			const response = await fetch('/api/auth/login', {
				method: 'POST',
				headers: {
					Authorization: `Basic ${credentials}`
				}
			});
			if (!response.ok) {
				throw new Error(response.status === 401 ? 'Invalid email or password' : 'Login failed');
			}
			const data: LoginSchema = await response.json();
			appState.accessToken = data.accessToken;
		} catch {
			appState.accessToken = null;
		}
	},

	async refresh() {
		try {
			const response = await fetch(`/api/auth/refresh`, {
				method: 'POST'
			});
			if (!response.ok) {
				throw new Error(response.status === 401 ? 'Invalid refresh token' : 'Refresh failed');
			}
			const data: RefreshSchema = await response.json();
			appState.accessToken = data.accessToken;
		} catch {
			appState.accessToken = null;
		}
	},

	async logout() {
		try {
			await fetch(`/api/auth/logout`, {
				method: 'POST'
			});
		} finally {
			appState.accessToken = null;
		}
	}
};
