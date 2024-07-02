# Get the id from the second currency
GET {{api_v1}}/currency
Authorization: Bearer {{admin_token}}
HTTP 200
[Captures]
currency_id: jsonpath "$['data'][1]['id']"

# Try to get all templates without supplying a token
GET {{api_v1}}/transaction/template
HTTP 401
[Asserts]
body == "{\"api_code\":{\"code\":1006,\"message\":\"No bearer token provided!\"},\"details\":\"No token provided!\",\"reference\":null}"

# Assert that there are now templates
GET {{api_v1}}/transaction/template
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
jsonpath "$['_metadata']['total']" == 0

# Create first test account
POST {{api_v1}}/account
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "name": "Test Account",
  "description": null,
  "iban": null,
  "original_balance": 10000,
  "currency_id": {{currency_id}}
}
HTTP 201
[Captures]
test_acc1_id: jsonpath "$['id']"

# Create second test account
POST {{api_v1}}/account
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "name": "Test Account 2",
  "description": null,
  "iban": null,
  "original_balance": 10000,
  "currency_id": {{currency_id}}
}
HTTP 201
[Captures]
test_acc2_id: jsonpath "$['id']"

# Create a deposit template
POST {{api_v1}}/transaction/template
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "source_id": null,
  "destination_id": {{test_acc1_id}},
  "amount": 1000,
  "currency_id":  {{currency_id}},
  "name": "10€ Deposit",
  "description": "A longer description bla bla",
  "budget_id": null
}
HTTP 201
[Captures]
template_id: jsonpath "$['id']"

# Create a deposit transaction
POST {{api_v1}}/transaction/from-template
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
    "template_id": {{template_id}},
    "executed_at": "2024-05-21T21:03:23.251Z"
}
HTTP 201
[Captures]
deposit_transaction_id: jsonpath "$['id']"

# Assert that test account 1 has now 11000
GET {{api_v1}}/account/{{test_acc1_id}}
Authorization: Bearer {{admin_token}}
[Options]
delay: 100
HTTP 200
[Asserts]
jsonpath "$['balance']" == 11000

# Update the template
PATCH {{api_v1}}/transaction/template/{{template_id}}
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "source_id": {{test_acc1_id}},
  "destination_id": {{test_acc2_id}},
  "amount": 2000,
  "currency_id": {{currency_id}},
  "name": "20€ Deposit",
  "description": "A longer description bla bla",
  "budget_id": null
}
HTTP 200

# Assert that updating the template did not affect the transaction
GET {{api_v1}}/account/{{test_acc1_id}}
Authorization: Bearer {{admin_token}}
[Options]
delay: 100
HTTP 200
[Asserts]
jsonpath "$['balance']" == 11000

# Assert that the template was updated
GET {{api_v1}}/transaction/template/{{template_id}}
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
jsonpath "$['id']" == {{template_id}}
jsonpath "$['source_id']" == {{test_acc1_id}}
jsonpath "$['destination_id']" == {{test_acc2_id}}
jsonpath "$['amount']" == 2000
jsonpath "$['currency_id']" == {{currency_id}}
jsonpath "$['name']" == "20€ Deposit"
jsonpath "$['description']" == "A longer description bla bla"
jsonpath "$['budget_id']" == null

# Delete the template
DELETE {{api_v1}}/transaction/template/{{template_id}}
Authorization: Bearer {{admin_token}}
HTTP 204

# Assert that the transaction is still there
GET {{api_v1}}/transaction/{{deposit_transaction_id}}
Authorization: Bearer {{admin_token}}
HTTP 200

# Assert that the template is gone
GET {{api_v1}}/transaction/template/{{template_id}}
Authorization: Bearer {{admin_token}}
HTTP 404

# Delete the first account
DELETE {{api_v1}}/account/{{test_acc1_id}}
Authorization: Bearer {{admin_token}}
HTTP 204

# Delete the second account
DELETE {{api_v1}}/account/{{test_acc2_id}}
Authorization: Bearer {{admin_token}}
HTTP 204
