import '../../../financrr_api.dart';

class RestrrEventHandler {
  final Map<Type, Function> _eventMap;

  const RestrrEventHandler(this._eventMap);

  void on<T extends RestrrEvent>(Type type, Function(T) callback) {
    _eventMap[type] = callback;
  }

  void fire<T extends RestrrEvent>(T event) {
    _eventMap[event.runtimeType]?.call(event);
  }
}
