<script>
  import {
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
    Checkbox,
    TableSearch,
    Button,
  } from "flowbite-svelte";
  import { modlists } from "$lib/stores";
</script>

<Table
  striped={true}
  items={$modlists}
  hoverable={true}
  filter={(item, searchTerm) =>
    item.name.toLowerCase().includes(searchTerm.toLowerCase())}
>
  <TableHead>
    <TableHeadCell>Name</TableHeadCell>
    <TableHeadCell>Link</TableHeadCell>
  </TableHead>
  <TableBody tableBodyClass="divide-y">
    <TableBodyRow slot="row" let:item>
      <TableBodyCell>{item.name}</TableBodyCell>
      <TableBodyCell>
        <Button
          size="xs"
          pill
          color="alternative"
          href={`/downloadPick?machineURL=${item.machineURL}`}
        >
          Download
        </Button>
      </TableBodyCell>
    </TableBodyRow>
  </TableBody>
</Table>
