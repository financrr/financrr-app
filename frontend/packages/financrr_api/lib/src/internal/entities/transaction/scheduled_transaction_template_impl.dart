import 'package:financrr_api/src/internal/entities/restrr_entity_impl.dart';
import 'package:financrr_api/src/internal/requests/responses/rest_response.dart';

import '../../../../financrr_api.dart';
import '../../utils/request_utils.dart';

class ScheduledTransactionTemplateIdImpl extends IdImpl<ScheduledTransactionTemplate>
    implements ScheduledTransactionTemplateId {
  const ScheduledTransactionTemplateIdImpl({required super.api, required super.value});

  @override
  ScheduledTransactionTemplate? get() => api.scheduledTransactionTemplateCache.get(this);

  @override
  Future<ScheduledTransactionTemplate> retrieve({forceRetrieve = false}) => RequestUtils.getOrRetrieveSingle(
      api: api,
      key: this,
      cacheView: api.scheduledTransactionTemplateCache,
      compiledRoute: ScheduledTransactionTemplateRoutes.getById.compile(params: [value]),
      mapper: (json) => api.entityBuilder.buildScheduledTransactionTemplate(json),
      forceRetrieve: forceRetrieve);
}

class ScheduledTransactionTemplateImpl extends RestrrEntityImpl<ScheduledTransactionTemplate, ScheduledTransactionTemplateId>
    implements ScheduledTransactionTemplate {
  @override
  final TransactionTemplateId templateId;
  @override
  final DateTime? lastExecutedAt;
  @override
  final DateTime? nextExecutedAt;
  @override
  final ScheduleRule scheduleRule;
  @override
  final DateTime createdAt;

  const ScheduledTransactionTemplateImpl({
    required super.api,
    required super.id,
    required this.templateId,
    required this.lastExecutedAt,
    required this.nextExecutedAt,
    required this.scheduleRule,
    required this.createdAt,
  });

  @override
  Future<bool> delete() => RequestUtils.deleteSingle(
      compiledRoute: ScheduledTransactionTemplateRoutes.deleteById.compile(params: [id.value]),
      api: api,
      key: id,
      cacheView: api.scheduledTransactionTemplateCache);

  @override
  Future<ScheduledTransactionTemplate> update({
    Id? templateId,
    DateTime? lastExecutedAt,
    ScheduleRule? scheduleRule,
  }) async {
    if (templateId == null && lastExecutedAt == null && scheduleRule == null) {
      throw ArgumentError('At least one field must be set');
    }
    final RestResponse<ScheduledTransactionTemplate> response = await api.requestHandler.apiRequest(
      route: ScheduledTransactionTemplateRoutes.patchById.compile(params: [id.value]),
      mapper: (json) => api.entityBuilder.buildScheduledTransactionTemplate(json),
      body: {
        if (templateId != null) 'template_id': templateId,
        if (lastExecutedAt != null) 'last_executed_at': lastExecutedAt.toIso8601String(),
        if (scheduleRule != null) 'recurring_rule': scheduleRule.toJson(), // TODO: rename once backend is updated
      },
    );
    return response.data!;
  }
}
