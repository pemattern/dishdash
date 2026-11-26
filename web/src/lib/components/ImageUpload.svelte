<script lang="ts">
	import * as m from '#lib/paraglide/messages.js';
	import EditButton from './Buttons/EditButton.svelte';
	import Label from './Label.svelte';
	import RemoveButton from './Buttons/RemoveButton.svelte';
	import { LucideUpload } from '@lucide/svelte';

	type Props = {
		maxSize?: number;
		accept?: string;
		onUpload?: (file: File) => void;
		required?: boolean;
		label?: string;
	};

	let {
		maxSize: maxFileSize = 5,
		accept = 'image/png,image/jpeg,image/webp',
		onUpload = () => {},
		required = false,
		label
	}: Props = $props();

	let file = $state<File | null>(null);
	let imageUrl = $state('');
	let error = $state('');
	let imageInput: HTMLInputElement;

	function addFile(selectedFile: File | null) {
		if (!selectedFile) return;

		error = '';

		if (!selectedFile.type.startsWith('image/')) return;

		if (selectedFile.size > maxFileSize * 1024 * 1024) {
			error = m.maxFileSize({ maxFileSize });
			return;
		}

		if (imageUrl) URL.revokeObjectURL(imageUrl);

		file = selectedFile;
		imageUrl = URL.createObjectURL(selectedFile);

		onUpload(selectedFile);
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		addFile(event.dataTransfer?.files[0] ?? null);
	}

	function handleChange(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		addFile(input.files?.item(0) ?? null);
		input.value = '';
	}

	function openDialog() {
		imageInput.click();
	}

	function removeFile() {
		if (imageUrl) URL.revokeObjectURL(imageUrl);

		file = null;
		imageUrl = '';
		error = '';
		imageInput.value = '';
	}

	$effect(() => {
		return () => {
			if (imageUrl) URL.revokeObjectURL(imageUrl);
		};
	});
</script>

<div class="flex flex-col">
	{#if label}
		<Label {required} for="fileInput">{label}</Label>
	{/if}

	<div
		class="group relative aspect-video w-full overflow-hidden rounded-xl border-dashed border-light-orange"
		class:border-2={!imageUrl}
	>
		<input
			bind:this={imageInput}
			id="fileInput"
			type="file"
			{accept}
			onchange={handleChange}
			class="hidden"
		/>

		{#if imageUrl}
			<img src={imageUrl} alt={file?.name ?? 'Uploaded image'} class="size-full object-cover" />

			<div class="absolute top-3 right-3 z-10 flex gap-3">
				<EditButton onclick={openDialog} />
				<RemoveButton onclick={removeFile} />
			</div>
		{:else}
			<button
				type="button"
				class="flex size-full cursor-pointer flex-col items-center justify-center gap-3 p-10 text-center text-orange hover:bg-off-white"
				ondrop={handleDrop}
				ondragover={(event) => event.preventDefault()}
				onclick={openDialog}
			>
				<LucideUpload />

				<div class="space-y-1">
					<p class="text-sm font-semibold">Drop your image here</p>
					<p class="text-sm">
						or <span class="font-semibold">browse files</span>
					</p>
				</div>

				<p class="text-xs">
					PNG, JPG or WebP · Max {maxFileSize} MB
				</p>
			</button>
		{/if}

		{#if error}
			<p class="text-red-600 text-sm">{error}</p>
		{/if}
	</div>
</div>
