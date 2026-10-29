<script lang="ts">
	import { auth } from '#lib/api/auth.svelte.ts';
	import Input from '#lib/components/Input.svelte';
	import Logo from '#lib/components/Logo.svelte';
	import { routes } from '#lib/const/routes.ts';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { onMount } from 'svelte';

	onMount(async () => {
		await auth.refresh();
		if (auth.accessToken) {
			await goto(resolve(routes.home), { replace: true });
		}
	});

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
		<div class="flex h-screen flex-col items-center justify-center">
			<Logo />
			<form class="flex flex-col gap-y-4 rounded-lg bg-white p-4 shadow-xl" onsubmit={handleSubmit}>
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
</main>
