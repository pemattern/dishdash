<script lang="ts">
	import type { Snippet } from 'svelte';
	import Modal from './Modal.svelte';

	let {
		open = $bindable(),
		title = undefined,
		handleSubmit,
		children
	}: {
		open: boolean;
		title?: string;
		handleSubmit: (event: SubmitEvent) => void;
		children: Snippet;
	} = $props();
</script>

<Modal bind:open {title}>
	<form onsubmit={handleSubmit} class="flex font-sans">
		<div class="flex flex-col gap-y-4">
			{@render children()}
			<input
				class="w-full cursor-pointer rounded-md bg-blue p-2 text-[18px] text-dark-gray hover:bg-light-blue"
				type="submit"
				value="Submit"
			/>
		</div>
	</form>
</Modal>
