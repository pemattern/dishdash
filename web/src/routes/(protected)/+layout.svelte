<script lang="ts">
  let { children } = $props();
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { auth } from '#lib';
  import { resolve } from '$app/paths';

  onMount(() => {
    void checkAuth();
  });

  async function checkAuth() {
    await auth.me();
    if (!auth.user) {
      await auth.refresh();
    }
    if (auth.user) {
      return;
    }
    await goto(resolve('/login'));
  }
</script>

{#if !auth.user}
  <div>
    Checking your session…
  </div>
{:else if auth.user}
  {@render children()}
{/if}
