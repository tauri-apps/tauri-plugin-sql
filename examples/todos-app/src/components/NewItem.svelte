<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import PlusIcon from './icons/PlusIcon.svelte';

  const dispatch = createEventDispatcher();

  let inputRef;
  let newTitle: string = '';

  function submit(event) {
    if (event.key !== undefined && event.key !== 'Enter') return;
    if (!newTitle.trim()) return;

    dispatch('add', { title: newTitle.trim() });
    newTitle = '';
    setFocus();
  }

  function setFocus() {
    if (inputRef) inputRef.focus();
  }
</script>

<div class="flex items-center justify-between px-3 py-2 m-4 text-gray-800 bg-white rounded shadow">
  <input
    bind:this={inputRef}
    bind:value={newTitle}
    on:keydown={submit}
    id="new-item-input"
    aria-label="new task"
    class="flex-auto mr-2 border-0"
  />

  <button
    on:click={submit}
    aria-label="create new task"
    class="w-5 h-5 border-0 rounded-full hover:bg-gray-400"
    tabindex="0"
  >
    <PlusIcon />
  </button>
</div>
