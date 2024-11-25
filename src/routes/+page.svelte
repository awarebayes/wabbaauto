<script lang="ts">
  import MainButtons from "$lib/components/MainButtons.svelte";
  import MainStatus from "$lib/components/MainStatus.svelte";
  import MainTable from "$lib/components/MainTable.svelte";
  import MainAlert from "$lib/components/MainAlert.svelte";
  import { getModlists } from "$lib/requests";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import type { Modlist } from "$lib/schema";

  let modlists = writable<Modlist[]>([]);

  onMount(async () => {
    modlists.set(await getModlists());
  });
</script>

<main class="container p-4">
  <h1 class="text-3xl font-medium text-center">Wabba Auto DL</h1>
  <MainButtons />
  <div class="flex flex-col gap-2 pt-4">
    <MainStatus modlists={$modlists} />
    <MainAlert />
    <br />
    <MainTable modlists={modlists} />
  </div>
</main>
