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

<form on:submit|preventDefault={createNewTask}>
  <input bind:value={todoTitle} type="text" placeholder="Create new todo" />
  <button class:valid={todoTitle} type="submit">
    <AddIcon />
  </button>
</form>

<style>
  form {
    display: flex;
    background-color: var(--bg-200);
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    font-size: x-large;
  }
  form > * {
    background-color: transparent;
    outline: none;
    border: none;
  }

  * {
    font-size: inherit;
  }
  button {
    height: auto;
    display: flex;
    height: 2.25rem;
    width: 2.25rem;
    cursor: pointer;
    fill: var(--bg-400);
  }
  button.valid {
    fill: var(--primary);
  }

  input {
    flex: 1;
    padding-left: 0.25rem;
    color: var(--primary);
  }
  input::placeholder {
    color: var(--bg-400);
  }
</style>
