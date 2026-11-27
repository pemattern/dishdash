type AppState = {
	accessToken: string | null;
	units: UnitResponse[] | null;
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
		return state.units;
	},

	async init() {
		const units = api.units.getAll();
		state = {
			...state,
			units
		};
	}
};
