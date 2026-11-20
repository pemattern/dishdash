<script lang="ts">
	import { api } from '#lib/api/instance.ts';
	import FormModal from '#lib/components/FormModal.svelte';
	import Input from '#lib/components/Input.svelte';
	import { routes } from '#lib/const/routes.ts';
	import { refreshAll } from '$app/navigation';
	import { resolve } from '$app/paths';
	import type { PostRecipeSchema } from '$bindings';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let open = $state(false);
	let name = $state('');
	let error = $state('');

	$effect(() => {
		if (!open) {
			resetForm();
		}
	});

	function resetForm() {
		name = '';
		error = '';
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		error = '';

		const schema: PostRecipeSchema = {
			name
		};

		try {
			await api.recipes.create(schema);
			await refreshAll();
			open = false;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Something went wrong';
		}
	}
</script>

<h1 class="text-4xl">Recipes</h1>

<button class="m-4 rounded-md bg-light-blue p-4 font-sans" onclick={() => (open = true)}
	>Create</button
>

{#each data.recipes as recipe (recipe.id)}
	<article>
		<h2>
			<a href={resolve(routes.recipe, { id: recipe.id })}>
				{recipe.name}
			</a>
		</h2>
	</article>
{/each}

<FormModal bind:open {handleSubmit}>
	<Input label="Title" bind:value={name} name="name" type="text" focus={open} />
	{error}
</FormModal>
