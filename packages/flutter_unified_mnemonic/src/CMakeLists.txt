# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

project(flutter_unified_mnemonic_library VERSION 0.0.1 LANGUAGES C)

add_library(flutter_unified_mnemonic SHARED
  "flutter_unified_mnemonic.c"
)

set_target_properties(flutter_unified_mnemonic PROPERTIES
  PUBLIC_HEADER flutter_unified_mnemonic.h
  OUTPUT_NAME "flutter_unified_mnemonic"
)

target_compile_definitions(flutter_unified_mnemonic PUBLIC DART_SHARED_LIB)
