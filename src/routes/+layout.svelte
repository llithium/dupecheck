<script lang="ts">
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<ModeWatcher />
{@render children()}
