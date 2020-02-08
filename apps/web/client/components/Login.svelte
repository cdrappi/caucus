<script>
  import {
    validateLoginInputs,
    fetchLogin
  } from "../../../../clients/js/ums/login";

  export let isLoggedIn, username, password;

  let message = "Login to the caucus app";

  function onClick() {
    validation = validateLoginInputs(username, password);
    if (validation.valid) {
      fetchLogin(username, password)
        .then(res => res.json())
        .then(json => {
          if (json.success) {
            setToken(json.token);
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
  <h3>LOGIN</h3>
  <p>{message}</p>
  <div>
    <input bind:value={username} placeholder="username" />
  </div>
  <div>
    <input bind:value={password} placeholder="password" type="password" />
  </div>
  <button on:click={onClick}>LOGIN</button>
</div>
