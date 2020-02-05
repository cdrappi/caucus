import { API_HOST, postAuthJson } from "./util";


function fetchRegisterPhone(phone) {
    // phone, e.g. "17327706758" (no leading "+")
    return postAuthJson(`${API_HOST}/register/phone/`, { phone: phone });
}

function fetchRegisterVerifyPhone(phone, code) {
    // phone (str) e.g. "17327706758" (no leading "+")
    // code (str) e.g. '012345'
    return postAuthJson(`${API_HOST}/register/verify/`, { phone: phone, code: code });
}

function fetchRegisterEmail(email, password) {
    return postAuthJson(`${API_HOST}/register/email/`, { email: email, password: password })
}

export { fetchRegisterPhone, fetchRegisterEmail, fetchRegisterVerifyPhone };