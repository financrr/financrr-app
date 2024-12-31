import 'package:financrr_api/src/internal/requests/responses/rest_response.dart';

import '../../../../financrr_api.dart';

class PaginatedResponseMetadataLinks {
  final String? prev;
  final String? next;

  const PaginatedResponseMetadataLinks({
    required this.prev,
    required this.next,
  });

  factory PaginatedResponseMetadataLinks.fromJson(Map<String, dynamic> json) {
    return PaginatedResponseMetadataLinks(
      prev: json['prev'],
      next: json['next'],
    );
  }
}

class PaginatedResponseMetadata {
  final int page;
  final int limit;
  final int total;
  final PaginatedResponseMetadataLinks links;

  const PaginatedResponseMetadata({
    required this.page,
    required this.limit,
    required this.total,
    required this.links,
  });

  factory PaginatedResponseMetadata.fromJson(Map<String, dynamic> json) {
    return PaginatedResponseMetadata(
      page: json['page'],
      limit: json['limit'],
      total: json['total'],
      links: PaginatedResponseMetadataLinks.fromJson(json['links']),
    );
  }
}

class PaginatedResponse<T> extends RestResponse<List<T>> {
  final PaginatedResponseMetadata metadata;
  final Route baseRoute;
  final T Function(dynamic) mapper;

  const PaginatedResponse({
    super.data,
    super.error,
    required super.statusCode,
    required this.metadata,
    required this.baseRoute,
    required this.mapper,
  });

  Paginated<T> toPage() {
    if (hasError) {
      throw error!;
    }
    return Paginated<T>(
      pageNumber: metadata.page,
      limit: metadata.limit,
      total: metadata.total,
      items: data!,
      nextPage: metadata.links.next != null ? (api) => _fetchPage(api, metadata.links.next!) : null,
      previousPage: metadata.links.prev != null ? (api) => _fetchPage(api, metadata.links.prev!) : null,
    );
  }

  Future<Paginated<T>> _fetchPage(Restrr api, String url) {
    final Uri uri = Uri.parse(url);
    return RequestHandler(api)
        .paginatedApiRequest(
            route: baseRoute.compile(),
            page: int.parse(uri.queryParameters['page']!),
            limit: int.parse(uri.queryParameters['limit']!),
            mapper: mapper)
        .then((response) {
      if (response.hasError) {
        throw error!;
      }
      return (response as PaginatedResponse<T>).toPage();
    });
  }
}
