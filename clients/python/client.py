import os

import requests

api_host = os.environ['API_HOST']


class Client:

    def __init__(self, jwt: str):
        self.jwt = jwt


def get_index():
    return requests.get(url=f'{api_host}')


def _login(password: str, **login):
    return requests.post(
        url=f'{api_host}/',
        json={'password': password, **login}
    )


def login_with_username(username: str, password: str):
    return _login(password, username=username)


def login_with_email(email: str, password: str):
    return _login(password, email=email)


def login_with_phone(phone: str, password: str):
    return _login(password, phone=phone)
