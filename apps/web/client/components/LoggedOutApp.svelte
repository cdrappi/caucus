<script>
  import Register from "../components/Register";
  export let isLoggedIn;
</script>

<style>
  div {
    color: red;
  }
</style>

<div>LOGGED IN? {isLoggedIn}</div>
<Register bind:isLoggedIn />
