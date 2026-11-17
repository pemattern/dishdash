<script>
	let { label = '', placeholder = '', value = $bindable(''), name = undefined } = $props();
</script>

<div class="flex flex-col">
	{#if label}
		<label class="pb-1 text-sm font-bold text-dark-gray" for="name">
			{label}
		</label>
	{/if}
	<textarea
		id={name}
		{name}
		{placeholder}
		bind:value
		rows={4}
		class="resize-none rounded-md border border-gray px-3 py-2"></textarea>
</div>
