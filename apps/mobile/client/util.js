const API_HOST = process.env.API_HOST || "http://127.0.0.1:8000";

function fetchAuthJson(method, url, obj) {
    return fetch(url, {
        method: method,
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(obj)
    });
}

function fetchJson(method, url, obj, jwt) {
    return fetch(url, {
        method: method,
        headers: {
            "Content-Type": "application/json",
            Authorization: `${jwt}`
        },
        body: JSON.stringify(obj)
    });
}


function fetchProtobuf(method, url, protobuf, jwt) {
    return fetch(url, {
        method: method,
        headers: {
            "Content-Type": "application/protobuf",
            Authorization: `${jwt}`
        },
        body: protobuf
    });
}

const getAuthJson = (url, obj) => fetchAuthJson("GET", url, obj);
const postAuthJson = (url, obj) => fetchAuthJson("POST", url, obj);
const getJson = (url, obj, token) => fetchJson("GET", url, obj, token);
const postJson = (url, obj, token) => fetchJson("POST", url, obj, token);
const getProtobuf = (url, protobuf, token) => fetchProtobuf("GET", url, protobuf, token);
const postProtobuf = (url, protobuf, token) => fetchProtobuf("POST", url, protobuf, token);

export { API_HOST, getAuthJson, getJson, postJson, postAuthJson, getProtobuf, postProtobuf };