import 'package:financrr_api/financrr_api.dart';
import 'package:financrr_api/src/internal/requests/responses/rest_response.dart';

enum ClientError {
  badRequest('Bad request'),

  noNetworkConnection('No network connection'),
  serverUnreachable('Server is unreachable'),
  invalidUri('Invalid URI'),
  ;

  final String message;

  const ClientError(this.message);

  RestrrException toException() => ClientException(this);
  RestResponse<T> toResponse<T>({required int? statusCode}) => RestResponse<T>(error: toException(), statusCode: statusCode);
}
