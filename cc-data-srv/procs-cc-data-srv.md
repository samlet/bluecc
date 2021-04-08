## start with data-srv
* POST: http://localhost:3030/api/v1/create

```json
{
	"entity": "SecurityGroupPermission",
	"values": {
		"groupId": "VIEWADMIN",
		"permissionId": "cc_PAY_INFO_VIEW",
		"fromDate": "2001-05-13 12:00:00.0"
	}
}
```

## dummy server
There are three hard-coded users:

* sibylle (Admin)
* herbert (Member)
* gordon (Anonymous)

Endpoints:

* `POST /login` with the username only (`{ "name": "sibylle" }`), returns a session token
* `/logout` invalidates the session token
* `/member` only members can access this (sibylle & herbert)
* `/admin` only admins can access this (herbert)

You can run this using `cargo run -- dummy`, which starts a server on http://localhost:8080

```bash
curl -X POST http://localhost:8080/login -d '{ "name": "sibylle" }' -H "content-type: application/json"
=> $TOKEN
curl http://localhost:8080/member -H "authorization: Bearer $TOKEN" -H "content-type: application/json"
=> 200
curl http://localhost:8080/admin -H "authorization: Bearer $TOKEN" -H "content-type: application/json"
=> 200

curl -X POST http://localhost:8080/login -d '{ "name": "herbert" }' -H "content-type: application/json"
=> $TOKEN
curl http://localhost:8080/member -H "authorization: Bearer $TOKEN" -H "content-type: application/json"
=> 200
curl http://localhost:8080/admin -H "authorization: Bearer $TOKEN" -H "content-type: application/json"
=> 401

curl -X POST http://localhost:8080/login -d '{ "name": "gordon" }' -H "content-type: application/json"
=> $TOKEN
curl http://localhost:8080/member -H "authorization: Bearer $TOKEN" -H "content-type: application/json"
=> 401
curl http://localhost:8080/admin -H "authorization: Bearer $TOKEN" -H "content-type: application/json"
=> 401
```

## jwt auth server

### Login

```bash
curl http://localhost:8000/login -d '{"email": "user@userland.com", "pw": "1234"}' -H 'Content-Type: application/json'

{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxIiwicm9sZSI6IlVzZXIiLCJleHAiOjE2MDMxMzQwODl9.dWnt5vfcGdwypEQUr3bLMrZYfdyxj3v6-io6VREWHXebMUCKBddf9xGcz4vHrCXruzx42zrS3Kygiqw3xV8W-A"}
```


### User Endpoint

```bash
curl http://localhost:8000/user -H 'Authorization: Bearer $TOKEN' -H 'Content-Type: application/json'

Hello User 1

curl http://localhost:8000/admin -H 'Authorization: Bearer $TOKEN' -H 'Content-Type: application/json'

{"message":"no permission","status":"401 Unauthorized"}
```


### Admin Endpoint

```bash
curl http://localhost:8000/login -d '{"email": "admin@adminaty.com", "pw": "4321"}' -H 'Content-Type: application/json'

{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIyIiwicm9sZSI6IkFkbWluIiwiZXhwIjoxNjAzMTM0MjA1fQ.uYglVKRvb3h0bDC0Uz8FwGTu4v__Rl3toVI9fMI4_IT8keKde_SZRFQ4ii_PKzI4wjmDsZlnpULe6Tg0vWfEnw"}

curl http://localhost:8000/admin -H 'Authorization: Bearer $TOKEN' -H 'Content-Type: application/json'

Hello Admin 2

curl http://localhost:8000/user -H 'Authorization: Bearer $TOKEN' -H 'Content-Type: application/json'

Hello User 2
```

