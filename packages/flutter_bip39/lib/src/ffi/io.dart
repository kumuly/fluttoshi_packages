import 'dart:ffi';
import 'dart:io';

/// Function that creates a platform-specific instance of the `DynamicLibrary`.
///
/// This function leverages the `dart:ffi` and `dart:io` libraries to handle the
/// native APIs for creating an instance of `DynamicLibrary`.
///
/// The function checks the platform it's running on to determine the type of
/// library it needs to load.
/// string which is the common part of the library file name across all
/// platforms.
///
/// On iOS or macOS, it loads the dynamic library linked into the executable.
/// On Windows, it loads a `.dll` file, and on other platforms,
/// it loads a `.so` file.
///
/// Returns an instance of `DynamicLibrary`, opened according to the specific
/// platform rules.
DynamicLibrary createLibraryImpl() {
  const base = 'bip39';

  if (Platform.isIOS || Platform.isMacOS) {
    return DynamicLibrary.executable();
  } else if (Platform.isWindows) {
    return DynamicLibrary.open('$base.dll');
  } else {
    return DynamicLibrary.open('lib$base.so');
  }
}
