<script lang="ts">
	let { children } = $props();
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import HeaderBar from '#lib/components/HeaderBar.svelte';
	import { auth } from '#lib/api/auth.svelte.ts';
	import { routes } from '#lib/const/routes.ts';
	import Navbar from '#lib/components/Navbar.svelte';

	onMount(() => {
		void checkAuth();
	});

	async function checkAuth() {
		await auth.refresh();
		if (auth.user) {
			return;
		}
		await goto(resolve(routes.login));
	}
</script>

{#if !auth.user}
	<div>Checking your session…</div>
{:else if auth.user}
	<div class="flex bg-light-gray">
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
