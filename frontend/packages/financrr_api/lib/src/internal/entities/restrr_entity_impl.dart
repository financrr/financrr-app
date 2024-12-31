import '../../../financrr_api.dart';
import '../restrr_impl.dart';

abstract class IdImpl<E> implements EntityId<E> {
  @override
  final RestrrImpl api;
  @override
  final Id value;

  const IdImpl({required this.api, required this.value});
}

class RestrrEntityImpl<E, ID extends EntityId<E>> implements RestrrEntity<E, ID> {
  @override
  final RestrrImpl api;
  @override
  final ID id;

  const RestrrEntityImpl({
    required this.api,
    required this.id,
  });
}
