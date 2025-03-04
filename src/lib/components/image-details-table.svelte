<script lang="ts">
  import prettyBytes from "pretty-bytes";
  import * as Table from "$lib/components/ui/table/index.js";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";

  interface Props {
    filePath: string;
    size: number;
    resolutionX: number;
    resolutionY: number;
    format: string;
  }

  let { filePath, format, resolutionX, resolutionY, size }: Props = $props();
</script>

<Table.Root>
  <Table.Caption>
    <button onclick={() => revealItemInDir(filePath)}>{filePath}</button
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
      <Table.Cell class="text-end">{prettyBytes(size)}</Table.Cell>
    </Table.Row>
    <Table.Row>
      <Table.Cell class="font-medium">Resolution</Table.Cell>
      <Table.Cell class="text-end">{resolutionX} x {resolutionY}</Table.Cell>
    </Table.Row>
    <Table.Row>
      <Table.Cell class="font-medium">Format</Table.Cell>
      <Table.Cell class="text-end">{format.toLocaleUpperCase()}</Table.Cell>
    </Table.Row>
  </Table.Body>
</Table.Root>

<style>
</style>
