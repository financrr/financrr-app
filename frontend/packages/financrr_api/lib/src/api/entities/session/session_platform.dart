import 'dart:io';

enum SessionPlatform {
  web,
  android,
  ios,
  macos,
  windows,
  linux,
  unknown;

  static SessionPlatform current() {
    switch (Platform.operatingSystem) {
      case 'android':
        return SessionPlatform.android;
      case 'ios':
        return SessionPlatform.ios;
      case 'macos':
        return SessionPlatform.macos;
      case 'windows':
        return SessionPlatform.windows;
      case 'linux':
        return SessionPlatform.linux;
      case 'web':
        return SessionPlatform.web;
      default:
        return SessionPlatform.unknown;
    }
  }
}
