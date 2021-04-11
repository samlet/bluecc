from datetime import datetime, timedelta
import jwt
from freezegun import freeze_time

def test_exp_configuration(app_with_extended_exp):
    sanic_app, sanic_jwt = app_with_extended_exp
    _, response = sanic_app.test_client.post(
        "/auth", json={"username": "user1", "password": "abcxyz"}
    )

    access_token = response.json.get(
        sanic_jwt.config.access_token_name(), None
    )
    payload = jwt.decode(
        access_token,
        sanic_jwt.config.secret(),
        algorithms=sanic_jwt.config.algorithm(),
        verify=False,
    )
    exp = payload.get("exp", None)
    exp = datetime.utcfromtimestamp(exp)

    with freeze_time(datetime.utcnow() + timedelta(seconds=(60 * 35))):
        assert isinstance(exp, datetime)
        assert datetime.utcnow() < exp

        _, response = sanic_app.test_client.get(
            "/protected",
            headers={"Authorization": "Bearer {}".format(access_token)},
        )
        assert response.status == 200

