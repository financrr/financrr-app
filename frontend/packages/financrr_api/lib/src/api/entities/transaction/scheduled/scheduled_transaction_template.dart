import '../../../../../financrr_api.dart';

abstract class ScheduledTransactionTemplateId extends EntityId<ScheduledTransactionTemplate> {}

abstract class ScheduledTransactionTemplate extends RestrrEntity<ScheduledTransactionTemplate, ScheduledTransactionTemplateId> {
  @override
  ScheduledTransactionTemplateId get id;

  TransactionTemplateId get templateId;
  DateTime? get lastExecutedAt;
  DateTime? get nextExecutedAt;
  ScheduleRule get scheduleRule;
  DateTime get createdAt;

  Future<bool> delete();

  Future<ScheduledTransactionTemplate> update({
    Id? templateId,
    DateTime? lastExecutedAt,
    ScheduleRule? scheduleRule,
  });
}
