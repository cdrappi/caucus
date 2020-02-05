import { API_HOST, getJson, postJson } from "../util";

function getEvent(token, eventId) {
    return getJson(`${API_HOST}/book/events/${eventId}`, token)
}

function getActiveEvents(token, count = 10) {
    return getJson(`${API_HOST}/book/events/active?count=${count}`, token);
}


function getLeagueEvents(token, league) {
    return getJson(`${API_HOST}/book/events/league=${league}`, token)
}


function getEventMarkets(token, eventId) {
    return getJson(`${API_HOST}/book/events/${eventId}/markets`, token)
}

function getEventWagers(token, eventId) {
    return getJson(`${API_HOST}/book/events/${eventId}/wagers`, token)
}

function getEventEquity(token, eventId) {
    return getJson(`${API_HOST}/book/events/${eventId}/equity`, token)
}

function cashoutEvent(token, eventId) {
    return postJson(`${API_HOST}/book/events/${eventId}/cashout`, {}, token);
}

export { getActiveEvents, getLeagueEvents, getEvent, getEventMarkets, getEventEquity, getEventWagers, cashoutEvent }

