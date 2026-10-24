type User = {
	id: string;
	email: string;
};

type LoginResponse = {
	accessToken: string;
	user: User;
};

let accessToken = $state<string | null>(null);
let user = $state<User | null>(null);
let loading = $state(false);

export const auth = {
	get user() {
		return user;
	},

	get isAuthenticated() {
		return !!accessToken;
	},

	get loading() {
		return loading;
	},

	async login(email: string, password: string) {
		loading = true;

		try {
			const credentials = btoa(`${email}:${password}`);
			const response = await fetch('/api/auth/login', {
				method: 'POST',
				headers: {
					Authorization: `Basic ${credentials}`
				},
				credentials: 'include'
			});
			if (!response.ok) {
				throw new Error(response.status === 401 ? 'Invalid email or password' : 'Login failed');
			}
			const data: LoginResponse = await response.json();
			accessToken = data.accessToken;
			user = data.user;
		} finally {
			loading = false;
		}
	},

	async logout() {
		try {
			await fetch(`/api/auth/logout`, {
				method: 'POST',
				credentials: 'include'
			});
		} finally {
			accessToken = null;
			user = null;
		}
	},

	getAccessToken() {
		return accessToken;
	}
};
