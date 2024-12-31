import 'package:financrr_api/financrr_api.dart';

class EntityCacheViewImpl<E extends RestrrEntity<E, ID>, ID extends EntityId<E>> implements EntityCacheView<E, ID> {
  final Map<Id, E> _cache = {};

  @override
  E? get(ID id) => _cache[id.value];

  @override
  List<E> getAll() => _cache.values.toList();

  @override
  E add(E entity) => _cache[entity.id.value] = entity;

  @override
  E? remove(ID id) => _cache.remove(id.value);

  @override
  void removeWhere(bool Function(E p1) predicate) => _cache.removeWhere((k, v) => predicate(v));

  @override
  bool contains(ID id) => _cache.containsKey(id.value);

  @override
  void clear() => _cache.clear();
}
