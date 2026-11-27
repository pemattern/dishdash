import { api } from './api/instance';
import type { Unit } from './types/unit';

type AppState = {
	accessToken: string | null;
	units: Unit[] | null;
};

let state = $state<AppState>({
	accessToken: null,
	units: null
});

export const appState = {
	get accessToken() {
		return state.accessToken;
	},

	set accessToken(value) {
		state = {
			...state,
			accessToken: value
		};
	},

	get units() {
		if (!state.units) {
			throw new Error('not laoded');
		}
		return state.units;
	},

	async init() {
		const units = await api.units.getAll();
		state = {
			...state,
			units
		};
	}
};
