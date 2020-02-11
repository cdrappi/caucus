
import { API_HOST, postJson } from "../util"

function updatePrecinctVotes(jwt) {
    return postJson(`${API_HOST}/caucus/orgs/${org}/precincts/${precinct}/votes`, {}, jwt)
}

function updatePrecinctTurnout(org, precinct, turnout, jwt) {
    return postJson(`${API_HOST}/caucus/orgs/${org}/precincts/${precinct}/turnout?${turnout}`, {}, jwt)
}
export { updatePrecinctVotes, updatePrecinctTurnout };