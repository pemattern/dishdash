type User = {
	id: string;
	email: string;
};

type LoginResponse = {
  accessToken: string;
  user: User;
};

type RefreshResponse = {
  accessToken: string;
  user: User;
}

let accessToken = $state<string | null>(null);
let user = $state<User | null>(null);

export const auth = {
	get user() {
		return user;
	},

	async login(email: string, password: string) {
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
    } catch {
      user = null;
      accessToken = null;
		}
  },

  async refresh() {
    try {
      const response = await fetch(`/api/auth/refresh`, {
        method: 'POST',
        credentials: 'include',
      });
      if (!response.ok) {
				throw new Error(response.status === 401 ? 'Invalid refresh token' : 'Refresh failed');
			}
			const data: RefreshResponse = await response.json();
			accessToken = data.accessToken;
			user = data.user;
    } catch {
      user = null;
      accessToken = null;
    }
  },

	async logout() {
		try {
			await fetch(`/api/auth/logout`, {
				method: 'POST',
        credentials: 'include',
			});
    } finally {
			accessToken = null;
			user = null;
		}
  },

  async me() {
    try {
      const response = await fetch(`/api/users/me`, {
        method: 'GET',
        headers: {
					Authorization: `Bearer ${accessToken}`
				},
        credentials: 'include',
      });
      if (!response.ok) {
				throw new Error(response.status === 401 ? 'Invalid bearer token' : 'request failed');
			}
			const data: User = await response.json();
      user = data;
    } catch {
      user = null;
      accessToken = null;
    }
  },

	get accessToken() {
		return accessToken;
	}
};
