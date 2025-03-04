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
  import prettyBytes from "pretty-bytes";
  import * as Table from "$lib/components/ui/table/index.js";
  import { openPath, revealItemInDir } from "@tauri-apps/plugin-opener";

  let loading = $state(false);
  let loadingMessage = $state("Hashing...");
  let totalFiles: number | null = $state(null);
  let hashedFiles = $state(0);
  let duplicates: PotentialDuplicate[] = $state(mockDuplicates);

  async function openFiles() {
    // duplicates =
    console.log(await invoke("open_files"));
    loading = false;
  }

  const hashingStartedUnlisten = listen("hashing-started", () => {
    totalFiles = 0;
    hashedFiles = 0;
    loading = true;
    loadingMessage = "Hashing...";
  });
  const totalFilesUnlisten = listen<number>("total-files", (event) => {
    totalFiles = event.payload;
  });
  const hashedFilesUnlisten = listen<number>("hashed-file", (event) => {
    hashedFiles += event.payload;
  });
  const comparingStartedUnlisten = listen("comparing-started", () => {
    loadingMessage = "Comparing...";
  });
  const comparingFinishedUnlisten = listen("comparing-finished", () => {
    //TODO
  });
  onDestroy(() => {
    hashingStartedUnlisten.then((f) => f());
    totalFilesUnlisten.then((f) => f());
    hashedFilesUnlisten.then((f) => f());
    comparingStartedUnlisten.then((f) => f());
    comparingFinishedUnlisten.then((f) => f());
  });
</script>

<main class="container pb-4">
  <div class="w-full flex justify-start py-4 items-center">
    <div class="basis-1/3">
      {#if duplicates.length > 0}
        <span class="text-sm font-medium leading-none">
          Potential Duplicates: {duplicates.length}
        </span>
      {/if}
    </div>
    <div class="basis-1/3 flex justify-center">
      <Button disabled={loading} onclick={openFiles} variant="outline">
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
  {#if loading && hashedFiles != totalFiles}
    <Progress value={hashedFiles} max={totalFiles || 100} />
  {/if}
  {#if duplicates.length > 0}
    <Carousel.Root class="mx-auto w-11/12">
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
                <div class="font-semibold text-sm pb-1">
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
                <Table.Root>
                  <Table.Caption>
                    <button
                      onclick={() => revealItemInDir(duplicate.file_path1)}
                      >{duplicate.file_path1}</button
                    ></Table.Caption
                  >
                  <Table.Header>
                    <Table.Row>
                      <Table.Head class="w-[100px]">Details</Table.Head>
                    </Table.Row>
                  </Table.Header>
                  <Table.Body>
                    <Table.Row>
                      <Table.Cell class="font-medium">Size</Table.Cell>
                      <Table.Cell class="text-end"
                        >{prettyBytes(duplicate.size1)}</Table.Cell
                      >
                    </Table.Row>
                    <Table.Row>
                      <Table.Cell class="font-medium">Resolution</Table.Cell>
                      <Table.Cell class="text-end"
                        >{duplicate.resolution1[0]} x {duplicate
                          .resolution1[1]}</Table.Cell
                      >
                    </Table.Row>
                    <Table.Row>
                      <Table.Cell class="font-medium">Format</Table.Cell>
                      <Table.Cell class="text-end"
                        >{duplicate.format1.toLocaleUpperCase()}</Table.Cell
                      >
                    </Table.Row>
                  </Table.Body>
                </Table.Root>

                <!--  TODO Add delete button -->
              </div>
              <div class="flex flex-col basis-1/2">
                <div class="font-semibold text-sm pb-1 text-end">
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
                <Table.Root>
                  <Table.Caption>
                    <button
                      onclick={() => revealItemInDir(duplicate.file_path2)}
                      >{duplicate.file_path2}</button
                    >
                  </Table.Caption>
                  <Table.Header>
                    <Table.Row>
                      <Table.Head class="w-[100px]">Details</Table.Head>
                    </Table.Row>
                  </Table.Header>
                  <Table.Body>
                    <Table.Row>
                      <Table.Cell class="font-medium">Size</Table.Cell>
                      <Table.Cell class="text-end"
                        >{prettyBytes(duplicate.size2)}</Table.Cell
                      >
                    </Table.Row>
                    <Table.Row>
                      <Table.Cell class="font-medium">Resolution</Table.Cell>
                      <Table.Cell class="text-end"
                        >{duplicate.resolution2[0]} x {duplicate
                          .resolution2[1]}</Table.Cell
                      >
                    </Table.Row>
                    <Table.Row>
                      <Table.Cell class="font-medium">Format</Table.Cell>
                      <Table.Cell class="text-end"
                        >{duplicate.format2.toLocaleUpperCase()}</Table.Cell
                      >
                    </Table.Row>
                  </Table.Body>
                </Table.Root>
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
