<script lang="ts">
  import type { Modlist } from "$lib/schema";
  import type { Writable } from "svelte/store";

  let { modlists } = $props<{
    modlists: Writable<Modlist[]>;
  }>();

  let search = $state("");

  let filtered_modlists = $derived(
    $modlists.filter((x) => x.name.toLowerCase().includes(search.toLowerCase()))
  );
</script>

<div class="relative overflow-x-auto shadow-md sm:rounded-lg">
  <div class="p-4">
    <input 
      type="text" 
      class="w-full p-2 border rounded-lg"
      placeholder="Search by name..."
      bind:value={search}
    >
  </div>
  
  <table class="w-full text-sm text-left text-gray-500">
    <thead class="text-xs text-gray-700 uppercase bg-gray-50">
      <tr>
        <th scope="col" class="px-6 py-3">
          Name
        </th>
        <th scope="col" class="px-6 py-3">
          Link
        </th>
      </tr>
    </thead>
    <tbody class="divide-y">
      {#each filtered_modlists as item}
        <tr class="bg-white border-b hover:bg-gray-50 even:bg-gray-50">
          <td class="px-6 py-4 font-medium text-gray-900 whitespace-nowrap">
            {item.name}
          </td>
          <td class="px-6 py-4">
            <a
              href={`/downloadPick?machineURL=${item.machineURL}`}
              class="px-4 py-2 text-xs font-medium text-gray-900 bg-white border border-gray-200 rounded-full hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700"
            >
              Download
            </a>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>