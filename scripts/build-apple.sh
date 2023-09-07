#!/bin/bash

for LIBRARY_NAME in bip39
do
    # Setup
    BUILD_DIR=platform-build
    mkdir $BUILD_DIR
    LIBRARY_DIR=$BUILD_DIR/${LIBRARY_NAME}
    mkdir $LIBRARY_DIR
    cd $LIBRARY_DIR

    # Build static libs
    for TARGET in \
            aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim \
            x86_64-apple-darwin aarch64-apple-darwin
    do
        rustup target add $TARGET
        cargo build -r --target=$TARGET
    done

    # Create XCFramework zip
    FRAMEWORK="$(echo "${LIBRARY_NAME}" | awk 'BEGIN{FS=OFS="_"}{for(i=1;i<=NF;i++){sub(".",toupper(substr($i,1,1)),$i)}}1' | sed 's/_//g').xcframework"
    LIBNAME=lib${LIBRARY_NAME}.a
    mkdir mac-lipo ios-sim-lipo
    IOS_SIM_LIPO=ios-sim-lipo/$LIBNAME
    MAC_LIPO=mac-lipo/$LIBNAME
    lipo -create -output $IOS_SIM_LIPO \
            ../../target/aarch64-apple-ios-sim/release/$LIBNAME \
            ../../target/x86_64-apple-ios/release/$LIBNAME
    lipo -create -output $MAC_LIPO \
            ../../target/aarch64-apple-darwin/release/$LIBNAME \
            ../../target/x86_64-apple-darwin/release/$LIBNAME
    xcodebuild -create-xcframework \
            -library $IOS_SIM_LIPO \
            -library $MAC_LIPO \
            -library ../../target/aarch64-apple-ios/release/$LIBNAME \
            -output $FRAMEWORK
    zip -r $FRAMEWORK.zip $FRAMEWORK

    # Cleanup
    rm -rf ios-sim-lipo mac-lipo $FRAMEWORK
    # Back to root
    cd ../..
done