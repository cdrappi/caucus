import { API_HOST, postAuthJson } from "../util";

function validateLoginInputs(loginType, login, password) {
    if (login && password) {
        return { valid: true };
    } else {
        let message = "";
        if (!login && !password) {
            message = `${loginType} and password cannot be blank`;
        } else if (!login) {
            message = `${loginType} cannot be blank`;
        } else if (!password) {
            message = "Password cannot be blank";
        }
        return { valid: false, message: message };
    }
}

function fetchLogin(username, password) {
    return postAuthJson(`${API_HOST}/ums/login`, { username: username, password: password })
}


export { fetchLogin, validateLoginInputs };