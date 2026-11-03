import type { UserSchema } from '$bindings';
import { api } from './request';

const usersPath = '/users';
const userMePath = `${usersPath}/me`;
// const userPath = (id: string) => `${usersPath}/${id}`;

export const users = {
	async me() {
		return await api.get<UserSchema>(userMePath);
	}
};
