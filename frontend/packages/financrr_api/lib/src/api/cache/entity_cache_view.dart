import 'package:financrr_api/financrr_api.dart';

abstract interface class EntityCacheView<E extends RestrrEntity<E, ID>, ID extends EntityId<E>> {
  E? get(ID id);
  List<E> getAll();
  E add(E entity);
  E? remove(ID id);
  void removeWhere(bool Function(E) predicate);
  bool contains(ID id);
  void clear();
}
