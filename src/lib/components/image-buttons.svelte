<script lang="ts">
  import Button, {
    buttonVariants,
  } from "$lib/components/ui/button/button.svelte";
  import { openPath, revealItemInDir } from "@tauri-apps/plugin-opener";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import Eye from "lucide-svelte/icons/eye";
  import FolderOpen from "lucide-svelte/icons/folder-open";
  import Trash2 from "lucide-svelte/icons/trash-2";

  interface Props {
    filepath: string;
    deleteFile: (event: SubmitEvent) => void;
    index: number;
  }

  let { filepath, deleteFile, index }: Props = $props();

  let dialogOpen = $state(false);
</script>

<div class="w-full flex gap-2 pt-1">
  <Button
    size="sm"
    onclick={() => openPath(filepath)}
    class="basis-1/3"
    variant="secondary"
  >
    <Eye />
    Open</Button
  >
  <Button
    size="sm"
    onclick={() => revealItemInDir(filepath)}
    class="basis-1/3"
    variant="secondary"
  >
    <FolderOpen />
    Reveal</Button
  >
  <Dialog.Root bind:open={dialogOpen}>
    <Dialog.Trigger
      class={buttonVariants({
        variant: "destructive",
        class: "basis-1/3",
        size: "sm",
      })}
    >
      <Trash2 />
      Delete
    </Dialog.Trigger>
    <Dialog.Content>
      <Dialog.Header class="select-none">
        <Dialog.Title>Are you sure?</Dialog.Title>
        <Dialog.Description>
          The file <strong>{filepath}</strong> will be moved to the system's trash.
        </Dialog.Description>
      </Dialog.Header>
      <Dialog.Footer class="select-none">
        <form
          onsubmit={(event) => {
            dialogOpen = false;
            deleteFile(event);
          }}
        >
          <input type="hidden" name="index" value={index} />
          <input type="hidden" name="path" value={filepath} />

          <Button variant="destructive" type="submit">Delete</Button>
        </form>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
</div>

<style>
</style>
