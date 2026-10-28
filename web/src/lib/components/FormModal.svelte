<script lang="ts">
	import type { Snippet } from 'svelte';
	import Modal from './Modal.svelte';

	let { open = $bindable(), title, handleSubmit, children }: { open: boolean, title: string, handleSubmit: () => void, children: Snippet } = $props();
</script>

<Modal bind:open {title}>
    <form onsubmit={handleSubmit}>
        <div class="space-y-4">
          {@render children()}
        </div>
    	<input
    		class="cursor-pointer rounded-sm bg-blue py-2 text-dark-gray hover:bg-light-blue"
    		type="submit"
    		value="Submit"
    	/>
     </form>
</Modal>
