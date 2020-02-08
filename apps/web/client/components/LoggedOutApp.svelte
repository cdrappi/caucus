<script>
  import Register from "../components/Register";
  import Login from "../components/Login";
  export let isLoggedIn;

  let isLoggingIn = true;
</script>

<div>
  <div>
    <button
      on:click={() => {
        isLoggingIn = true;
      }}
      style={`background-color: ${isLoggingIn ? 'yellow' : 'white'}`}>
      LOGIN
    </button>
    <button
      on:click={() => {
        isLoggingIn = false;
      }}
      style={`background-color: ${isLoggingIn ? 'white' : 'yellow'}`}>
      REGISTER
    </button>
  </div>
  {#if isLoggingIn}
    <Login bind:isLoggedIn />
  {:else}
    <Register bind:isLoggedIn />
  {/if}
</div>
