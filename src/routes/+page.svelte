<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import { mockDuplicates } from "$lib/mock-duplicates";
  import type { PotentialDuplicate } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import * as Carousel from "$lib/components/ui/carousel/index.js";
  import LoaderCircle from "lucide-svelte/icons/loader-circle";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";
  import { Progress } from "$lib/components/ui/progress";

  let loading = $state(false);
  let totalFiles = $state(0);
  let hashedFiles = $state(0);
  let duplicates: PotentialDuplicate[] = $state(mockDuplicates);

  async function openFiles() {
    // loading = true;
    // duplicates =
    await invoke("open_files");
    loading = false;
  }

  const hashingStartedUnlisten = listen("hashing-started", () => {
    totalFiles = 0;
    hashedFiles = 0;
    loading = true;
  });
  const totalFilesUnlisten = listen<number>("total-files", (event) => {
    totalFiles = event.payload;
  });
  const hashedFilesUnlisten = listen<number>("hashed-file", (event) => {
    hashedFiles += event.payload;
  });

  onDestroy(() => {
    hashingStartedUnlisten.then((f) => f());
    totalFilesUnlisten.then((f) => f());
    hashedFilesUnlisten.then((f) => f());
  });
</script>

<main class="container">
  <div class="w-full flex justify-center pt-4">
    <Button disabled={loading} onclick={openFiles} variant="outline">
      {#if loading}
        <LoaderCircle class="animate-spin" />
      {/if}
      Open Folders</Button
    >
  </div>
  {#if loading}
    <Progress value={hashedFiles} max={totalFiles} />
  {/if}
  <Carousel.Root class="mx-auto w-10/12">
    <Carousel.Content>
      {#each duplicates as duplicate}
        <Carousel.Item>
          <span>Distance: {duplicate[2]}</span>
          <div class="flex gap-2">
            <div class="flex flex-col basis-1/2">
              <img
                class="w-full max-h-[calc(100vh-150px)] object-contain"
                src={convertFileSrc(duplicate[0])}
                alt=""
                srcset=""
              />
              <span>{duplicate[0]}</span>
              <!--  TODO Add delete button -->
              <!-- TODO Add show in file browser button -->
            </div>
            <div class="flex-col basis-1/2">
              <img
                class="w-full max-h-[calc(100vh-150px)] object-contain"
                src={convertFileSrc(duplicate[1])}
                alt=""
                srcset=""
              />
              <span>{duplicate[1]}</span>
            </div>
          </div>
        </Carousel.Item>
      {/each}
    </Carousel.Content>
    <Carousel.Previous />
    <Carousel.Next />
  </Carousel.Root>
</main>
