import 'dart:async';

import 'package:financrr_frontend/shared/ui/auth_page_template.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_text_button.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_text_field.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:financrr_frontend/utils/l10n_utils.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:financrr_api/financrr_api.dart';

import '../../../shared/models/store.dart';
import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../routing/page_path.dart';
import '../../../utils/input_utils.dart';
import 'login_page.dart';

class ServerConfigPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder('/server-config');

  final String? redirectTo;

  const ServerConfigPage({super.key, this.redirectTo});

  @override
  ConsumerState<ServerConfigPage> createState() => ServerConfigPageState();
}

class ServerConfigPageState extends ConsumerState<ServerConfigPage> {
  final TextEditingController _urlController = TextEditingController();

  bool _isLoading = false;
  bool _isValid = false;
  Uri? _hostUri;
  int? _apiVersion;

  @override
  void initState() {
    super.initState();
    final String? hostUrl = StoreKey.hostUrl.readSync();
    if (hostUrl != null && InputValidators.url(hostUrl) == null) {
      _urlController.text = hostUrl;
      _handleUrlCheck();
    }
  }

  @override
  Widget build(BuildContext context) {
    buildVerticalLayout(Size size) {
      return AuthPageTemplate(
          child: Column(
        children: [
          Form(
              autovalidateMode: AutovalidateMode.onUserInteraction,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Padding(
                    padding: const EdgeInsets.only(top: 20),
                    child: FinancrrTextField(
                      controller: _urlController,
                      label: L10nKey.serverConfigUrl,
                      hint: L10nKey.brandDemoUrl,
                      status: _isValid && _apiVersion != null
                          ? L10nKey.serverConfigStatus
                              .toString(namedArgs: {'hostStatus': 'Healthy', 'apiVersion': '$_apiVersion'})
                          : null,
                      prefixIcon: const Icon(Icons.link),
                      autofillHints: const [AutofillHints.username],
                      validator: (value) => InputValidators.url(value),
                      onChanged: (_) => setState(() {
                        _isValid = false;
                        _apiVersion = null;
                      }),
                    ),
                  ),
                ],
              )),
          Padding(
              padding: const EdgeInsets.only(top: 20, bottom: 20),
              child: _isLoading
                  ? const CircularProgressIndicator()
                  : FinancrrTextButton(
                      onPressed: () {
                        if (_isValid) {
                          context.goPath(LoginPage.pagePath.build(), extra: _hostUri);
                        } else {
                          _handleUrlCheck();
                        }
                      },
                      label: const Icon(Icons.arrow_forward, size: 18),
                      icon: (_isValid ? L10nKey.commonNext : L10nKey.serverConfigCheckUrl).toText(),
                    )),
        ],
      ));
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => SafeArea(child: buildVerticalLayout(size)),
    );
  }

  Future<void> _handleUrlCheck() async {
    final String url = _urlController.text;
    if (InputValidators.url(url) != null) {
      L10nKey.commonUrlInvalid.showSnack(context);
      return;
    }
    setState(() {
      _isLoading = true;
      _isValid = false;
    });
    late final ServerInfo info;
    try {
      info = await Restrr.checkUri(Uri.parse(url), isWeb: kIsWeb);
    } on RestrrException catch (e) {
      setState(() => _isLoading = false);
      if (mounted) {
        context.showSnackBar(e.message ?? 'err');
      }
      return;
    }
    setState(() {
      _isLoading = false;
      _isValid = true;
      _apiVersion = info.apiVersion;
    });
    _hostUri = Uri.parse(url);
    StoreKey.hostUrl.write(url);
  }
}
