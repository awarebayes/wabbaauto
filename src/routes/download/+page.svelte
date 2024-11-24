<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { page } from "$app/stores";
  import MainAlert from "$lib/components/MainAlert.svelte";
  import { getModistLong } from "$lib/requests";
  import { AppState, DownloadResponseSchema, validateAppState, type AppStateType, type DownloadResponse, type ModGetStateType, type ModlistLong } from "$lib/schema";
  import { onMount } from "svelte";
  import { errorMsg } from "$lib/stores";
  import { listen } from '@tauri-apps/api/event'
  import Table from "./Table.svelte";

  let machineURL = $page.url.searchParams.get("machineURL")!;
  let directory = $page.url.searchParams.get("dir")!;
  let modlist = $state<ModlistLong | null>(null);
  let download_status = $state<DownloadResponse | null>(null);
  let current_download_state = $state<AppStateType | null>(null);
  let mods = $state<[string, ModGetStateType][]>([]);
  let recent_fails = $state<[string][]>([]);

  onMount(async () => {
    modlist = await getModistLong(machineURL);
    let download_status_response = await invoke("run_download_thread", {"machineUrl": machineURL, "downloadDir": directory}) as string;
    download_status = DownloadResponseSchema.parse(JSON.parse(download_status_response));
    if (!download_status.success) {
      $errorMsg = download_status.error;
    }
  });


  listen("download-progress", (event) => {
    current_download_state = AppState.parse(JSON.parse(event.payload as string));
    mods = current_download_state.getting_archives;
    recent_fails = current_download_state.recent_fails;
  })

</script>

<main class="container p-4">
  <MainAlert />
  Name: {modlist?.Name}
  <br />
  Version: {modlist?.Version}
  <br />
  Directory: {directory}

  {#if current_download_state !== null}
    <br />
    Total: {current_download_state.total}
    <br />
    Failed: {current_download_state.failed}
    <br />
    Successes: {current_download_state.successes}
    <br />
    <Table mods={mods}/>
    <br />
    {#each recent_fails as fail}
      Failed: {fail}
      <br />
    {/each}
  {/if}
</main>
