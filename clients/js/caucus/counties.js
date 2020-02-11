import { API_HOST, getJson } from "../util"

function getStateCounties(stateCode, jwt) {
    return getJson(`${API_HOST}/caucus/states/${stateCode}/counties`, jwt)
}

export { getStateCounties };