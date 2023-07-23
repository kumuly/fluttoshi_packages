import 'dart:ffi';

import 'package:unified_mnemonic/src/bridge_generated.dart';

/// A typedef for the ExternalLibrary, which is a wrapper for a
/// platform-specific library of functions and data, such as a DLL, dylib,
/// or .so file.
typedef ExternalLibrary = DynamicLibrary;

/// Creates a platform-specific instance of the [UnifiedMnemonic] class using
/// the [dylib] as the platform-specific library.
///
/// The [UnifiedMnemonic] class defines the methods available in the
/// unified_mnemonic plugin. The actual implementations of these methods
/// are provided by the platform-specific library referred to by [dylib].
///
/// Returns a [UnifiedMnemonic] instance.
UnifiedMnemonic createWrapperImpl(ExternalLibrary dylib) =>
    UnifiedMnemonicImpl(dylib);
