import 'package:flutter/material.dart';
import 'package:restrr/restrr.dart';

class PaginatedDataResult<T> {
  final int total;
  final List<T> items;
  final Function(Restrr)? nextPage;
  final Function(Restrr)? previousPage;

  const PaginatedDataResult({required this.total, required this.items, this.nextPage, this.previousPage});
}

class PaginatedWrapper<T> extends StatefulWidget {
  final Future<Paginated<T>> Function(bool) initialPageFunction;
  final Widget Function(BuildContext, AsyncSnapshot<PaginatedDataResult<T>>) onSuccess;
  final Widget Function(BuildContext, AsyncSnapshot<PaginatedDataResult<T>>)? onLoading;
  final Widget Function(BuildContext, AsyncSnapshot<PaginatedDataResult<T>>)? onError;

  const PaginatedWrapper({super.key, required this.initialPageFunction, required this.onSuccess, this.onLoading, this.onError});

  @override
  State<PaginatedWrapper<T>> createState() => PaginatedWrapperState<T>();
}

class PaginatedWrapperState<T> extends State<PaginatedWrapper<T>> {
  final Map<int, Paginated<T>> _pages = {};

  int? _currentPage;
  RestrrException? _error;

  @override
  void initState() {
    super.initState();
    _loadInitialPage();
  }

  Future<void> _loadInitialPage({bool forceRetrieve = false}) async {
    try {
      final Paginated<T> page = await widget.initialPageFunction.call(forceRetrieve);
      _pages[page.pageNumber] = page;
      setState(() => _currentPage = page.pageNumber);
    } on RestrrException catch (e) {
      setState(() => _error = e);
    }
  }

  @override
  Widget build(BuildContext context) {
    if (widget.onError != null && _error != null) {
      return widget.onError!.call(context, AsyncSnapshot.withError(ConnectionState.done, _error!));
    }
    return _currentPage == null
        ? (widget.onLoading ?? (_, __) => const Center(child: CircularProgressIndicator()))
            .call(context, const AsyncSnapshot.waiting())
        : widget.onSuccess.call(
            context,
            AsyncSnapshot<PaginatedDataResult<T>>.withData(
                ConnectionState.done,
                PaginatedDataResult<T>(
                    total: _pages[_currentPage]?.total ?? 0,
                    items: _pages.entries.map((e) => e.value.items).expand((e) => e).toList(),
                    nextPage: hasNext ? nextPage : null,
                    previousPage: hasPrevious ? previousPage : null)));
  }

  bool get hasNext => _pages[_currentPage]?.hasNext ?? false;

  bool get hasPrevious => _pages[_currentPage]?.hasPrevious ?? false;

  Future<void> reset() {
    _pages.clear();
    return _loadInitialPage(forceRetrieve: true);
  }

  Future<void>? nextPage(Restrr api) async {
    final Paginated<T>? currentPage = _pages[_currentPage];
    if (currentPage == null || !currentPage.hasNext) {
      return;
    }
    if (!_pages.containsKey(currentPage.pageNumber + 1)) {
      final Paginated<T> nextPage = await currentPage.nextPage!(api);
      _pages[nextPage.pageNumber] = nextPage;
    }
    setState(() => _currentPage = currentPage.pageNumber + 1);
  }

  Future<void>? previousPage(Restrr api) async {
    final Paginated<T>? currentPage = _pages[_currentPage];
    if (currentPage == null || !currentPage.hasPrevious) {
      return;
    }
    if (!_pages.containsKey(currentPage.pageNumber - 1)) {
      final Paginated<T> previousPage = await currentPage.previousPage!(api);
      _pages[previousPage.pageNumber] = previousPage;
    }
    setState(() => _currentPage = currentPage.pageNumber - 1);
  }
}
