import 'package:dio/dio.dart';

import '../../../financrr_api.dart';
import '../../internal/utils/string_utils.dart';

class Route {
  /// The HTTP method of this route.
  final String method;

  /// The path of this route.
  final String path;

  /// The amount of parameters this route expects.
  final int paramCount;

  /// Whether this route is versioned (i.e. /api/v1/...), or not.
  final bool isVersioned;

  Route._(this.method, this.path, {this.isVersioned = true})
      : assert(StringUtils.count(path, '{') == StringUtils.count(path, '}')),
        paramCount = StringUtils.count(path, '{');

  Route.get(String path, {bool isVersioned = true}) : this._('GET', path, isVersioned: isVersioned);

  Route.post(String path, {bool isVersioned = true}) : this._('POST', path, isVersioned: isVersioned);

  Route.put(String path, {bool isVersioned = true}) : this._('PUT', path, isVersioned: isVersioned);

  Route.delete(String path, {bool isVersioned = true}) : this._('DELETE', path, isVersioned: isVersioned);

  Route.patch(String path, {bool isVersioned = true}) : this._('PATCH', path, isVersioned: isVersioned);

  CompiledRoute compile({List<dynamic> params = const []}) {
    if (params.length != paramCount) {
      throw ArgumentError(
          'Error compiling route [$method $path}]: Incorrect amount of parameters! Expected: $paramCount, Provided: ${params.length}');
    }
    final Map<String, String> values = {};
    String compiledRoute = path;
    for (dynamic param in params) {
      int paramStart = compiledRoute.indexOf('{');
      int paramEnd = compiledRoute.indexOf('}');
      values[compiledRoute.substring(paramStart + 1, paramEnd)] = param.toString();
      compiledRoute = compiledRoute.replaceRange(paramStart, paramEnd + 1, param.toString());
    }
    return CompiledRoute(this, compiledRoute, values);
  }
}

class CompiledRoute {
  final Route baseRoute;
  final String compiledRoute;
  final Map<String, String> parameters;
  Map<String, String>? queryParameters;

  CompiledRoute(this.baseRoute, this.compiledRoute, this.parameters, {this.queryParameters});

  CompiledRoute withQueryParams(Map<String, String> params) {
    String newRoute = compiledRoute;
    params.forEach((key, value) {
      newRoute = '$newRoute${queryParameters == null || queryParameters!.isEmpty ? '?' : '&'}$key=$value';
      queryParameters ??= {};
      queryParameters![key] = value;
    });
    return CompiledRoute(baseRoute, newRoute, parameters, queryParameters: queryParameters);
  }

  Future<Response> submit(
      {required RouteOptions routeOptions,
      dynamic body,
      bool isWeb = false,
      String? bearerToken,
      String contentType = 'application/json'}) {
    if (baseRoute.isVersioned && routeOptions.apiVersion == -1) {
      throw StateError('Cannot submit a versioned route without specifying the API version!');
    }
    final Dio dio = Dio();
    Map<String, dynamic> headers = {'Content-Type': contentType};
    if (bearerToken != null) {
      headers['Authorization'] = 'Bearer $bearerToken';
    }
    return dio
        .fetch(RequestOptions(
            path: compiledRoute,
            headers: headers,
            data: body,
            method: baseRoute.method,
            baseUrl: _buildBaseUrl(routeOptions, baseRoute.isVersioned)))
        .then((response) {
      Restrr.log.info('${baseRoute.method} $compiledRoute => ${response.statusCode} ${response.statusMessage}');
      return response;
    });
  }

  String _buildBaseUrl(RouteOptions options, bool isVersioned) {
    String effectiveHostUrl = options.hostUri.toString();
    if (effectiveHostUrl.endsWith('/')) {
      effectiveHostUrl = effectiveHostUrl.substring(0, effectiveHostUrl.length - 1);
    }
    return isVersioned ? '$effectiveHostUrl/api/v${options.apiVersion}' : '$effectiveHostUrl/api';
  }
}
