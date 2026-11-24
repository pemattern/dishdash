<script lang="ts">
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

<div class="flex flex-col">
	{#if label}
		<label for={props.id} class="mb-1 text-sm font-bold text-dark-gray">{label}</label>
	{/if}
	<input
		bind:this={input}
		bind:value
		class="rounded-md border border-light-orange px-3 py-2 text-gray hover:bg-off-white"
		{...props}
	/>
</div>
