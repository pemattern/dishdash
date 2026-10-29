<script lang="ts">
	import { auth } from '#lib/api/auth.svelte.ts';
	import { routes } from '#lib/const/routes.ts';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	async function logout() {
		await auth.logout();
		await goto(resolve(routes.home));
	}
</script>

<header class="flex pt-2 pr-4 pl-4">
	<div class="flex flex-1/4 justify-end">
		<button class="rounded-md bg-blue px-4 py-2" onclick={logout}> Logout </button>
	</div>
</header>
