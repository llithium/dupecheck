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

  let loading = $state(false);
  let loadingMessage = $state("Hashing...");
  let totalFiles: number | null = $state(null);
  let hashedFiles = $state(0);
  let duplicates: PotentialDuplicate[] = $state(mockDuplicates);

  async function openFiles() {
    // loading = true;
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
  <div class="w-full flex justify-center pt-4">
    <Button disabled={loading} onclick={openFiles} variant="outline">
      {#if loading}
        <LoaderCircle class="animate-spin" />
        {loadingMessage}
      {:else}
        Open Folders
      {/if}
    </Button>
  </div>
  {#if loading}
    <Progress value={hashedFiles} max={totalFiles || 100} />
  {/if}
  <Carousel.Root class="mx-auto w-11/12">
    <Carousel.Content>
      {#each duplicates as duplicate}
        <Carousel.Item>
          <span>Distance: {duplicate.distance}</span>
          <div class="flex gap-2">
            <div class="flex flex-col basis-1/2">
              <img
                class="w-full max-h-[calc(100vh-150px)] object-contain"
                src={convertFileSrc(duplicate.file_path1)}
                alt=""
                srcset=""
              />

              <Table.Root>
                <Table.Caption>{duplicate.file_path1}</Table.Caption>
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
                </Table.Body>
              </Table.Root>

              <!--  TODO Add delete button -->
              <!-- TODO Add show in file browser button -->
            </div>
            <div class="flex flex-col basis-1/2">
              <img
                class="w-full max-h-[calc(100vh-150px)] object-contain"
                src={convertFileSrc(duplicate.file_path2)}
                alt=""
                srcset=""
              />
              <Table.Root>
                <Table.Caption>{duplicate.file_path2}</Table.Caption>
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
</main>
