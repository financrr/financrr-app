# Get the id from the second currency
GET {{api_v1}}/currency
Authorization: Bearer {{admin_token}}
HTTP 200
[Captures]
currency_id: jsonpath "$['data'][1]['id']"

# Try to get all transaction without supplying a token
GET {{api_v1}}/transaction
HTTP 401
[Asserts]
body == "{\"api_code\":{\"code\":1006,\"message\":\"No bearer token provided!\"},\"details\":\"No token provided!\",\"reference\":null}"

# Assert that there are no transactions
GET {{api_v1}}/transaction
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

# Create a deposit transaction
POST {{api_v1}}/transaction
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "source_id": null,
  "destination_id": {{test_acc1_id}},
  "amount": 1000,
  "currency_id": 2,
  "name": "10€ Deposit",
  "description": null,
  "budget_id": null,
  "executed_at": "2024-05-19T15:39:03.013Z"
}

# Assert that account balance is now 11000
GET {{api_v1}}/account/{{test_acc1_id}}
Authorization: Bearer {{admin_token}}
[Options]
delay: 100
HTTP 200
[Asserts]
jsonpath "$['balance']" == 11000

# Create a withdrawal transaction
POST {{api_v1}}/transaction
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "source_id": {{test_acc1_id}},
  "destination_id": null,
  "amount": 1000,
  "currency_id": 2,
  "name": "10€ Withdrawal",
  "description": null,
  "budget_id": null,
  "executed_at": "2024-05-19T15:39:03.013Z"
}

# Assert that account balance is now 10000
GET {{api_v1}}/account/{{test_acc1_id}}
Authorization: Bearer {{admin_token}}
[Options]
delay: 100
HTTP 200
[Asserts]
jsonpath "$['balance']" == 10000

# Create a transfer transaction
POST {{api_v1}}/transaction
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "source_id": {{test_acc1_id}},
  "destination_id": {{test_acc2_id}},
  "amount": 1000,
  "currency_id": {{currency_id}},
  "name": "10€ Transfer",
  "description": null,
  "budget_id": null,
  "executed_at": "2024-05-19T15:39:03.013Z"
}
HTTP 201
[Captures]
transfer_transaction_id: jsonpath "$['id']"

# Assert that test1 account balance is now 9000
GET {{api_v1}}/account/{{test_acc1_id}}
Authorization: Bearer {{admin_token}}
[Options]
delay: 100
HTTP 200
[Asserts]
jsonpath "$['balance']" == 9000

# Assert that test2 account balance is now 11000
GET {{api_v1}}/account/{{test_acc2_id}}
Authorization: Bearer {{admin_token}}
[Options]
delay: 100
HTTP 200
[Asserts]
jsonpath "$['balance']" == 11000

# Assert that there are 3 transactions
GET {{api_v1}}/transaction
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
jsonpath "$['_metadata']['total']" == 3

# Update transfer transaction
PATCH {{api_v1}}/transaction/{{transfer_transaction_id}}
Authorization: Bearer {{admin_token}}
Content-Type: application/json
{
  "source_id": {{test_acc1_id}},
  "destination_id": {{test_acc2_id}},
  "amount": 1500,
  "currency_id": 2,
  "name": "10€ Transfer",
  "description": null,
  "budget_id": null,
  "executed_at": "2024-05-19T15:39:03.013Z"
}

# Assert that test1 account balance is now 7500
GET {{api_v1}}/account/{{test_acc1_id}}
Authorization: Bearer {{admin_token}}
[Options]
delay: 200
HTTP 200
[Asserts]
jsonpath "$['balance']" == 8500

# Assert that test2 account balance is now 12500
GET {{api_v1}}/account/{{test_acc2_id}}
Authorization: Bearer {{admin_token}}
[Options]
delay: 100
HTTP 200
[Asserts]
jsonpath "$['balance']" == 11500

# Assert that there are 3 transactions
GET {{api_v1}}/transaction
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
jsonpath "$['_metadata']['total']" == 3

# Delete the second test account
DELETE {{api_v1}}/account/{{test_acc2_id}}
Authorization: Bearer {{admin_token}}
HTTP 204

# Assert that there are 2 transactions
GET {{api_v1}}/transaction
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
jsonpath "$['_metadata']['total']" == 2

# Delete the first test account
DELETE {{api_v1}}/account/{{test_acc1_id}}
Authorization: Bearer {{admin_token}}
HTTP 204

# Assert that there are no transactions
GET {{api_v1}}/transaction
Authorization: Bearer {{admin_token}}
HTTP 200
[Asserts]
jsonpath "$['_metadata']['total']" == 0
