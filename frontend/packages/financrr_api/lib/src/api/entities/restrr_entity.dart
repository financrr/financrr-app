import '../../../financrr_api.dart';

typedef Id = int;

abstract class EntityId<E> {
  Restrr get api;
  Id get value;

  E? get();
  Future<E> retrieve({forceRetrieve = false});

  @override
  String toString() => value.toString();

  @override
  bool operator ==(Object other) {
    if (other is EntityId<E>) {
      return other.value == value;
    }
    return false;
  }

  @override
  int get hashCode => value.hashCode;
}

/// The base class for all Restrr entities.
/// This simply provides a reference to the Restrr instance.
abstract class RestrrEntity<E, ID extends EntityId<E>> {
  /// A reference to the Restrr instance.
  Restrr get api;

  ID get id;
}
