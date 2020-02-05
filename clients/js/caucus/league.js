import { API_HOST, getJson } from "../util";

function getActiveLeagues(token) {
    return getJson(`${API_HOST}/book/leagues`, token)
}

export { getActiveLeagues };