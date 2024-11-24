<script>
  import { Alert } from "flowbite-svelte";
  import { errorMsg } from "$lib/stores";
</script>

{#if $errorMsg != ""}
  <Alert border>
    <span class="font-medium">Something went wrong</span>
    {$errorMsg}
  </Alert>
{/if}
