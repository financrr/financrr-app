import 'package:easy_localization/easy_localization.dart';
import 'package:financrr_frontend/modules/settings/providers/theme.provider.dart';
import 'package:financrr_frontend/utils/extensions.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:styled_text/styled_text.dart';

enum L10nKey {
  // account
  accountCreate('account_create'),
  accountDelete('account_delete'),
  accountEdit('account_edit'),
  accountListManage('account_list_manage'),
  accountNoneFoundBody('account_none_found_body'),
  accountNoneFoundTitle('account_none_found_title'),
  accountNotFound('account_not_found'),
  accountPropertiesCurrency('account_properties_currency'),
  accountPropertiesDescription('account_properties_description'),
  accountPropertiesIban('account_properties_iban'),
  accountPropertiesName('account_properties_name'),
  accountPropertiesOriginalBalance('account_properties_original_balance'),
  // appearance
  appearanceCurrentDarkTheme('appearance_current_dark_theme'),
  appearanceCurrentDeviceTheme('appearance_current_device_theme'),
  appearanceCurrentLightTheme('appearance_current_light_theme'),
  appearanceUseDeviceTheme('appearance_use_device_theme'),
  // brand
  brandDemoUrl('brand_demo_url'),
  brandName('brand_name'),
  // common
  commonClipboardCopy('common_clipboard_copy'),
  commonClipboardCopyObject('common_clipboard_copy_object', hasParams: true),
  commonCreate('common_create'),
  commonCreateObject('common_create_object', hasParams: true),
  commonCreateObjectSuccess('common_create_object_success', hasParams: true),
  commonDelete('common_delete'),
  commonDeleteAll('common_delete_all'),
  commonDeleteObjectSuccess('common_delete_object_success', hasParams: true),
  commonEdit('common_edit'),
  commonEditObject('common_edit_object', hasParams: true),
  commonEditObjectSuccess('common_edit_object_success', hasParams: true),
  commonIbanInvalid('common_iban_invalid'),
  commonLoadMore('common_load_more'),
  commonLogin('common_login'),
  commonLogout('common_logout'),
  commonNext('common_next'),
  commonPassword('common_password'),
  commonPasswordNoMatch('common_password_no_match'),
  commonPasswordRepeat('common_password_repeat'),
  commonPasswordRepeatRequired('common_password_repeat_required'),
  commonPasswordRequired('common_password_required'),
  commonPasswordWeak('common_password_weak'),
  commonPreview('common_preview'),
  commonRegister('common_register'),
  commonRequiredObject('common_required_object', hasParams: true),
  commonSampleName('common_sample_name'),
  commonSave('common_save'),
  commonSaveSuccess('common_save_success'),
  commonSortNewestFirst('common_sort_newest_first'),
  commonSortOldestFirst('common_sort_oldest_first'),
  commonUrlInvalid('common_url_invalid'),
  commonUsername('common_username'),
  commonUsernameRequired('common_username_required'),
  commonVersion('common_version', hasParams: true),
  // currency
  currencyCreate('currency_create'),
  currencyEdit('currency_edit'),
  currencyNotEditable('currency_not_editable'),
  currencyNotFound('currency_not_found'),
  currencyPropertiesDecimalPlaces('currency_properties_decimal_places'),
  currencyPropertiesIsoCode('currency_properties_iso_code'),
  currencyPropertiesName('currency_properties_name'),
  currencyPropertiesSymbol('currency_properties_symbol'),
  // dashboard
  dashboardAccounts('dashboard_accounts'),
  dashboardQuickActions('dashboard_quick_actions'),
  dashboardTotal('dashboard_total'),
  dashboardTransactions('dashboard_transactions'),
  // l10n
  l10nDateFormat('l10n_date_format'),
  l10nDecimalSeparator('l10n_decimal_separator'),
  l10nThousandsSeparator('l10n_thousands_separator'),
  // login
  loginFailed('login_failed'),
  loginMessage1('login_message_1'),
  loginMessage2('login_message_2'),
  loginMessage3('login_message_3'),
  loginMessage4('login_message_4'),
  loginMessage5('login_message_5'),
  loginNoAccount('login_no_account'),
  // navigation
  navigationAccounts('navigation_accounts'),
  navigationDashboard('navigation_dashboard'),
  navigationSettings('navigation_settings'),
  navigationStatistics('navigation_statistics'),
  // register
  registerExistingAccount('register_existing_account'),
  registerFailed('register_failed'),
  registerMessage1('register_message_1'),
  registerMessage2('register_message_2'),
  registerMessage3('register_message_3'),
  registerMessage4('register_message_4'),
  registerMessage5('register_message_5'),
  // server
  serverConfigCheckUrl('server_config_check_url'),
  serverConfigStatus('server_config_status', hasParams: true),
  serverConfigUrl('server_config_url'),
  // session
  sessionDeleteAllSuccess('session_delete_all_success'),
  // settings
  settingsCategoryAccount('settings_category_account'),
  settingsCategoryApp('settings_category_app'),
  settingsCategoryDeveloper('settings_category_developer'),
  settingsFooter('settings_footer'),
  settingsItemAppearance('settings_item_appearance'),
  settingsItemCurrencies('settings_item_currencies'),
  settingsItemLanguage('settings_item_language'),
  settingsItemLocalStorage('settings_item_local_storage'),
  settingsItemLogs('settings_item_logs'),
  settingsItemSessions('settings_item_sessions'),
  settingsItemTransactionTemplates('settings_item_transaction_templates'),
  // startup
  startupErrorSubtitle('startup_error_subtitle'),
  startupErrorTitle('startup_error_title'),
  // template
  templateNotFound('template_not_found'),
  templatePropertiesAmount('template_properties_amount'),
  templatePropertiesCreatedAt('template_properties_created_at'),
  templatePropertiesDescription('template_properties_description'),
  templatePropertiesFrom('template_properties_from'),
  templatePropertiesName('template_properties_name'),
  templatePropertiesTo('template_properties_to'),
  templateTitleTransfer('template_title_transfer', hasParams: true, hasStyleTags: true),
  templateNoneFoundBody('template_none_found_body'),
  templateNoneFoundTitle('template_none_found_title'),
  // theme
  themeDark('theme_dark'),
  themeLight('theme_light'),
  themeMidnight('theme_midnight'),
  // transaction
  transactionCreate('transaction_create'),
  transactionCreateDeposit('transaction_create_deposit'),
  transactionCreateTemplate('transaction_create_template'),
  transactionCreateTransfer('transaction_create_transfer'),
  transactionCreateTransferTo('transaction_create_transfer_to'),
  transactionCreateWithdrawal('transaction_create_withdrawal'),
  transactionEdit('transaction_edit'),
  transactionNoneFoundBody('transaction_none_found_body'),
  transactionNoneFoundTitle('transaction_none_found_title'),
  transactionNotFound('transaction_not_found'),
  transactionPropertiesAmount('transaction_properties_amount'),
  transactionPropertiesCreatedAt('transaction_properties_created_at'),
  transactionPropertiesDescription('transaction_properties_description'),
  transactionPropertiesExecutedAt('transaction_properties_executed_at'),
  transactionPropertiesFrom('transaction_properties_from'),
  transactionPropertiesName('transaction_properties_name'),
  transactionPropertiesTo('transaction_properties_to'),
  transactionPropertiesType('transaction_properties_type'),
  ;

  static final Logger _log = Logger('L10nKeyLogger');

  final String key;
  final bool hasParams;
  final bool hasStyleTags;
  const L10nKey(this.key, {this.hasParams = false, this.hasStyleTags = false});

  Text toText({Map<String, String>? namedArgs, TextStyle? style, TextAlign? textAlign, bool? softWrap}) {
    if (hasStyleTags) {
      _log.warning('L10nKey $key has style tags, which are not supported by toText()');
    }
    if (hasParams && namedArgs == null) {
      _log.warning('L10nKey $key has params, but namedArgs is null');
    }
    return Text(
      key,
      style: style,
      textAlign: textAlign,
      softWrap: softWrap,
    ).tr(namedArgs: namedArgs);
  }

  StyledText toStyledText(WidgetRef ref,
      {Map<String, String>? namedArgs, TextStyle? style, TextAlign? textAlign, bool? softWrap}) {
    if (hasParams && namedArgs == null) {
      _log.warning('L10nKey $key has params, but namedArgs is null');
    }
    var theme = ref.watch(themeProvider);

    styleIf(
        {TextStyle? baseStyle, Color? color, FontWeight? fontWeight, FontStyle? fontStyle, TextDecoration? decoration, bool invert = false}) {
      return StyledTextCustomTag(
          baseStyle: baseStyle,
          parse: (baseStyle, attributes) {
            final String? key = namedArgs?[attributes['key']];
            final String? value = attributes['value'];
            return (invert ? (key != value) : (key == value))
                ? baseStyle?.copyWith(color: color, fontWeight: fontWeight, fontStyle: fontStyle, decoration: decoration)
                : baseStyle;
          });
    }

    return StyledText(
      text: key.tr(namedArgs: namedArgs),
      style: style,
      textAlign: textAlign,
      softWrap: softWrap,
      tags: {
        'bold': StyledTextTag(style: style?.copyWith(fontWeight: FontWeight.bold)),
        'italic': StyledTextTag(style: style?.copyWith(fontStyle: FontStyle.italic)),
        'underlined': StyledTextTag(style: style?.copyWith(decoration: TextDecoration.underline)),
        'primary': StyledTextTag(style: style?.copyWith(color: theme.financrrExtension.primary)),
        'error': StyledTextTag(style: style?.copyWith(color: theme.themeData.colorScheme.error)),
        // conditional tags
        'boldIf': styleIf(baseStyle: style, fontWeight: FontWeight.bold),
        'italicIf': styleIf(baseStyle: style, fontStyle: FontStyle.italic),
        'underlinedIf': styleIf(baseStyle: style, decoration: TextDecoration.underline),
        'primaryIf': styleIf(baseStyle: style, color: theme.financrrExtension.primary),
        'errorIf': styleIf(baseStyle: style, color: theme.themeData.colorScheme.error),
        'boldIfNot': styleIf(baseStyle: style, fontWeight: FontWeight.bold, invert: true),
        'italicIfNot': styleIf(baseStyle: style, fontStyle: FontStyle.italic, invert: true),
        'underlinedIfNot': styleIf(baseStyle: style, decoration: TextDecoration.underline, invert: true),
        'primaryIfNot': styleIf(baseStyle: style, color: theme.financrrExtension.primary, invert: true),
        'errorIfNot': styleIf(baseStyle: style, color: theme.themeData.colorScheme.error, invert: true),
      },
    );
  }

  void showSnack(BuildContext context, {Map<String, String>? namedArgs}) {
    context.showSnackBar(key.tr(namedArgs: namedArgs));
  }

  @override
  String toString({Map<String, String>? namedArgs}) {
    if (hasStyleTags) {
      _log.warning('L10nKey $key has style tags, which are not supported by toString()');
    }
    if (hasParams && namedArgs == null) {
      _log.warning('L10nKey $key has params, but namedArgs is null');
    }
    return key.tr(namedArgs: namedArgs);
  }

  static L10nKey? fromKey(String key) {
    for (L10nKey value in L10nKey.values) {
      if (value.key == key) {
        return value;
      }
    }
    return null;
  }
}
