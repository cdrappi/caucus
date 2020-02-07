<script>
  import LoggedInApp from "./components/LoggedInApp.svelte";
  import LoggedOutApp from "./components/LoggedOutApp.svelte";
  import { getToken } from "./util/auth";

  let isLoggedIn = getIsLoggedIn();

  function getIsLoggedIn() {
    return false;
  }
</script>

{#if isLoggedIn}
  <LoggedInApp bind:isLoggedIn />
{:else}
  <LoggedOutApp bind:isLoggedIn />
{/if}
