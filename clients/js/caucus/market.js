import { API_HOST, getJson, postJson } from "../util";

function getFeaturedMarkets(token, count) {
    return getJson(`${API_HOST}/book/markets/featured?count=${count}`, token)
}

function getMarket(token, marketId) {
    return getJson(`${API_HOST}/book/markets/${marketId}`, token)
}

function getLiveWagers(token) {
    return getJson(`${API_HOST}/book/wagers/live`, token)
}

function placeLimitOrder(token, buyOrSell, penniesRisked, penniesPayout) {
    return postJson(
        `${API_HOST}/book/markets/${marketId}/market_order/${buyOrSell}/pennies_risked=${penniesRisked}&pennies_payout=${penniesPayout}`,
        {},
        token
    )
}

function placeMarketOrder(token, buyOrSell, penniesRisked) {
    return postJson(
        `${API_HOST}/book/markets/${marketId}/market_order/${buyOrSell}/pennies_risked=${penniesRisked}`,
        {},
        token
    )
}

export { getLiveWagers, getFeaturedMarkets, getMarket, placeMarketOrder, placeLimitOrder };
