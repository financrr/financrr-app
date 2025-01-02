# Fail to get current session when no token is provided
GET {{api_v1}}/session/current
HTTP 401

# Get current session
GET {{api_v1}}/session/current
Authorization: Bearer {{admin_token}}
HTTP 200

# Create a new session
POST {{api_v1}}/session
Content-Type: application/json
{
  "username": "admin",
  "password": "Financrr123",
  "name": "test_session"
}
HTTP 201
[Captures]
new_admin_token: jsonpath "$['token']"
new_session_id: jsonpath "$['id']"

# Get public session by id
GET {{api_v1}}/session/{{new_session_id}}
Authorization: Bearer {{admin_token}}
HTTP 200

# Delete current session
DELETE {{api_v1}}/session/current
Authorization: Bearer {{new_admin_token}}

# Create a new session
POST {{api_v1}}/session
Content-Type: application/json
{
  "username": "admin",
  "password": "Financrr123",
  "name": "test_session"
}
HTTP 201
[Captures]
new_admin_token: jsonpath "$['token']"
new_session_id: jsonpath "$['id']"
expires_at: jsonpath "$['expires_at']" toDate "%+" format "%s" toInt

# Refresh session
PATCH {{api_v1}}/session/refresh
Authorization: Bearer {{new_admin_token}}
[Options]
delay: 1000
HTTP 200
[Asserts]
jsonpath "$['expires_at']" toDate "%+" format "%s" toInt > {{expires_at}}

# Delete session by id
DELETE {{api_v1}}/session/{{new_session_id}}
Authorization: Bearer {{admin_token}}
HTTP 204
