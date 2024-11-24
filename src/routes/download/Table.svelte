<script lang="ts">
  import { type ModGetStateType } from "$lib/schema";
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";
  import { stateToString } from "./formatter";

  let { mods = [] } = $props<{
      mods: [string, ModGetStateType][];
  }>();

</script>

<Table
  striped={true}
  hoverable={true}
>
  <TableHead>
      <TableHeadCell>Name</TableHeadCell>
      <TableHeadCell>Status</TableHeadCell>
  </TableHead>
  <TableBody tableBodyClass="divide-y">
      {#each mods as [name, state] (name)}
          <TableBodyRow>
              <TableBodyCell>{name}</TableBodyCell>
              <TableBodyCell>
                  {stateToString(state)}
              </TableBodyCell>
          </TableBodyRow>
      {/each}
  </TableBody>
</Table>