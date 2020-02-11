<script>
  export let states, selectedStateCode;
</script>

<select bind:value={selectedStateCode}>
  <option value={undefined}>Select</option>
  {#each states as state}
    <option value={state.state_code}>{state.state_code}</option>
  {/each}
</select>
