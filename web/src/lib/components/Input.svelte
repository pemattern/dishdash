<script>
  let {
    label = '',
    placeholder = '',
    type = 'text',
    value = $bindable(''),
    name = undefined
  } = $props();
</script>

<div class="flex flex-col">
  {#if label}
    <label class="text-dark-gray font-bold text-sm" for=name>
        {label}
    </label>
  {/if}
    <input
        id={name}
        {name}
        {type}
        {placeholder}
        bind:value
        class="mt-1 px-2 py-1 border border-gray rounded-md"
    />
</div>
