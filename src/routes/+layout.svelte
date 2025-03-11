<script lang="ts">
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";
  import type { Snippet } from "svelte";
  import { Toaster } from "$lib/components/ui/sonner/index.js";

  let { children }: { children: Snippet } = $props();
</script>

<ModeWatcher />
<Toaster />
{@render children()}
