# Get the total of public currencies
GET {{api_v1}}/currency
HTTP 200
[Captures]
total: jsonpath "$['_metadata']['total']"

# Get the currency with id 1
GET {{api_v1}}/currency/1
HTTP 200
[Asserts]
body == "{\"id\":1,\"name\":\"U.S. dollar\",\"symbol\":\"US$\",\"iso_code\":\"USD\",\"decimal_places\":2,\"user\":null}"

# Create a new currency
POST {{api_v1}}/currency
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "name": "Test Euro",
  "symbol": "€€",
  "iso_code": "EUR",
  "decimal_places": 2
}
HTTP 201
[Captures]
id: jsonpath "$['id']"

# Try to get that currency with no token
GET {{api_v1}}/currency/{{id}}
HTTP 404

# Try to get that currency with a user token
GET {{api_v1}}/currency/{{id}}
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
body == "{\"id\":{{id}},\"name\":\"Test Euro\",\"symbol\":\"€€\",\"iso_code\":\"EUR\",\"decimal_places\":2,\"user\":1}"

# Get the total of amount of currencies and check if it has increased
GET {{api_v1}}/currency
Authorization: Bearer {{admin_token}}
HTTP 200
[Captures]
new_total: jsonpath "$['_metadata']['total']"
[Asserts]
jsonpath "$['_metadata']['total']" > {{total}}

# Update the currency
PATCH {{api_v1}}/currency/{{id}}
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "name": "Test Euro Updated",
  "symbol": "€€€",
  "iso_code": "EUR",
  "decimal_places": 2
}
HTTP 200

# Get the currency and check if it has been updated
GET {{api_v1}}/currency/{{id}}
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
body == "{\"id\":{{id}},\"name\":\"Test Euro Updated\",\"symbol\":\"€€€\",\"iso_code\":\"EUR\",\"decimal_places\":2,\"user\":1}"

# Check if the total of currencies is still the same
GET {{api_v1}}/currency
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
jsonpath "$['_metadata']['total']" == {{new_total}}

# Delete the currency
DELETE {{api_v1}}/currency/{{id}}
Authorization: Bearer {{admin_token}}
HTTP 204

# Try to get that currency again
GET {{api_v1}}/currency/{{id}}
Authorization: Bearer {{admin_token}}
HTTP 404

# Check if the total of currencies has decreased
GET {{api_v1}}/currency
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
jsonpath "$['_metadata']['total']" == {{total}}
