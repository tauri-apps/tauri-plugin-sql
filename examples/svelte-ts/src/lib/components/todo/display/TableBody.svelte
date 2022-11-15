<script lang="ts">
  import { todoStore } from '../../../stores/todoStore';
</script>

<tbody>
  {#each $todoStore as { completed, id, title } (id)}
    <tr>
      <td>{id}</td>
      <td>{title}</td>
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
        />
      </td>
      <td>
        <button
          on:click={() => {
            todoStore.remove(id);
          }}
          type="checkbox">Delete</button
        >
      </td>
    </tr>
  {/each}
</tbody>

<style>
  button {
    padding: 0.3rem 0.4rem;
    background-color: var(--bg-midtone);
    color: var(--text);

    cursor: pointer;
    border: none;
    border-radius: 0.35rem;
    transition: background-color 0.1s ease-in;
  }
  button:hover {
    background-color: var(--text-highlited);
    color: var(--bg-light);
  }
  input {
    cursor: pointer;
    transform: scale(2);
  }
  td:nth-child(3) {
    width: 1px;
  }
  tbody:before {
    content: '';
    display: block;
    height: 0.5rem;
  }

  tr:first-child > td:last-child {
    border-radius: 0 1rem 0 0;
  }
  tr:first-child > td:first-child {
    border-radius: 1rem 0 0 0;
  }
  tr:last-child > td:last-child {
    border-radius: 0 0 1rem;
  }
  tr:last-child > td:first-child {
    border-radius: 0 0 0 1rem;
  }

  tr:nth-child(even) {
    background-color: var(--bg-200);
  }
  tr:nth-child(odd) {
    background-color: var(--bg-100);
  }
</style>
