import { auth } from './auth.svelte';
import { api } from './request';

export type GetUserResponse = {
	id: string;
	email: string;
	createdAt: string;
	updatedAt: string;
};

export const users = {
	async me() {
		console.log(`Bearer ${auth.accessToken}`);
		return await api.get<GetUserResponse>('/users/me');
	}
};
