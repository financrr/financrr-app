import 'dart:convert';

import 'package:financrr_frontend/modules/settings/models/themes/app_theme.model.dart';
import 'package:test/expect.dart';
import 'package:test/scaffolding.dart';

const String validTheme = '''
{
  "id": "DARK",
  "logo_path": "assets/logo/logo_light.svg",
  "translation_key": "theme_dark",
  "preview_color": "0x111111",
  "theme_mode": "dark",
  "brightness": "dark",
  "colors": {
    "primary": "0x4D94FF",
    "error": "0xFF0000",
    "surface": "0x333333",
    "surface_variant1": "0x4E4E4E",
    "surface_variant2": "0x7F7F7F",
    "surface_variant3": "0xE0E0E0",
    "on_primary": "0xFFFFFF",
    "on_error": "0xFFFFFF",
    "on_surface": "0xFFFFFF"
  }
}
''';

void main() {
  group('AppTheme', () {
    group('.tryFromJson', () {
      test(' (valid)', () {
        final AppTheme? theme = AppTheme.tryFromJson(jsonDecode(validTheme));
        expect(theme, isNotNull);
        expect(theme!.id, 'DARK');
      });
    });
  });
}
