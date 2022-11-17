<script>
  import Svg from './Svg.svelte';
  import { theme } from '../../stores/theme';
</script>

<button on:click={theme.toggleTheme}>
  <Svg />
</button>
