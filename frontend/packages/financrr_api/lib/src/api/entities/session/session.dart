import 'package:financrr_api/src/api/entities/session/partial_session.dart';

import '../../../../financrr_api.dart';

abstract class Session extends PartialSession {
  String get token;
}
