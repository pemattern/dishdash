<script lang="ts">
	import Label from './Label.svelte';

	let { value = $bindable(''), focus = false, label = '', ...props } = $props();

	let input: HTMLInputElement;

	$effect(() => {
		if (focus) {
			requestAnimationFrame(() => {
				input?.focus();
			});
		}
	});
</script>

<div class="flex min-w-0 flex-1 flex-col">
	{#if label}
		<Label for={props.id}>{label}</Label>
	{/if}
	<input
		bind:this={input}
		bind:value
		class="w-full min-w-0 rounded-md border border-light-orange px-3 py-2 text-gray hover:bg-off-white"
		{...props}
	/>
</div>
