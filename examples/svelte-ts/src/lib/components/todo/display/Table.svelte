<script lang="ts">
  import { todoStore } from '../../../stores/todoStore';
</script>

{#if $todoStore.length > 0}
  <table>
    <slot />
  </table>
{:else}
  <div>
    <span> No todos found in the database </span>
    <br />
    <br />
    <span> Try creating some todos in the above field </span>
  </div>
{/if}

<style>
  div,
  table {
    text-align: center;
    color: var(--primary);
    border-spacing: 0;
  }
  span {
    font-size: large;
    font-weight: bold;
    color: var(--text);
  }
</style>
