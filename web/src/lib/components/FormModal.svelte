<script lang="ts">
	import type { Snippet } from 'svelte';
	import Modal from './Modal.svelte';

	let {
		open = $bindable(),
		handleSubmit,
		isValid = () => true,
		children
	}: {
		open: boolean;
		handleSubmit: (event: SubmitEvent) => void;
		isValid?: () => boolean;
		children: Snippet;
	} = $props();
</script>

<Modal bind:open>
	<form onsubmit={handleSubmit} class="flex font-sans">
		<div class="flex flex-col gap-y-4">
			{@render children()}
			<input
				class="cursor-pointer rounded-sm bg-light-orange py-2 text-sm font-medium text-dark-orange inset-ring inset-ring-orange hover:bg-off-white hover:text-orange hover:inset-ring-light-orange"
				type="submit"
				value="Submit"
				disabled={!isValid()}
			/>
		</div>
	</form>
</Modal>
