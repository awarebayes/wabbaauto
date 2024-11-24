<script lang="ts">
  import {
    modlists,
    chromeRunning,
    loversLoggedIn,
    nexusLoggedIn,
  } from "$lib/stores";
</script>

<div class="grid grid-cols-4 divide-dashed divide-x">
  <div class="text-center">Loaded {$modlists.length} modlists</div>
  <div class="text-center">
    Chrome: {$chromeRunning ? "Running" : "Not running"}
  </div>
  <div class="text-center">
    Nexus: {$nexusLoggedIn ? "Logged in" : "Not logged in"}
  </div>
  <div class="text-center">
    LoversLab: {$loversLoggedIn ? "Logged in" : "Not logged in"}
  </div>
</div>
