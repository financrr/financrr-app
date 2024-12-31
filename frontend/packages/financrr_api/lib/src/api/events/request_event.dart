import 'package:financrr_api/financrr_api.dart';

class RequestEvent extends RestrrEvent {
  final String route;
  final int? statusCode;

  const RequestEvent({required super.api, required this.route, this.statusCode});
}
