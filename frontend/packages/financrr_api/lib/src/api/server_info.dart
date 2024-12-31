class ServerInfo {
  final bool healthy;
  final int apiVersion;
  final String? details;

  const ServerInfo({
    required this.healthy,
    required this.apiVersion,
    required this.details,
  });

  factory ServerInfo.fromJson(Map<String, dynamic> json) {
    return ServerInfo(
      healthy: json['healthy'] as bool,
      apiVersion: json['api_version'] as int,
      details: json['details'] as String?,
    );
  }
}
