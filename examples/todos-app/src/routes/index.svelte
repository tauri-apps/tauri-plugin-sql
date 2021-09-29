<script lang="ts">
  import { onMount } from 'svelte';
  import { todos, incompleteCount, init, add, remove, toggleComplete } from '../stores/todos';

  import List from '../components/List.svelte';
  import NewItem from '../components/NewItem.svelte';
  import DisplayCounts from '../components/DisplayCounts.svelte';

  onMount(init);
</script>

<div class="flex justify-center h-screen pt-12 bg-gray-100">
  <div class="w-full max-w-lg mx-4">
    <List items={$todos} on:toggleComplete={toggleComplete} on:remove={remove} />

    <NewItem on:add={add} />

    <DisplayCounts total={$todos.length} remaining={$incompleteCount} />
  </div>
</div>
