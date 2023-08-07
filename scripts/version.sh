#!/bin/bash

for LIBRARY_NAME in bip39 lightning_message
do
  CURR_VERSION=${LIBRARY_NAME}-v`awk '/^version: /{print $2}' packages/${LIBRARY_NAME}/pubspec.yaml`

  # iOS & macOS
  APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
  sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/${LIBRARY_NAME}/ios/${LIBRARY_NAME}.podspec
  sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/${LIBRARY_NAME}/macos/${LIBRARY_NAME}.podspec
  rm packages/${LIBRARY_NAME}/macos/*.bak packages/${LIBRARY_NAME}/ios/*.bak

  # CMake platforms (Linux, Windows, and Android)
  CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
  for CMAKE_PLATFORM in android linux windows
  do
      sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/${LIBRARY_NAME}/${CMAKE_PLATFORM}/CMakeLists.txt
      rm packages/${LIBRARY_NAME}/${CMAKE_PLATFORM}/*.bak
  done

  git add packages/${LIBRARY_NAME}/
done
