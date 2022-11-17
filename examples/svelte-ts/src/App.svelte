<script>
  import TodoDisplay from './lib/components/todo/display/TodoDisplay.svelte';
  import TodoCreator from './lib/components/todo/create/TodoCreator.svelte';
  import Header from './lib/components/header/Header.svelte';
  import { theme } from './lib/stores/theme';
  import { todoStore } from './lib/stores/todoStore';

  $: {
    document.documentElement.setAttribute('data-theme', $theme);
    document.documentElement.classList.value = $theme;
  }
</script>

<div class="min-h-screen bg-base-300">
  <main class="max-w-2xl m-auto flex flex-col space-y-4 py-2">
    {#await todoStore.init()}
      <span>Initializing your database.AddIcon...</span>
    {:then}
      <Header />
      <TodoCreator />
      <TodoDisplay />
    {/await}
  </main>
</div>
