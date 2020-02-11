<script>
  export let precincts, selectedPrecinct;
</script>

<select bind:value={selectedPrecinct}>
  <option value={undefined}>Select</option>
  {#each precincts as precinct}
    <option value={precinct.precinct}>{precinct.precinct}</option>
  {/each}
</select>
