import { API_HOST, getJson } from "../util"

function getCountyPrecincts(county, jwt) {
    return getJson(`${API_HOST}/caucus/counties/${county}/precincts`, jwt)
}

export { getCountyPrecincts };