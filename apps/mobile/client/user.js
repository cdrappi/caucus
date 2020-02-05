import { API_HOST, getAuthJson, getProtobuf, postJson } from "./util";

function fetchWhoseJson(token) {
    return getAuthJson(`${API_HOST}/user/me`, {}, token);
}

function fetchWhoseProtobuf(token) {
    return getProtobuf(`${API_HOST}/user/me`, {}, token);
}

function fetchChangePassword(password, token) {
    return postJson(`${API_HOST}/user/password/`, { password: password }, token);
}

function fetchChangeUsername(username, token) {
    return postJson(`${API_HOST}/user/username/`, { username: username }, token);
}

function fetchChangeEmail(email, token) {
    return postJson(`${API_HOST}/user/email/`, { email: email }, token);
}

export {
    fetchWhoseJson,
    fetchWhoseProtobuf,
    fetchChangeEmail,
    fetchChangePassword,
    fetchChangeUsername
};