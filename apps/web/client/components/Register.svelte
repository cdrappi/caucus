<script>
  import { validateLoginInputs } from "../../../../clients/js/ums/login";
  import { fetchRegister } from "../../../../clients/js/ums/register";
  import { setToken } from "../util/auth";
  export let isLoggedIn;

  let username = "";
  let password = "";
  let message = "Register with any username but make a strong password";

  function onClick() {
    let validation = validateLoginInputs(username, password);
    if (validation.valid) {
      fetchRegister(username, password)
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
  <p>{message}</p>
  <div>
    <input bind:value={username} placeholder="username" />
  </div>
  <div>
    <input bind:value={password} placeholder="password" type="password" />
  </div>
  <button on:click={onClick}>REGISTER</button>
</div>
