<script lang="ts">
  import Button from "$lib/components/ui/button/button.svelte";
  import { mockDuplicates } from "$lib/mock-duplicates";
  import type { PotentialDuplicate } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import * as Carousel from "$lib/components/ui/carousel/index.js";

  let duplicates: PotentialDuplicate[] = $state(mockDuplicates);

  async function openFiles() {
    // duplicates =
    await invoke("open_files");
  }
</script>

<main class="container">
  <div class="w-full flex justify-center pt-4">
    <Button onclick={openFiles} variant="outline">Open Folders</Button>
  </div>

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
  {#each duplicates as duplicate}
    <!-- <div class="flex flex-col">
      <div class="flex">
        <img
          class="max-h-44 max-w-44"
          src={convertFileSrc(duplicate[0])}
          alt=""
          srcset=""
        />
        <span>{duplicate[0]}</span>
      </div>
      <div class="flex">
        <img
          class="max-h-44 max-w-44"
          src={convertFileSrc(duplicate[1])}
          alt=""
          srcset=""
        />
        <span>{duplicate[1]}</span>
      </div>
      <span>Distance: {duplicate[2]}</span>
    </div> -->
  {/each}
</main>
