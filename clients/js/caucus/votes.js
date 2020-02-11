import { API_HOST, getJson } from "../util"

function getPrecinctVotes(jwt) {
    return getJson(`${API_HOST}/caucus/orgs/${org}/precincts/${precinct}`, jwt)
}

export { getPrecinctVotes };