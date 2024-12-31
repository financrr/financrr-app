import 'package:financrr_api/src/internal/requests/restrr_errors.dart';

class ErrorResponse {
  final String details;
  final dynamic reference;
  final RestrrError? error;
  final ApiCode apiCode;

  const ErrorResponse({
    required this.details,
    required this.reference,
    required this.error,
    required this.apiCode,
  });

  static ErrorResponse? tryFromJson(Map<String, dynamic>? json) {
    final ApiCode? apiCode = ApiCode.tryFromJson(json?['api_code']);
    if (json == null || json.isEmpty || apiCode == null || json['details'] == null) {
      return null;
    }
    final RestrrError? error = RestrrError.fromCode(apiCode.code);
    return ErrorResponse(
      details: json['details'],
      reference: json['reference'],
      error: error,
      apiCode: apiCode,
    );
  }
}

class ApiCode {
  final int code;
  final String message;

  const ApiCode(this.code, this.message);

  static ApiCode? tryFromJson(Map<String, dynamic>? json) {
    if (json == null || json.isEmpty || json['code'] == null) {
      return null;
    }
    return ApiCode(
      json['code'],
      json['message'],
    );
  }
}
