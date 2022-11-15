<script>
  import TodoDisplay from './lib/components/todo/display/TodoDisplay.svelte';
  import TodoCreator from './lib/components/todo/create/TodoCreator.svelte';
</script>

<main>
  <div class="container">
    <TodoCreator />
    <TodoDisplay />
  </div>
</main>

<style>
  :global(:root) {
    --primary: #c6d7dd;
    --secondary: #c0c0c2;
    --bg-100: rgb(94, 94, 94);
    --bg-200: #424242;
    --bg-300: #252525;
    --bg-400: #202020;
  }
  main {
    min-height: 100vh;
    min-width: 100vw;
    background-color: var(--bg-300);
  }

  .container {
    max-width: 600px;
    margin: auto;
    padding: 1rem 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
