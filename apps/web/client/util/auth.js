function getToken() {
    return localStorage.getItem("token");
}

function deleteToken() {
    localStorage.removeItem("token");
}

function setToken(token) {
    localStorage.setItem("token", token);
}


export { getToken, deleteToken, setToken };