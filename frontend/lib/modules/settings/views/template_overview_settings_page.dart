import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/shared/ui/cards/transaction_template_card.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_text_button.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:restrr/restrr.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../../routing/page_path.dart';
import '../../../../modules/settings/views/settings_page.dart';
import '../../../shared/ui/paginated_wrapper.dart';

class TemplateOverviewSettingsPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder.child(parent: SettingsPage.pagePath, path: 'templates');

  const TemplateOverviewSettingsPage({super.key});

  @override
  ConsumerState<TemplateOverviewSettingsPage> createState() => _TemplateOverviewSettingsPageState();
}

class _TemplateOverviewSettingsPageState extends ConsumerState<TemplateOverviewSettingsPage> {
  final GlobalKey<PaginatedWrapperState<Currency>> _paginatedTemplateKey = GlobalKey();
  late final Restrr _api = api;

  final ValueNotifier<int> _amount = ValueNotifier(0);

  @override
  Widget build(BuildContext context) {
    buildVerticalLayout(Size size) {
      return Padding(
        padding: const EdgeInsets.only(top: 10, bottom: 20),
        child: Center(
          child: SizedBox(
            width: size.width / 1.1,
            child: RefreshIndicator(
              onRefresh: () async => _paginatedTemplateKey.currentState?.reset(),
              child: ListView(
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      FinancrrTextButton(
                        onPressed: () {},
                        icon: const Icon(Icons.add),
                        label: L10nKey.commonCreate.toText(),
                      ),
                      ValueListenableBuilder(
                        valueListenable: _amount,
                        builder: (context, value, child) {
                          // TODO: add plurals
                          return Text('${_amount.value} template(s)');
                        },
                      ),
                    ],
                  ),
                  const SizedBox(height: 20),
                  PaginatedWrapper(
                    key: _paginatedTemplateKey,
                    initialPageFunction: (forceRetrieve) =>
                        _api.retrieveAllTransactionTemplates(limit: 10, forceRetrieve: forceRetrieve),
                    onSuccess: (context, snap) {
                      final PaginatedDataResult<TransactionTemplate> templates = snap.data!;
                      WidgetsBinding.instance.addPostFrameCallback((_) => _amount.value = templates.total);
                      return Column(
                        children: [
                          for (TransactionTemplate c in templates.items)
                            Padding(
                              padding: const EdgeInsets.only(bottom: 10),
                              child: TransactionTemplateCard(template: c, onDelete: () => _deleteTemplate(c)),
                            ),
                          if (templates.nextPage != null)
                            FinancrrTextButton(
                              onPressed: () => templates.nextPage!(_api),
                              label: L10nKey.commonLoadMore.toText(),
                            ),
                        ],
                      );
                    },
                  )
                ],
              ),
            ),
          ),
        ),
      );
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => buildVerticalLayout(size),
    );
  }

  void _deleteTemplate(TransactionTemplate template) async {
    try {
      await template.delete();
      if (!mounted) return;
      L10nKey.commonDeleteObjectSuccess.showSnack(context, namedArgs: {'object': template.name});
    } on RestrrException catch (e) {
      context.showSnackBar(e.message!);
    }
  }
}
