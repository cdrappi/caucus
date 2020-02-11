<script>
  import { getCaucuses } from "../../../../clients/js/caucus/caucuses";
  import { getStateCounties } from "../../../../clients/js/caucus/counties";
  import { getToken } from "../util/auth";
  import States from "./States";

  export let isLoggedIn;

  let jwt = getToken();
  let states = [];
  let counties = [];

  let selectedStateCode = undefined;
  let selectedCountyCode = undefined;

  onRefresh();

  $: getCounties(selectedStateCode).then(result => {
    counties = result;
  });
  $: console.log(`counties: ${JSON.stringify(counties)}`);

  function onRefresh() {
    getCaucuses(jwt)
      .then(res => res.json())
      .then(json => {
        states = json.data;
        if (states.length > 0) {
          selectedStateCode = states[0].state_code;
        }
      })
      .catch(reason => {
        console.log(`Error getting caucuses: ${reason}`);
      });
  }

  async function getCounties(stateCode) {
    let stateCounties = [];

    getStateCounties(stateCode, jwt)
      .then(res => res.json())
      .then(json => {
        if (json.success) {
          stateCounties = json.data;
        }
      })
      .catch(reason =>
        console.log(`Failed to get counties for ${stateCode}: ${reason}`)
      );
    return stateCounties;
  }
</script>

<style>
  div {
    color: green;
  }
</style>

<div>LOGGED IN? {isLoggedIn}</div>
<States {states} {selectedStateCode} />
