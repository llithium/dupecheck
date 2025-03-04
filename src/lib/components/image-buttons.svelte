<script lang="ts">
  import Button, {
    buttonVariants,
  } from "$lib/components/ui/button/button.svelte";
  import { openPath, revealItemInDir } from "@tauri-apps/plugin-opener";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  interface Props {
    filepath: string;
  }

  let { filepath }: Props = $props();
</script>

<div class="w-full flex gap-2 pt-1">
  <Button
    size="sm"
    onclick={() => openPath(filepath)}
    class="basis-1/3"
    variant="secondary">Open</Button
  >
  <Button
    size="sm"
    onclick={() => revealItemInDir(filepath)}
    class="basis-1/3"
    variant="secondary">Reveal</Button
  >
  <Dialog.Root>
    <Dialog.Trigger
      disabled
      class={buttonVariants({
        variant: "destructive",
        class: "basis-1/3",
        size: "sm",
      })}
    >
      Delete
    </Dialog.Trigger>
    <Dialog.Content>
      <Dialog.Header>
        <Dialog.Title>Are you sure?</Dialog.Title>
        <Dialog.Description>
          This action cannot be undone. The file <strong>{filepath}</strong> will
          be permanently deleted from your device.
        </Dialog.Description>
      </Dialog.Header>
      <Dialog.Footer>
        <Button variant="destructive" type="submit">Delete</Button>
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
</div>

<style>
</style>
