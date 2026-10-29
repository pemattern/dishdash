<script>
	import FormModal from '#lib/components/FormModal.svelte';

	let open = $state(true);
</script>

<FormModal bind:open title="Create" handleSubmit={() => (open = false)}>
	<p>Hello World</p>
</FormModal>
