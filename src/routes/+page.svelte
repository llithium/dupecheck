<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import type { PotentialDuplicate } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import * as Carousel from "$lib/components/ui/carousel/index.js";
  import LoaderCircle from "lucide-svelte/icons/loader-circle";
  import FolderOpen from "lucide-svelte/icons/folder-open";
  import Info from "lucide-svelte/icons/info";
  import { listen } from "@tauri-apps/api/event";
  import { onDestroy } from "svelte";
  import { Progress } from "$lib/components/ui/progress";
  import { openPath } from "@tauri-apps/plugin-opener";
  import ImageButtons from "$lib/components/image-buttons.svelte";
  import ImageDetailsTable from "$lib/components/image-details-table.svelte";
  import { type CarouselAPI } from "$lib/components/ui/carousel/context.js";
  import { Slider } from "$lib/components/ui/slider/index.js";
  import { Badge } from "$lib/components/ui/badge/index.js";
  import * as Tooltip from "$lib/components/ui/tooltip/index.js";
  import debounce from "debounce";
  import { toast } from "svelte-sonner";

  let loading = $state(false);
  let loadingMessage = $state("Hashing...");
  let totalFiles = $state(0);
  let currentFiles = $state(0);
  let duplicates: PotentialDuplicate[] = $state([]);
  let threshold = $state(parseInt(localStorage.getItem("threshold") || "9"));
  let currentSlide = $state(0);

  let carouselAPI = $state<CarouselAPI>();
  $effect(() => {
    if (carouselAPI) {
      carouselAPI.on("slidesInView", () => {
        currentSlide = carouselAPI?.slidesInView()[0] || 0;
      });
    }
  });

  const saveThreshold = debounce(
    () => localStorage.setItem("threshold", threshold.toString()),
    300
  );
  $effect(() => {
    threshold;
    saveThreshold();
  });

  async function openFiles() {
    try {
      const files: PotentialDuplicate[] = await invoke("open_files", {
        distanceThreshold: threshold,
      });
      if (files) {
        duplicates = files;
        if (files.length === 0) {
          toast.info("No duplicates found");
          currentSlide = 0;
        } else if (files.length > 1) {
          carouselAPI?.scrollTo(0);
          currentSlide = 0;
        }
      }
    } catch (error) {
      toast.error(error as string);
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
      if (duplicates.length === 0) {
        currentSlide = 0;
      } else if (currentSlide + 1 > duplicates.length) {
        carouselAPI?.scrollTo(duplicates.length - 1);
        currentSlide = duplicates.length - 1;
      }
    } catch (error) {
      toast.error(error as string);
    }
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
  <div class="w-full flex justify-between py-4 gap-4 items-center">
    <div class="flex gap-4 items-center">
      {#if duplicates.length > 0 && !loading}
        <Badge class="rounded-2xl" variant="secondary">
          {currentSlide + 1}/{duplicates.length} Potential Duplicates
        </Badge>

        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger>
              {#snippet child({ props })}
                <Badge class="rounded-2xl" {...props}>
                  <Info class="h-3.5 w-3.5 mr-1.5" />Distance: {duplicates[
                    currentSlide
                  ].distance}
                </Badge>{/snippet}
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p class="text-sm">Similarity of the images (0 = identical)</p>
            </Tooltip.Content>
          </Tooltip.Root>
        </Tooltip.Provider>
      {:else if loading}
        <Badge class="rounded-2xl">
          {currentFiles}/{totalFiles}
        </Badge>
      {/if}
    </div>
    <div class="flex items-center gap-4">
      <div
        class={`flex gap-3 items-center ${
          loading && "opacity-50 pointer-events-none"
        }`}
      >
        <Tooltip.Provider>
          <Tooltip.Root>
            <Tooltip.Trigger>
              {#snippet child({ props })}
                <span {...props} class="text-sm font-medium leading-none"
                  >Threshold:</span
                >
              {/snippet}
            </Tooltip.Trigger>
            <Tooltip.Content>
              <p class="text-sm">
                Distance threshold for potential duplicates (0 = identical)
              </p>
            </Tooltip.Content>
          </Tooltip.Root>
        </Tooltip.Provider>
        <div class="w-32 flex items-center gap-2">
          <Slider
            type="single"
            bind:value={threshold}
            min={0}
            max={20}
            step={1}
          />

          <span class="w-6 text-center pl-1 text-sm font-medium leading-none"
            >{threshold}</span
          >
        </div>
      </div>
      <Button disabled={loading} onclick={openFiles}>
        {#if loading}
          <LoaderCircle class="animate-spin" />
          {loadingMessage}
        {:else}
          <FolderOpen />
          Open Folders
        {/if}
      </Button>
    </div>
  </div>

  {#if duplicates.length > 0}
    <Carousel.Root
      setApi={(emblaApi) => (carouselAPI = emblaApi)}
      class={`mx-auto w-11/12 ${loading && "opacity-20 pointer-events-none"}`}
    >
      <Carousel.Content>
        {#each duplicates as duplicate, i}
          <Carousel.Item>
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
  {#if loading && currentFiles != totalFiles}
    <div class="flex justify-center">
      <Progress
        class="fixed drop-shadow-xl w-11/12 h-3 top-1/2 -translate-y-1/2"
        value={currentFiles}
        max={totalFiles || 100}
      />
    </div>
  {/if}
</main>
