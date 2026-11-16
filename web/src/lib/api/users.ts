import type { UserSchema } from '$bindings';
import { methods } from './client';

const usersPath = '/users';

export const users = {
	me: () => methods.get<UserSchema>(`${usersPath}/me`)
};
