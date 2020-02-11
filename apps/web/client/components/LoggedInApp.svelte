<script>
  import { getCaucuses } from "../../../../clients/js/caucus/caucuses";
  import { getToken } from "../util/auth";
  import States from "./States";

  export let isLoggedIn;

  let jwt = getToken();
  let states = [];
  let selectedStateCode = undefined;

  onRefresh();

  function onRefresh() {
    getCaucuses(jwt)
      .then(res => res.json())
      .then(json => {
        states = json.data;
      })
      .catch(reason => {
        console.log(`Error getting caucuses: ${reason}`);
      });
  }
</script>

<style>
  div {
    color: green;
  }
</style>

<div>LOGGED IN? {isLoggedIn}</div>
<States {states} {selectedStateCode} />
