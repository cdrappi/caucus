import { API_HOST, getJson } from "../util"

function getCaucuses(jwt) {
    return getJson(`${API_HOST}/caucus/states`, jwt)
}

export { getCaucuses };