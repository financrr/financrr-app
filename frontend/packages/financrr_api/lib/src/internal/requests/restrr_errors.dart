import 'package:financrr_api/financrr_api.dart';
import 'package:financrr_api/src/internal/requests/responses/rest_response.dart';

enum RestrrError {
  /* Not thrown by the server directly, but still server-related */
  internalServerError(0000, 'Internal server error!'),
  serviceUnavailable(0001, 'Service unavailable!'),

  /* Authentication errors */
  invalidSession(1000, 'Invalid session!'),
  sessionLimitReached(1001, 'Session limit reached!'),
  invalidCredentials(1002, 'Invalid credentials provided!'),
  unauthorized(1004, 'Unauthorized!'),
  noTokenProvided(1006, 'No bearer token provided!'),

  /* Client errors */
  resourceNotFound(1100, 'Requested resource was not found!'),
  serializationError(1101, 'Serialization error!'),
  missingPermissions(1102, 'Missing permissions!'),

  /* Validation errors */
  jsonPayloadValidationError(1200, 'JSON payload validation error!'),
  genericValidationError(1201, 'Validation error!'),

  /* Server errors */
  entityError(1300, "DB-Entity error!"),
  dbError(1301, "Database error!"),
  redisError(1302, "Redis error!"),

  /* Misc errors */
  actixError(9000, "Actix error!"),
  unknown(9999, 'Unknown error!'),
  ;

  final int code;
  final String message;

  const RestrrError(this.code, this.message);

  // TODO: replace this with status code in the future

  static RestrrError? fromCode(int errorCode) {
    for (RestrrError value in RestrrError.values) {
      if (value.code == errorCode) {
        return value;
      }
    }
    return null;
  }

  RestrrException toException() => ServerException(this);
  RestResponse<T> toResponse<T>({required int? statusCode}) => RestResponse<T>(error: toException(), statusCode: statusCode);
}
