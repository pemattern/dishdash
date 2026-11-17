<script lang="ts">
	let { label = '', value = $bindable(''), focus = false, ...props } = $props();

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
		<label class="pb-1 text-sm font-bold text-dark-gray" for="name">
			{label}
		</label>
	{/if}
	<input bind:this={input} bind:value class="rounded-md border border-gray px-3 py-2" {...props} />
</div>
