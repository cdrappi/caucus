<script>
  import LoggedInApp from "./components/LoggedInApp.svelte";
  import LoggedOutApp from "./components/LoggedOutApp.svelte";
  import { getToken } from "./util/auth";

  import { fetchWhoseJson } from "../../../clients/js/ums/user";

  let isLoggedIn = getIsLoggedIn();

  function getIsLoggedIn() {
    let token = getToken();
    if (token === null) {
      return false;
    }
    let isLoggedInValue = false;
    let isValidToken = fetchWhoseJson(token)
      .then(res => res.json())
      .then(json => {
        if (json.success) {
          isLoggedInValue = true;
        }
      })
      .catch(reason => {
        console.log(`Failed to see if user is logged in: ${reason}`);
      });

    return isLoggedInValue;
  }
</script>

{#if isLoggedIn}
  <LoggedInApp bind:isLoggedIn />
{:else}
  <LoggedOutApp bind:isLoggedIn />
{/if}
