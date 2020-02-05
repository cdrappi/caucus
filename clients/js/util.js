const API_HOST = process.env.API_HOST || "http://127.0.0.1:8000";

function postAuthJson(url, obj) {
    return fetch(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(obj)
    });
}

function postJson(url, obj, jwt) {
    return fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            Authorization: `${jwt}`
        },
        body: JSON.stringify(obj)
    });
}


function postProtobuf(url, protobuf, jwt) {
    return fetch(url, {
        method: "POST",
        headers: {
            "Content-Type": "application/protobuf",
            Authorization: `${jwt}`
        },
        body: protobuf
    });
}


function getAuthJson(url) {
    return fetch(url, {
        method: "GET",
        headers: { "Content-Type": "application/json" },
    });
}

function getJson(url, jwt) {
    return fetch(url, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            Authorization: `${jwt}`
        },
    });
}


function getProtobuf(url, jwt) {
    return fetch(url, {
        method: "GET",
        headers: {
            "Content-Type": "application/protobuf",
            Authorization: `${jwt}`
        },
    });
}

export { API_HOST, getAuthJson, getJson, postJson, postAuthJson, getProtobuf, postProtobuf };