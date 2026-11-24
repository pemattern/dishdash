<script lang="ts">
	type Props = {
		text: string;
		textColor?: string;
		bgColor?: string;
		borderColor?: string;
		ringColor?: string;
		insetRingColor?: string;
	};

	let { text, textColor, bgColor, borderColor, ringColor, insetRingColor }: Props = $props();
</script>

<div
	class={`rounded-full px-2 py-1 text-xs font-semibold
        ${textColor ? textColor : ''} ${bgColor ? bgColor : ''}
        ${borderColor ? `border ${borderColor}` : ''}
        ${ringColor ? `ring ${ringColor}` : ''}
        ${insetRingColor ? `inset-ring ${insetRingColor}` : ''}
    `}
>
	{text}
</div>
