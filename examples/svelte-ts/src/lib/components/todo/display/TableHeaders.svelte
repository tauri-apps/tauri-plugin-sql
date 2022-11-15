<script lang="ts">
  let tableHeaders = ['ID', 'Title', 'Completed', 'Action'];
</script>

<!-- Table head -->
<thead>
  <tr>
    {#each tableHeaders as header}
      <th>{header}</th>
    {/each}
  </tr>
</thead>
