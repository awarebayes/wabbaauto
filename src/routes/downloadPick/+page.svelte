<script lang="ts">
  import { page } from "$app/stores";
  import MainAlert from "$lib/components/MainAlert.svelte";
  import { getModistLong } from "$lib/requests";
  import { type ModlistLong } from "$lib/schema";
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Button } from "flowbite-svelte";
  import { errorMsg } from "$lib/stores";

  let machineURL = $page.url.searchParams.get("machineURL")!;
  let modlist = $state<ModlistLong | null>(null);

  onMount(async () => {
    modlist = await getModistLong(machineURL);
  });

  async function pickDirectory() {
    const selectedDir = await open({
      directory: true, // Ensures that only directories can be selected
      multiple: false, // Single directory selection
    });

    if (selectedDir) {
      directory = selectedDir;
    } else {
      $errorMsg = "No directory was selected";
    }
  }

  let directory = $state("None");
</script>

<main class="container p-4">
  <MainAlert />
  Name: {modlist?.Name}
  <br />
  Version: {modlist?.Version}
  <br />
  Directory: {directory}
  <br />
  Mods: {modlist?.Archives.length}
  <br />
  <Button on:click={pickDirectory}>Pick download directory</Button>
  {#if directory != "None"}
    <Button
      color="green"
      href={`/download?dir=${directory}&machineURL=${machineURL}`}
      >Download here</Button
    >
  {/if}
  <br />
  <div class="mt-4">
    <img src={modlist?.LargeImage} alt="Large" />
  </div>
</main>
