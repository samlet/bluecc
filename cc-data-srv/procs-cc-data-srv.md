## start
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


