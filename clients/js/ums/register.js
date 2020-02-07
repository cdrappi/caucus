import { API_HOST, postAuthJson } from "../util";

function fetchRegister(username, password) {
    return postAuthJson(`${API_HOST}/ums/register`, { username: username, password: password })
}

export { fetchRegister };