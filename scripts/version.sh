#!/bin/bash

for PACKAGE in unified_mnemonic lightning_message
do
  CURR_VERSION=${PACKAGE}-v`awk '/^version: /{print $2}' packages/${PACKAGE}/pubspec.yaml`

  # iOS & macOS
  APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
  sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/${PACKAGE}/ios/${PACKAGE}.podspec
  sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/${PACKAGE}/macos/${PACKAGE}.podspec
  rm packages/${PACKAGE}/macos/*.bak packages/${PACKAGE}/ios/*.bak

  # CMake platforms (Linux, Windows, and Android)
  CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
  for CMAKE_PLATFORM in android linux windows
  do
      sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/${PACKAGE}/${CMAKE_PLATFORM}/CMakeLists.txt
      rm packages/${PACKAGE}/${CMAKE_PLATFORM}/*.bak
  done

  git add packages/${PACKAGE}/
done
