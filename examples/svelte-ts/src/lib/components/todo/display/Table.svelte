<script lang="ts">
  import { todoStore } from '../../../stores/todoStore';
</script>

<div class="justify-center overflow-x-auto text-center">
  {#if $todoStore.length > 0}
    <table class="table w-full">
      <slot />
    </table>
  {:else}
    <span class="font-bold text-2xl"> No todos found in database </span>
  {/if}
</div>
