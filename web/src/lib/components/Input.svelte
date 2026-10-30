<script>
	let {
		label = '',
		placeholder = '',
		type = 'text',
		value = $bindable(''),
		name = undefined
	} = $props();
</script>

<div class="flex flex-col">
	{#if label}
		<label class="text-sm font-bold text-dark-gray" for="name">
			{label}
		</label>
	{/if}
	<input
		id={name}
		{name}
		{type}
		{placeholder}
		bind:value
		class="rounded-md border border-gray px-3 py-2"
	/>
</div>
