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
			class="rounded-md px-4 py-2 text-dark-orange inset-ring inset-ring-light-orange hover:cursor-pointer hover:bg-off-white hover:inset-ring-orange"
			onclick={logout}
		>
			Logout
		</button>
	</div>
</header>
