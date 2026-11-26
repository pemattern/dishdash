<script lang="ts">
	import { LucideTrash } from '@lucide/svelte';
	import ContextButton from './ContextButton.svelte';

	const { onclick = () => {} }: { onclick?: () => void } = $props();
</script>

<ContextButton {onclick}>
	<LucideTrash className="text-dark-orange" size="18" />
</ContextButton>
