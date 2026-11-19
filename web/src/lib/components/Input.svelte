<script lang="ts">
	let { value = $bindable(''), focus = false, ...props } = $props();

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
	<input
		bind:this={input}
		bind:value
		class="rounded-md border border-light-green px-3 py-2 text-dark-green"
		{...props}
	/>
</div>
