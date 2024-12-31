import '../../../financrr_api.dart';
import '../../internal/requests/restrr_errors.dart';

class ServerException extends RestrrException {
  final RestrrError error;

  ServerException(this.error) : super(error.message);

  @override
  String toString() => '${runtimeType.toString()}: $message (${error.code})';
}
