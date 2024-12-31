import '../../../financrr_api.dart';

/// Gets fired when the user requested to delete their current session (logging out)
class SessionDeleteEvent extends RestrrEvent {
  const SessionDeleteEvent({required super.api});
}
