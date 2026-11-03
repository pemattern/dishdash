import type { UserSchema } from '$bindings';
import { api } from './client';

const usersPath = '/users';

export const users = {
	me: () => api.get<UserSchema>(`${usersPath}/me`)
};
