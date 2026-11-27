import type { UnitResponse } from '$bindings';
import { methods } from './client';

const unitsPath = '/units';

export const units = {
	getAll: () => methods.get<UnitResponse[]>(`${unitsPath}`)
};
