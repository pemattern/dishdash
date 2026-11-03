type AppState = {
	accessToken: string | null;
};

let state = $state<AppState>({
	accessToken: null
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
	}
};
