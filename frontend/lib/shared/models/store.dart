import 'dart:async';

import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:shared_preferences/shared_preferences.dart';

enum StoreKey<T> {
  hostUrl<String>('host_url', type: String),
  dateTimeFormat<DateFormat>('date_time_format',
      type: DateFormat, defaultFactory: _defaultDateFormat, fromValue: _dateFormatFromValue, toValue: _dateFormatToValue),
  decimalSeparator<String>('decimal_separator', type: String, defaultValue: '.'),
  thousandSeparator<String>('thousand_separator', type: String, defaultValue: ','),
  sessionToken<String>('session_token', type: String, secure: true),
  themeMode<ThemeMode>('theme_mode',
      type: ThemeMode, defaultValue: ThemeMode.system, fromValue: _themeModeFromValue, toValue: _enumToValue),
  currentLightThemeId<String>('current_light_theme_id', type: String, defaultValue: 'LIGHT'),
  currentDarkThemeId<String>('current_dark_theme_id', type: String, defaultValue: 'DARK'),
  ;

  final String key;
  final Type type;
  final T? defaultValue;
  final T Function()? defaultFactory;
  final bool secure;
  final T? Function(String?)? fromValue;
  final String? Function(T?)? toValue;

  const StoreKey(this.key,
      {required this.type, this.defaultValue, this.defaultFactory, this.secure = false, this.fromValue, this.toValue})
      : assert(!secure || (type == String), 'Secure keys must be of type String!'),
        assert(defaultValue == null || defaultFactory == null);

  T? readSync() => KeyValueStore._instance.readSync(this);
  String? readAsStringSync() => KeyValueStore._instance.readAsStringSync(this);
  Future<T?> readAsync() => KeyValueStore._instance.readAsync(this);
  Future<String?> readAsStringAsync() => KeyValueStore._instance.readAsStringAsync(this);
  void write(T value) => KeyValueStore._instance.write(this, value);
  Future<void> delete() => KeyValueStore._instance.delete(this);

  static DateFormat _defaultDateFormat() => DateFormat('dd/MM/yyyy HH:mm');
  static DateFormat? _dateFormatFromValue(String? value) => value == null ? null : DateFormat(value);
  static String? _dateFormatToValue(dynamic format) => format is DateFormat? ? format?.pattern : null;

  // Enums
  static T? _enumFromValue<T extends Enum>(String? value, List<T> values) =>
      value == null ? null : values.firstWhere((element) => element.name == value);
  static String? _enumToValue<T extends Enum>(dynamic value) => value is T? ? value?.name : null;
  static ThemeMode? _themeModeFromValue(String? value) => _enumFromValue(value, ThemeMode.values);
}

/// A simple key-value store that uses SharedPreferences and FlutterSecureStorage to store data.
/// This class is a singleton and should be initialized once before using it. (See [init])
/// The store uses a local cache to allow synchronous reads, making it easier to use
/// in the UI layer.
class KeyValueStore {
  static final KeyValueStore _instance = KeyValueStore._();
  static final Map<String, dynamic> _localCache = {};

  late final SharedPreferences _preferences;
  late final FlutterSecureStorage _storage;
  bool _initialized = false;

  factory KeyValueStore() => _instance;

  KeyValueStore._();

  /// Initializes the KeyValueStore by loading the SharedPreferences and FlutterSecureStorage.
  /// This will also initialize all default values for the keys & populate the local cache.
  /// This method should only be called once - Otherwise, it will throw a StateError.
  static Future<void> init() async {
    if (_instance._initialized) {
      throw StateError('Store has already been initialized!');
    }
    _instance._preferences = await SharedPreferences.getInstance();
    _instance._storage = const FlutterSecureStorage();
    _instance._initialized = true;
    for (StoreKey key in StoreKey.values) {
      // initializes all default values for keys that do not exist &
      // populate local cache
      _localCache[key.key] = await key.readAsync();
    }
  }

  /// Reads the value of the key from the local cache.
  /// If the key does not exist in the cache, it will return null.
  T? readSync<T>(StoreKey<T> key) {
    _checkInitialized();
    return _localCache[key.key];
  }

  /// Reads the value of the key from the local cache and converts it to a string,
  /// by either using the [toValue] function of the key or by simply returning
  /// its toString() representation.
  String? readAsStringSync<T>(StoreKey<T> key) {
    return _toStringOrValue<T?>(readSync(key), key).toString();
  }

  /// Reads the value of the key from the local cache.
  /// If the key does not exist in the cache, it will be read from the storage.
  Future<T?> readAsync<T>(StoreKey<T> key) async {
    _checkInitialized();
    if (key.secure) {
      return await _readOrInitSecureStringKey(key);
    }
    return await _readOrInitSharedPrefsKey(key);
  }

  /// Reads the value of the key from the local cache and converts it to a string,
  /// by either using the [toValue] function of the key or by simply returning
  /// its toString() representation.
  Future<String?> readAsStringAsync<T>(StoreKey<T> key) async {
    return _toStringOrValue<T?>(await readAsync(key), key);
  }

  /// Writes the value to the local cache and the storage.
  Future<void> write<T>(StoreKey<T> key, T value, {bool storeLocal = true}) async {
    _checkInitialized();
    // simplify enums to strings
    if (storeLocal) {
      _localCache[key.key] = value;
    }
    if (value is Enum) {
      return await write(key, value.name, storeLocal: false);
    }
    final dynamic effectiveValue = _toStringOrValue<T?>(value, key);
    if (key.secure) {
      return await _storage.write(key: key.key, value: effectiveValue);
    }
    await switch (effectiveValue) {
      (String value) => _preferences.setString(key.key, value),
      (int value) => _preferences.setInt(key.key, value),
      (bool value) => _preferences.setBool(key.key, value),
      (double value) => _preferences.setDouble(key.key, value),
      _ => throw StateError(
          'Invalid value for key ${key.key}! Got: ${effectiveValue.runtimeType}, Expected either String, int, bool or double')
    };
  }

  /// Deletes the key from the local cache and the storage.
  Future<void> delete<T>(StoreKey<T> key) async {
    _checkInitialized();
    _localCache.remove(key.key);
    if (key.secure) {
      return await _storage.delete(key: key.key);
    }
    _preferences.remove(key.key);
  }

  Future<T?> _readOrInitSecureStringKey<T>(StoreKey<T> key) async {
    if (key.type != String) {
      throw StateError('Secure keys must be of type String!');
    }
    final String? value = await _storage.read(key: key.key);
    final T? effectiveValue = value == null ? null : _fromStringOrValue(value, key);
    if (effectiveValue != null) {
      return effectiveValue;
    }
    T? defaultValue = _getDefault(key);
    if (defaultValue == null) {
      return null;
    }
    await write(key, effectiveValue);
    return defaultValue as T?;
  }

  FutureOr<T>? _readOrInitSharedPrefsKey<T>(StoreKey<T> key) {
    if (_preferences.containsKey(key.key)) {
      final dynamic value = _preferences.get(key.key);
      return _fromStringOrValue(value, key);
    }
    final T? defaultValue = _getDefault(key);
    if (defaultValue == null) {
      return null;
    }
    return write(key, defaultValue).then((_) => defaultValue);
  }

  void _checkInitialized() {
    if (!_initialized) {
      throw StateError('Store has not been initialized yet!');
    }
  }

  /// Returns the default value of the key.
  T? _getDefault<T>(StoreKey<T> key) {
    if (key.defaultFactory != null) {
      return key.defaultFactory!();
    }
    return key.defaultValue;
  }

  static dynamic _toStringOrValue<T>(T value, StoreKey<T> key) {
    final dynamic d = key.toValue?.call(value) ?? value;
    if (d is Enum) {
      return d.name;
    }
    return d;
  }

  static T? _fromStringOrValue<T>(dynamic value, StoreKey<T> key) {
    return key.fromValue?.call(value) ?? value as T?;
  }
}
