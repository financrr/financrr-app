import '../../../../../financrr_api.dart';

class ScheduleRule {
  static const ScheduleRule yearly = ScheduleRule(special: '@yearly');
  static const ScheduleRule annually = ScheduleRule(special: '@annually');
  static const ScheduleRule monthly = ScheduleRule(special: '@monthly');
  static const ScheduleRule weekly = ScheduleRule(special: '@weekly');
  static const ScheduleRule daily = ScheduleRule(special: '@daily');

  final CronPattern? cronPattern;
  final String? special;

  const ScheduleRule({this.cronPattern, this.special});

  static ScheduleRule fromJson(Map<String, dynamic>? json) {
    if (json == null) throw ArgumentError.notNull('json');
    return ScheduleRule(
      cronPattern: CronPattern.fromJson(json['cron_pattern']),
      special: json['special'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      if (cronPattern != null) 'cron_pattern': cronPattern!.toJson(),
      if (special != null) 'special': special,
    };
  }

  ScheduleRule copyWith({
    CronPattern? cronPattern,
    String? special,
  }) {
    return ScheduleRule(
      cronPattern: cronPattern ?? this.cronPattern,
      special: special ?? this.special,
    );
  }
}
