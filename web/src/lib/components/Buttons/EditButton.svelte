<script lang="ts">
	import { LucidePencil } from '@lucide/svelte';
	import ContextButton from './ContextButton.svelte';

	const { onclick = () => {} }: { onclick?: () => void } = $props();
</script>

<ContextButton {onclick}>
	<LucidePencil size="18" />
</ContextButton>
