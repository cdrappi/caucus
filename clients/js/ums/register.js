import { API_HOST, postAuthJson } from "../util";


function fetchRegisterPhone(phone) {
    // phone, e.g. "17327706758" (no leading "+")
    return postAuthJson(`${API_HOST}/ums/register/phone`, { phone: phone });
}

function fetchRegisterVerifyPhone(phone, code, bookId = 1) {
    // phone (str) e.g. "17327706758" (no leading "+")
    // code (str) e.g. '012345'
    return postAuthJson(`${API_HOST}/ums/register/verify`, { phone: phone, code: code, book_id: bookId });
}


export { fetchRegisterPhone, fetchRegisterVerifyPhone };