<script lang="ts">
  import { todoStore } from '../../../stores/todoStore';
  import Todo from './Todo.svelte';
</script>

<tbody class="text-center">
  {#each $todoStore as todo (todo.id)}
    <Todo {...todo} />
  {/each}
</tbody>
