<script>
  export let states, selectedStateCode;
</script>

<select bind:value={selectedStateCode}>
  {#each states as state}
    <option value={state.state_code}>{state.state_code}</option>
  {/each}
</select>
