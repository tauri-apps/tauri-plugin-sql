<script lang="ts">
  import { appConfigDir } from '@tauri-apps/api/path';
  import { writeText } from '@tauri-apps/api/clipboard';
  import ThemeSwitcher from '../theme/ThemeSwitcher.svelte';

  const db_dir = appConfigDir();
</script>

<header class="relative flex items-center justify-center">
  <div class="absolute left-0 ">
    <ThemeSwitcher />
  </div>
  <div>
    <span class="text-lg font-bold">SQLITE database file is located at:</span>
    <br />
    {#await db_dir then dir}
      <code class="bg-base-100  p-2 rounded-md">
        {dir}
      </code>
      <button
        on:click={() => {
          writeText(dir);
        }}
        class="ml-2 btn btn-sm btn-accent">Copy</button
      >
    {/await}
    <span />
  </div>
</header>
