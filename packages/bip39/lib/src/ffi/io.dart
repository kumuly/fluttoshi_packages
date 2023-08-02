import 'dart:ffi';

import 'package:bip39/src/bridge_generated.dart';

/// A typedef for the ExternalLibrary, which is a wrapper for a
/// platform-specific library of functions and data, such as a DLL, dylib,
/// or .so file.
typedef ExternalLibrary = DynamicLibrary;

/// Creates a platform-specific instance of the [Bip39] class using
/// the [dylib] as the platform-specific library.
///
/// The [Bip39] class defines the methods available in the
/// bip39 plugin. The actual implementations of these methods
/// are provided by the platform-specific library referred to by [dylib].
///
/// Returns a [Bip39] instance.
Bip39 createWrapperImpl(ExternalLibrary dylib) => Bip39Impl(dylib);
