import 'package:financrr_api/financrr_api.dart';

import '../../internal/requests/client_errors.dart';

class ClientException extends RestrrException {
  final ClientError error;

  ClientException(this.error) : super(error.message);

  @override
  String toString() => '${runtimeType.toString()}: $message';
}
