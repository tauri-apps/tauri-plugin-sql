<script lang="ts">
  import { todoStore } from '../../../stores/todoStore';
</script>

<tbody class="text-center">
  {#each $todoStore as { completed, id, title } (id)}
    <tr>
      <td>{id}</td>
      <td class="font-bold text-lg">{title}</td>
      <td>
        <!-- IMPORTANTR -->
        <!-- Ordering of the directives (bind & on:change) matters! -->
        <input
          bind:checked={completed}
          on:change={() => {
            todoStore.update({
              completed,
              id,
              title,
            });
          }}
          type="checkbox"
          class="checkbox checkbox-primary"
        />
      </td>
      <td>
        <button
          on:click={() => {
            todoStore.remove(id);
          }}
          type="checkbox"
          class="btn btn-sm btn-primary">Delete</button
        >
      </td>
    </tr>
  {/each}
</tbody>
