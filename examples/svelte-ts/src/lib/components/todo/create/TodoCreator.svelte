<script>
  import { todoStore } from '../../../stores/todoStore';
  import AddIcon from './AddIcon.svelte';

  let todoTitle = '';

  const createNewTask = () => {
    if (!todoTitle) return;
    todoStore.create(todoTitle);
    todoTitle = '';
  };
</script>

<form
  on:submit|preventDefault={createNewTask}
  class="bg-base-100 flex items-center "
>
  <input
    bind:value={todoTitle}
    type="text"
    placeholder="Create new todo"
    class="flex-1 bg-transparent pl-2 h-12 text-xl outline-none border-none"
  />
  <button type="submit" class="h-8 w-8 mr-2 fill-base-content p-1">
    <AddIcon />
  </button>
</form>
