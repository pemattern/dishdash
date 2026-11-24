<script lang="ts">
	import * as m from '#lib/paraglide/messages.js';

	type UploadedImage = {
		file: File;
		url: string;
	};

	type Props = {
		multiple?: boolean;
		maxSize?: number;
		accept?: string;
		onUpload?: (files: File[]) => void;
	};

	let {
		multiple = false,
		maxSize: maxFileSize = 5,
		accept = 'image/png,image/jpeg,image/webp',
		onUpload = () => {}
	}: Props = $props();

	let files = $state<UploadedImage[]>([]);
	let dragging = $state(false);
	let error = $state('');

	function addFiles(fileList: FileList | null) {
		if (!fileList) return;
		error = '';
		const selected = Array.from(fileList);
		let updated = [...files];
		if (!multiple) {
			updated.forEach((item) => URL.revokeObjectURL(item.url));
			updated = [];
		}
		for (const file of selected) {
			if (!file.type.startsWith('image/')) {
				continue;
			}
			if (file.size > maxFileSize * 1024 * 1024) {
				error = m.maxFileSize({ maxFileSize });
				continue;
			}
			if (!multiple && updated.length > 0) break;
			updated.push({
				file,
				url: URL.createObjectURL(file)
			});
			if (!multiple) break;
		}
		files = updated;
		onUpload(files.map((item) => item.file));
	}

	function removeFile(index: number) {
		URL.revokeObjectURL(files[index].url);
		files = files.filter((_, i) => i !== index);
		onUpload(files.map((item) => item.file));
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		dragging = false;
		addFiles(event.dataTransfer?.files ?? null);
	}

	function handleChange(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		addFiles(input.files);
		input.value = '';
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
		dragging = true;
	}

	function handleDragLeave() {
		dragging = false;
	}

	$effect(() => {
		return () => {
			files.forEach((item) => URL.revokeObjectURL(item.url));
		};
	});
</script>

<div class="w-full space-y-4">
	<label
		class="flex cursor-pointer flex-col items-center justify-center gap-3 rounded-xl border-2 border-dashed p-10 text-center text-orange hover:bg-off-white"
		class:border-orange={!dragging}
		class:bg-white={!dragging}
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		ondrop={handleDrop}
	>
		<input type="file" {accept} {multiple} onchange={handleChange} class="hidden" />

		<div
			class="bg-indigo-100 text-indigo-600 flex size-14 items-center justify-center rounded-full"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="size-7"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 7.5 12 3m0 0L7.5 7.5M12 3v13.5"
				/>
			</svg>
		</div>

		<div class="space-y-1">
			<p class="text-sm font-semibold">Drop your images here</p>
			<p class="text-sm">
				or <span class="font-semibold">browse files</span>
			</p>
		</div>

		<p class="text-xs">
			PNG, JPG or WebP · Max {maxFileSize} MB
		</p>
	</label>

	{#if error}
		<p class="text-sm">{error}</p>
	{/if}

	{#if files.length > 0}
		<div class="grid grid-cols-2 gap-3 sm:grid-cols-3 md:grid-cols-4">
			{#each files as item, i (item.file.name + item.file.lastModified)}
				<div class="group relative min-w-0">
					<img
						src={item.url}
						alt={item.file.name}
						class="aspect-square w-full rounded-lg border border-light-orange object-cover"
					/>

					<button
						type="button"
						onclick={() => removeFile(i)}
						class="absolute top-2 right-2 flex size-7 items-center justify-center rounded-full border border-light-orange bg-white text-orange hover:cursor-pointer hover:bg-off-white"
					>
						×
					</button>

					<p class="mt-1 truncate text-xs text-gray">
						{item.file.name}
					</p>
				</div>
			{/each}
		</div>
	{/if}
</div>
