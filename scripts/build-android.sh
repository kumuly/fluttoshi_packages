#!/bin/bash

for LIBRARY_NAME in bip39 lightning_message
do
    # Setup
    BUILD_DIR=platform-build
    mkdir $BUILD_DIR
    mkdir $BUILD_DIR/${LIBRARY_NAME}
    cd $BUILD_DIR

    # Create the jniLibs build directory
    JNI_DIR=jniLibs
    mkdir -p $JNI_DIR

    # Set up cargo-ndk
    cargo install cargo-ndk
    rustup target add \
            aarch64-linux-android \
            armv7-linux-androideabi \
            x86_64-linux-android \
            i686-linux-android

    # Build the android libraries in the jniLibs directory
    cargo ndk -o $JNI_DIR \
            --manifest-path ../Cargo.toml \
            -t armeabi-v7a \
            -t arm64-v8a \
            -t x86 \
            -t x86_64 \
            build --release 

    # Archive the dynamic libs
    cd $JNI_DIR
    tar -czvf ../${LIBRARY_NAME}/android.tar.gz *
    cd -

    # Cleanup
    rm -rf $JNI_DIR
    # Back to root
    cd ..
done