abstract class RestrrException implements Exception {
  final String? message;

  RestrrException(this.message);

  @override
  String toString() => '${runtimeType.toString()}: $message';
}
