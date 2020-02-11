<script>
  import { getCaucuses } from "../../../../clients/js/caucus/caucuses";
  import { getStateCounties } from "../../../../clients/js/caucus/counties";
  import { getCountyPrecincts } from "../../../../clients/js/caucus/precincts";
  import { getToken } from "../util/auth";
  import States from "./States";
  import Counties from "./Counties";
  import Precincts from "./Precincts";

  export let isLoggedIn;

  let jwt = getToken();
  let states = [];
  let counties = [];
  let precincts = [];

  let selectedStateCode = undefined;
  let selectedCounty = undefined;
  let selectedPrecinct = undefined;

  onRefresh();

  $: getCounties(selectedStateCode).then(result => {
    counties = result;
  });

  $: getPrecincts(selectedCounty).then(result => {
    precincts = result;
  });

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
    if (stateCode === undefined) {
      return [];
    }

    let stateCounties = [];
    await getStateCounties(stateCode, jwt)
      .then(res => res.json())
      .then(json => {
        if (json.success) {
          stateCounties = json.data;
          // console.log(`sc: ${stateCounties}`);
        }
      })
      .catch(reason =>
        console.log(`Failed to get counties for ${stateCode}: ${reason}`)
      );
    return stateCounties;
  }

  async function getPrecincts(county) {
    if (county === undefined) {
      return [];
    }

    let countyPrecincts = [];
    await getCountyPrecincts(county, jwt)
      .then(res => res.json())
      .then(json => {
        if (json.success) {
          countyPrecincts = json.data;
        }
      })
      .catch(reason =>
        console.log(`Failed to get precincts for ${county}: ${reason}`)
      );
    return countyPrecincts;
  }
</script>

<style>
  div {
    color: green;
  }
</style>

<div>LOGGED IN? {isLoggedIn}</div>

<table>
  <tr>
    <td>State</td>
    <td>
      <States bind:states bind:selectedStateCode />
    </td>
  </tr>
  <tr>
    <td>County</td>
    <td>
      <Counties bind:counties bind:selectedCounty />
    </td>
  </tr>

  <tr>
    <td>Precinct</td>
    <td>
      <Precincts bind:precincts bind:selectedPrecinct />
    </td>
  </tr>
</table>
