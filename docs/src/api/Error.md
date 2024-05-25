# API Error

We provide a custom error object that is utilized throughout the library.

## Error and HTTP Codes

We employ semantic HTTP codes to signify the type of error that occurred, enabling more precise error handling.
<br>
If HTTP codes lack granularity, we offer a custom error code for more specific error handling scenarios. By default,
HTTP codes are used.

## Custom Error Codes

Custom error codes come into play when HTTP codes lack granularity. For instance, if you need to handle a particular
error in a specific manner, custom error codes allow for such tailored handling.
<br>
Custom error codes always consist of at least four digits to prevent conflicts with HTTP codes.
<br>
We call these custom error codes `Api Codes`.

## Api Codes

### Auth related

| Code | Description                  |
|------|------------------------------|
| 1000 | Invalid Session              |
| 1002 | Invalid credentials provided |
| 1003 | Unauthorized                 |
| 1004 | No bearer token provided     |

### User-causes errors

| Code | Description                 |
|------|-----------------------------|
| 1100 | Resource not found          |
| 1101 | Serialization error         |
| 1102 | Missing permissions         |
| 1103 | Error while parsing to cron |

### Validation errors

| Code | Description                   |
|------|-------------------------------|
| 1200 | JSON payload validation error | 

### Internal server errors

| Code | Description        |
|------|--------------------|
| 1300 | DB-Entitiy error   |
| 1301 | Database error     |
| 1302 | Redis error        |
| 1303 | Cron builder error |

### Misc errors

| Code | Description   |
|------|---------------|
| 9000 | Actix error   |
| 9999 | Unknown error |
