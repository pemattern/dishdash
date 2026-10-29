<script lang="ts">
	let { children } = $props();
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import HeaderBar from '#lib/components/HeaderBar.svelte';
	import { auth } from '#lib/api/auth.svelte.ts';
	import { routes } from '#lib/const/routes.ts';
	import Navbar from '#lib/components/Navbar.svelte';

	let loading = $state(true);

	onMount(async () => {
		try {
			if (!auth.accessToken) {
				await auth.refresh();
			}

			if (!auth.accessToken) {
				await goto(resolve(routes.login), { replace: true });
				return;
			}
		} finally {
			loading = false;
		}
	});
</script>

{#if loading}
	<div>authenticating…</div>
{:else if auth.accessToken}
	<div class="flex">
		<Navbar />
		<div class="flex w-full flex-1 flex-col">
			<HeaderBar />
			<main class="m-4 rounded-md bg-white p-4 font-serif text-xl shadow-xl">
				{@render children()}
			</main>
		</div>
	</div>
	<Navbar />
{/if}
