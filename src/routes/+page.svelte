<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import type { PotentialDuplicate } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import * as Carousel from "$lib/components/ui/carousel/index.js";
  import LoaderCircle from "lucide-svelte/icons/loader-circle";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";
  import { Progress } from "$lib/components/ui/progress";
  import { openPath } from "@tauri-apps/plugin-opener";
  import ImageButtons from "$lib/components/image-buttons.svelte";
  import ImageDetailsTable from "$lib/components/image-details-table.svelte";
  import { type CarouselAPI } from "$lib/components/ui/carousel/context.js";

  let loading = $state(false);
  let loadingMessage = $state("Hashing...");
  let totalFiles: number | null = $state(null);
  let currentFiles = $state(0);
  let duplicates: PotentialDuplicate[] = $state([]);

  let carouselAPI = $state<CarouselAPI>();

  async function openFiles() {
    try {
      const files: PotentialDuplicate[] = await invoke("open_files");
      if (files.length > 0) {
        duplicates = files;
        carouselAPI?.scrollTo(0);
      }
    } catch (error) {
    } finally {
      loading = false;
    }
  }
  async function deleteFile(event: SubmitEvent) {
    event.preventDefault();
    const formData = new FormData(event.target as HTMLFormElement);
    const index = formData.get("index") as unknown as number;
    const path = formData.get("path") as string;

    try {
      await invoke("delete_file", { path: path });
      duplicates.splice(index, 1);
    } catch (error) {}
  }
  const hashingStartedUnlisten = listen("hashing-started", () => {
    totalFiles = 0;
    currentFiles = 0;
    loading = true;
    loadingMessage = "Hashing...";
  });
  const totalFilesUnlisten = listen<number>("total-files", (event) => {
    totalFiles = event.payload;
  });
  const hashedFilesUnlisten = listen<number>("current-file-count", (event) => {
    currentFiles += event.payload;
  });
  const comparingStartedUnlisten = listen("comparing-started", () => {
    currentFiles = 0;
    loadingMessage = "Comparing...";
  });

  onDestroy(() => {
    hashingStartedUnlisten.then((f) => f());
    totalFilesUnlisten.then((f) => f());
    hashedFilesUnlisten.then((f) => f());
    comparingStartedUnlisten.then((f) => f());
  });
</script>

<main class="container pb-4">
  <div class="w-full flex justify-start py-4 items-center">
    <div class="basis-1/3">
      {#if duplicates.length > 0 && !loading}
        <span class="text-sm font-medium leading-none">
          Potential Duplicates: {duplicates.length}
        </span>
      {/if}
    </div>
    <div class="basis-1/3 flex justify-center">
      <Button disabled={loading} onclick={openFiles}>
        {#if loading}
          <LoaderCircle class="animate-spin" />
          {loadingMessage}
        {:else}
          Open Folders
        {/if}
      </Button>
    </div>
    <div class="basis-1/3"></div>
  </div>
  {#if loading && currentFiles != totalFiles}
    <Progress value={currentFiles} max={totalFiles || 100} />
  {/if}
  {#if duplicates.length > 0}
    <Carousel.Root
      setApi={(emblaApi) => (carouselAPI = emblaApi)}
      class={`mx-auto w-11/12 ${loading && "opacity-60 pointer-events-none"}`}
    >
      <Carousel.Content>
        {#each duplicates as duplicate, i}
          <Carousel.Item>
            <div class="w-full flex justify-start items-center pb-2">
              <div class="basis-1/3">
                <span class="text-muted-foreground text-sm">
                  {i + 1}/{duplicates.length}
                </span>
              </div>
              <div class="basis-1/3 flex justify-center">
                <div class="font-semibold text-sm text-center">
                  Distance: {duplicate.distance}
                </div>
              </div>
              <div class="basis-1/3"></div>
            </div>

            <div class="flex gap-2">
              <div class="flex flex-col basis-1/2">
                <div class="font-semibold text-center text-sm pb-1">
                  {duplicate.filename1}
                </div>

                <button onclick={() => openPath(duplicate.file_path1)}>
                  <img
                    class="w-full max-h-[calc(100vh-150px)] object-contain"
                    src={convertFileSrc(duplicate.file_path1)}
                    alt=""
                    srcset=""
                  />
                </button>
                <ImageButtons
                  {deleteFile}
                  index={i}
                  filepath={duplicate.file_path1}
                />
                <ImageDetailsTable
                  filePath={duplicate.file_path1}
                  size={duplicate.size1}
                  resolutionX={duplicate.resolution1[0]}
                  resolutionY={duplicate.resolution1[1]}
                  format={duplicate.format1}
                />
              </div>
              <div class="flex flex-col basis-1/2">
                <div class="font-semibold text-center text-sm pb-1">
                  {duplicate.filename2}
                </div>
                <button onclick={() => openPath(duplicate.file_path2)}>
                  <img
                    class="w-full max-h-[calc(100vh-150px)] object-contain"
                    src={convertFileSrc(duplicate.file_path2)}
                    alt=""
                    srcset=""
                  />
                </button>
                <ImageButtons
                  {deleteFile}
                  index={i}
                  filepath={duplicate.file_path2}
                />
                <ImageDetailsTable
                  filePath={duplicate.file_path2}
                  size={duplicate.size2}
                  resolutionX={duplicate.resolution2[0]}
                  resolutionY={duplicate.resolution2[1]}
                  format={duplicate.format2}
                />
              </div>
            </div>
          </Carousel.Item>
        {/each}
      </Carousel.Content>
      <Carousel.Previous />
      <Carousel.Next />
    </Carousel.Root>
  {/if}
</main>
