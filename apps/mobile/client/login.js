import { API_HOST, postAuthJson } from "./util";

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

function fetchLogin(loginType, jsonBody) {
    return postAuthJson(`${API_HOST}/login/${loginType}/`, jsonBody)
}

function fetchLoginWith(loginType, login, password) {
    return fetchLogin(loginType, {
        [loginType]: login,
        password: password
    })
}

export { validateLoginInputs, fetchLoginWith };