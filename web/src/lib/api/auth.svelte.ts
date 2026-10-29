type LoginResponse = {
	accessToken: string;
};

type RefreshResponse = {
	accessToken: string;
};

let accessToken = $state<string | null>(null);

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
			const data: LoginResponse = await response.json();
			accessToken = data.accessToken;
		} catch {
			accessToken = null;
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
			const data: RefreshResponse = await response.json();
			accessToken = data.accessToken;
		} catch {
			accessToken = null;
		}
	},

	async logout() {
		try {
			await fetch(`/api/auth/logout`, {
				method: 'POST'
			});
		} finally {
			accessToken = null;
		}
	},

	get accessToken() {
		return accessToken;
	}
};
