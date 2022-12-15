<script lang="ts">
  let tableHeaders = ['ID', 'Title', 'Completed', 'Action'];
</script>

<thead>
  <tr class="text-center ">
    {#each tableHeaders as header}
      <th class="[&:nth-child(2)]:w-[50%]">{header}</th>
    {/each}
  </tr>
</thead>
