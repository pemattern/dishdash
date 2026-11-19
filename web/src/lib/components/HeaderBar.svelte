<script lang="ts">
	import { api } from '#lib/api/instance.ts';
	import { routes } from '#lib/const/routes.ts';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import MasterSearch from './MasterSearch.svelte';

	async function logout() {
		await api.auth.logout();
		await goto(resolve(routes.login));
	}
</script>

<header class="flex flex-row pt-4 pr-4 pl-4">
	<div class="flex justify-center">
		<MasterSearch />
	</div>
	<div class="flex flex-1/4 justify-end">
		<button
			class="rounded-md bg-light-orange px-4 py-2 font-bold text-dark-orange"
			onclick={logout}
		>
			Logout
		</button>
	</div>
</header>
