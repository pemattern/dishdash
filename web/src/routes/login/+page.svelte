<script lang="ts">
	import { auth } from '#lib/api/auth.svelte.ts';
	import Input from '#lib/components/Input.svelte';
	import { routes } from '#lib/const/routes.ts';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { onMount } from 'svelte';

	onMount(() => {
		void checkAuth();
	});

	async function checkAuth() {
		await auth.refresh();
		await goto(resolve(routes.home));
	}
	let email = $state('');
	let password = $state('');
	let error = $state('');

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		error = '';

		try {
			await auth.login(email, password);
			await goto(resolve(routes.home));
		} catch (err) {
			error = err instanceof Error ? err.message : 'Something went wrong';
		}
	}
</script>

<main>
	<div
		class="h-full w-full bg-[url('data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20width=%2220%22%20height=%2220%22%3E%3Cpath%20d=%22M0%200%20L20%200%22%20stroke=%22%23eee%22%20stroke-width=%221%22/%3E%3Cpath%20d=%22M0%200%20L0%2020%22%20stroke=%22%23eee%22%20stroke-width=%221%22/%3E%3C/svg%3E')]"
	>
		<div class="h-full w-full bg-radial from-transparent to-white">
			<div class="flex h-screen items-center justify-center">
				<form
					class="flex flex-col gap-y-4 rounded-lg bg-white p-4 shadow-xl"
					onsubmit={handleSubmit}
				>
					<Input name="email" type="text" placeholder="E-Mail" bind:value={email} />
					<Input name="password" type="password" placeholder="Password" bind:value={password} />
					<div class="flex justify-end text-xs text-dark-blue hover:underline">
						<a href={resolve(routes.home)} class="rounded-sm">Forgot your password?</a>
					</div>
					<input
						class="cursor-pointer rounded-sm bg-blue py-2 text-dark-gray hover:bg-light-blue"
						type="submit"
						value="Submit"
					/>
					{#if error}
						<p>{error}</p>
					{/if}
				</form>
			</div>
		</div>
	</div>
</main>
