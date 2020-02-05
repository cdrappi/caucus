import { API_HOST, getJson } from "../util";

function fetchWhoseJson(token) {
    return getJson(`${API_HOST}/ums/me`, token);
}

export { fetchWhoseJson };