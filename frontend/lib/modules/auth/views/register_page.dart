import 'dart:async';

import 'package:financrr_frontend/shared/ui/auth_page_template.dart';
import 'package:financrr_frontend/modules/auth/providers/authentication.provider.dart';
import 'package:financrr_frontend/modules/auth/models/authentication.state.dart';
import 'package:financrr_frontend/modules/dashboard/views/dashboard_page.dart';
import 'package:financrr_frontend/routing/router_extensions.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_button.dart';
import 'package:financrr_frontend/shared/ui/custom_replacements/custom_text_field.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../../shared/ui/adaptive_scaffold.dart';
import '../../../routing/page_path.dart';
import '../../../shared/ui/custom_replacements/custom_text_button.dart';
import '../../../utils/l10n_utils.dart';
import 'login_page.dart';

class RegisterPage extends StatefulHookConsumerWidget {
  static const PagePathBuilder pagePath = PagePathBuilder('/register');

  final Uri hostUri;

  const RegisterPage({super.key, required this.hostUri});

  @override
  ConsumerState<RegisterPage> createState() => RegisterPageState();
}

class RegisterPageState extends ConsumerState<RegisterPage> {
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final TextEditingController _passwordRepeatController = TextEditingController();

  bool _obscureText = true;
  bool _obscureRepeatText = true;
  bool _isLoading = false;

  @override
  Widget build(BuildContext context) {
    buildVerticalLayout(Size size) {
      return AuthPageTemplate(
          showBackButton: true,
          registerWelcomeMessages: true,
          child: Column(
            children: [
              Padding(
                padding: const EdgeInsets.only(top: 5),
                child: Text(widget.hostUri.host),
              ),
              Form(
                  autovalidateMode: AutovalidateMode.onUserInteraction,
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Padding(
                        padding: const EdgeInsets.symmetric(vertical: 20),
                        child: FinancrrTextField(
                          controller: _usernameController,
                          label: L10nKey.commonUsername,
                          hint: L10nKey.commonSampleName,
                          prefixIcon: const Icon(Icons.person_outline),
                          autofillHints: const [AutofillHints.newUsername],
                          validator: (value) => value!.isEmpty ? L10nKey.commonUsernameRequired.toString() : null,
                        ),
                      ),
                      Padding(
                        padding: const EdgeInsets.only(bottom: 20),
                        child: FinancrrTextField(
                          controller: _passwordController,
                          label: L10nKey.commonPassword,
                          hint: L10nKey.commonPassword,
                          prefixIcon: const Icon(Icons.lock_outline),
                          suffixIcon: Padding(
                            padding: const EdgeInsets.only(right: 10),
                            child: IconButton(
                                icon: Icon(_obscureText ? Icons.visibility : Icons.visibility_off),
                                onPressed: () => setState(() => _obscureText = !_obscureText)),
                          ),
                          obscureText: _obscureText,
                          autofillHints: const [AutofillHints.newPassword],
                          validator: (value) => value!.isEmpty ? L10nKey.commonPasswordRequired.toString() : null,
                        ),
                      ),
                      FinancrrTextField(
                        controller: _passwordRepeatController,
                        label: L10nKey.commonPasswordRepeat,
                        hint: L10nKey.commonPasswordRepeat,
                        prefixIcon: const Icon(Icons.lock_outline),
                        suffixIcon: Padding(
                          padding: const EdgeInsets.only(right: 10),
                          child: IconButton(
                              icon: Icon(_obscureRepeatText ? Icons.visibility : Icons.visibility_off),
                              onPressed: () => setState(() => _obscureRepeatText = !_obscureRepeatText)),
                        ),
                        obscureText: _obscureRepeatText,
                        autofillHints: const [AutofillHints.newPassword],
                        validator: (value) => value!.isEmpty ? L10nKey.commonPasswordRepeatRequired.toString() : null,
                      ),
                    ],
                  )),
              Padding(
                  padding: const EdgeInsets.only(top: 20),
                  child: FinancrrButton(
                    onPressed: _handleRegistration,
                    isLoading: _isLoading,
                    text: L10nKey.commonRegister.toString(),
                  )),
              Padding(
                  padding: const EdgeInsets.only(top: 10),
                  child: FinancrrTextButton(
                    onPressed: () {
                      if (_isLoading) {
                        return;
                      }
                      context.goPath(LoginPage.pagePath.build(), extra: widget.hostUri);
                    },
                    label: L10nKey.registerExistingAccount.toText(),
                  )),
            ],
          ));
    }

    return AdaptiveScaffold(
      resizeToAvoidBottomInset: false,
      verticalBuilder: (_, __, size) => SafeArea(child: buildVerticalLayout(size)),
    );
  }

  Future<void> _handleRegistration() async {
    if (_isLoading) {
      return;
    }
    // check if username is empty
    final String username = _usernameController.text;
    if (username.isEmpty) {
      L10nKey.commonUsernameRequired.showSnack(context);
      return;
    }
    // validate password
    final String password = _passwordController.text;
    final L10nKey? passwordError = _validatePassword(password, _passwordRepeatController.text);
    if (passwordError != null) {
      passwordError.showSnack(context);
      return;
    }

    setState(() => _isLoading = true);
    final AuthenticationState state = await ref.read(authProvider.notifier).register(username, password, widget.hostUri);
    setState(() => _isLoading = false);
    if (!mounted) return;
    if (state.status == AuthenticationStatus.authenticated) {
      context.goPath(DashboardPage.pagePath.build());
    } else {
      L10nKey.registerFailed.showSnack(context);
    }
  }

  L10nKey? _validatePassword(String password, String repeated) {
    // check if password is empty
    if (password.isEmpty) {
      return L10nKey.commonPasswordRequired;
    }
    // TODO: implement password strength check
    if (password.length < 16) {
      return L10nKey.commonPasswordWeak;
    }
    // check if repeat password is empty
    if (repeated.isEmpty) {
      return L10nKey.commonPasswordRequired;
    }
    // check if passwords match
    if (password != repeated) {
      return L10nKey.commonPasswordNoMatch;
    }
    return null;
  }
}
