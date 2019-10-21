#!/bin/bash
cp target/aarch64-linux-android/release/librust_wallet_android.so ../rust-wallet-android-app/app/src/main/jniLibs/arm64-v8a/librust_wallet_android.so
cp target/armv7-linux-androideabi/release/librust_wallet_android.so ../rust-wallet-android-app/app/src/main/jniLibs/armeabi-v7a/librust_wallet_android.so
cp target/i686-linux-android/release/librust_wallet_android.so ../rust-wallet-android-app/app/src/main/jniLibs/x86/librust_wallet_android.so
