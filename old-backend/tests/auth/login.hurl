# Assert that the service is up and running
GET {{base_url}}/api/status/health
HTTP 200

# Login as admin
POST {{api_v1}}/session
{
  "username": "admin",
  "password": "Financrr123",
  "name": "test_session"
}
HTTP 201
