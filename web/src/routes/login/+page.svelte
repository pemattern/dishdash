<script lang="ts">
	import { api } from '#lib/api/instance.ts';
	import Input from '#lib/components/Input.svelte';
	import Logo from '#lib/components/Logo.svelte';
	import { routes } from '#lib/const/routes.ts';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';

	let email = $state('');
	let password = $state('');
	let error = $state('');

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		error = '';
		try {
			await api.auth.login(email, password);
			await goto(resolve(routes.home));
		} catch (err) {
			console.log(error);
			error = err instanceof Error ? err.message : 'Something went wrong';
		}
	}
</script>

<main>
	<div class="h-full w-full bg-radial from-lightest-green to-white">
		<div class="flex h-screen flex-col items-center justify-center">
			<Logo />
			<form class="flex flex-col gap-y-4 rounded-lg bg-white p-4 shadow-xl" onsubmit={handleSubmit}>
				<Input name="email" type="text" placeholder="E-Mail" bind:value={email} />
				<Input name="password" type="password" placeholder="Password" bind:value={password} />
				<div class="-my-2 flex justify-end text-xs text-dark-orange hover:underline">
					<a href={resolve(routes.home)} class="rounded-sm">Forgot your password?</a>
				</div>
				<input
					class="cursor-pointer rounded-sm bg-light-orange py-2 font-bold text-dark-orange inset-ring inset-ring-orange hover:bg-white hover:text-orange hover:inset-ring-light-orange"
					type="submit"
					value="Login"
				/>
				{#if error}
					<p>{error}</p>
				{/if}
			</form>
		</div>
	</div>
</main>
