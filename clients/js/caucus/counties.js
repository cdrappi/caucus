import { API_HOST, getJson } from "../util"

function getCountyPrecincts(stateCode, jwt) {
    return getJson(`${API_HOST}/caucus/states/${stateCode}/counties/`, jwt)
}

export { getCountyPrecincts };