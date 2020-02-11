<script>
  import {
    validateLoginInputs,
    fetchLogin
  } from "../../../../clients/js/ums/login";

  import { setToken } from "../util/auth";

  export let isLoggedIn;

  let username = "";
  let password = "";
  let message = "Login to the caucus app";

  function onClick() {
    let validation = validateLoginInputs(username, password);
    if (validation.valid) {
      fetchLogin(username, password)
        .then(res => res.json())
        .then(json => {
          if (json.success) {
            setToken(json.jwt);
            isLoggedIn = true;
          } else {
            message = json.error;
          }
        });
    } else {
      message = validation.message;
    }
  }
</script>

<div>
  <p>{message}</p>
  <div>
    <input bind:value={username} placeholder="username" />
  </div>
  <div>
    <input bind:value={password} placeholder="password" type="password" />
  </div>
  <button on:click={onClick}>LOGIN</button>
</div>
