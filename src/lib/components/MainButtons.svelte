<script>
  import { Button } from "flowbite-svelte";
  import { checkBrowser, getModlists } from "$lib/requests";
</script>

<div class="flex gap-2 mt-2 justify-center">
  <Button color="dark" on:click={getModlists}>Update lists</Button>
  <Button color="red" on:click={checkBrowser}>Check browser</Button>
</div>
