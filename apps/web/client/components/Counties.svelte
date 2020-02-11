<script>
  export let counties, selectedCounty;
</script>

<select bind:value={selectedCounty}>
  <option value={undefined}>Select</option>
  {#each counties as county}
    <option value={county.county}>{county.county}</option>
  {/each}
</select>
