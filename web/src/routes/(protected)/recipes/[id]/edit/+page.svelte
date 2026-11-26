<script lang="ts">
	import RecipeForm from '#lib/components/Forms/RecipeForm.svelte';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<RecipeForm recipe={data.recipe} />
