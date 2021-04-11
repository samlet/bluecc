import pytest
from sanic import Blueprint, Sanic
from sanic.response import json, text

from sanic_jwt import Claim, exceptions, Initialize
from sanic_jwt.decorators import protected

Sanic.test_mode = True

class User:
    def __init__(self, id, username, password):
        self.user_id = id
        self.username = username
        self.password = password

    def to_dict(self):
        properties = ["user_id", "username"]
        return {prop: getattr(self, prop, None) for prop in properties}


@pytest.fixture
def users():
    yield [User(1, "user1", "abcxyz"), User(2, "user2", "abcxyz")]


@pytest.fixture
def username_table(users):
    yield {u.username: u for u in users}


@pytest.fixture
def userid_table(users):
    yield {u.user_id: u for u in users}


@pytest.fixture
def authenticate(username_table):
    async def authenticate(request, *args, **kwargs):
        username = request.json.get("username", None)
        password = request.json.get("password", None)

        if not username or not password:
            raise exceptions.AuthenticationFailed(
                "Missing username or password."
            )

        user = username_table.get(username, None)
        if user is None:
            raise exceptions.AuthenticationFailed("User not found.")

        if password != user.password:
            raise exceptions.AuthenticationFailed("Password is incorrect.")

        return user

    yield authenticate


@pytest.fixture
def retrieve_user(userid_table):
    async def retrieve_user(request, payload, *args, **kwargs):
        if payload:
            user_id = payload.get("user_id", None)
            if user_id is not None:
                return userid_table.get(user_id)

        else:
            return None

    yield retrieve_user


@pytest.fixture
def retrieve_user_secret():
    async def retrieve_user_secret(user_id, **kwargs):
        return f"foobar<{user_id}>"

    yield retrieve_user_secret


@pytest.fixture
def app(username_table, authenticate):

    sanic_app = Sanic("sanic-jwt-test")
    sanic_jwt = Initialize(sanic_app, authenticate=authenticate)

    @sanic_app.route("/")
    async def helloworld(request):
        return json({"hello": "world"})

    @sanic_app.route("/protected")
    @protected()
    async def protected_request(request):
        return json({"protected": True})

    @sanic_app.route("/options", methods=["OPTIONS"])
    @protected()
    async def protected_request_options(request):
        return text("", status=204)

    @sanic_app.route("/protected/<verify:int>")
    @protected()
    def protected_regression_verify(request, verify):
        """
        for regression test see
        https://github.com/ahopkins/sanic-jwt/issues/59#issuecomment-380034269
        """
        return json({"protected": True})

    yield (sanic_app, sanic_jwt)

# ......

@pytest.fixture
def app_with_extended_exp(username_table, authenticate):

    sanic_app = Sanic("sanic-jwt-test")
    sanic_jwt = Initialize(
        sanic_app, authenticate=authenticate, expiration_delta=(60 * 60)
    )

    @sanic_app.route("/")
    async def helloworld(request):
        return json({"hello": "world"})

    @sanic_app.route("/protected")
    @protected()
    async def protected_request(request):
        return json({"protected": True})

    yield (sanic_app, sanic_jwt)

