
<script lang="ts">
  import { onMount, type Snippet } from 'svelte';

  let {
    open = $bindable(),
    title,
    children
  }: {
    open?: boolean;
    title: string;
    children: Snippet;
  } = $props();

  onMount(() => {
    const handleKeydown = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        open = false;
      }
    };
    window.addEventListener('keydown', handleKeydown);
    return () => {
      window.removeEventListener('keydown', handleKeydown);
    };
  });
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 grid place-items-center overflow-y-auto bg-black/50 p-4"
    role="presentation"
    onclick={(event) => {
      if (event.target === event.currentTarget) {
        open = false;
      }
    }}
  >
    <section
      class="w-full max-w-xl max-h-[calc(100dvh-2rem)] overflow-y-auto rounded-xl bg-white text-gray-900 shadow-2xl"
    >
        <h2 class="text-lg font-semibold">
           {title}
         </h2>

      <div class="p-4">
        {@render children()}
      </div>
    </section>
  </div>
{/if}
