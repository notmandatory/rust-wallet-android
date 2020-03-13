Rust Wallet Android AAR
=======================

This is a very WIP project to wrap the 
[rust-wallet](https://github.com/rust-bitcoin/rust-wallet) library in to an 
android aar library to be used in an Android bitcoin wallet app.

## Install rust-wallet-android libraries

1. Install [Android Studio](https://developer.android.com/studio) and include these components:
* SDK build tools
* LLDB
* Android SDK Command-line Tools
* Android SDK platform tools
* SDK platform Android 9.0 Pie (API level 28) 	    # or whichever is suitable for your device

2. Install Rust targets (if not already installed)
   
   Android: 
   ```
   rustup target add x86_64-linux-android aarch64-linux-android armv7-linux-androideabi i686-linux-android
   ```
   
   iOS:
   ```
   rustup target add aarch64-apple-ios armv7-apple-ios armv7s-apple-ios x86_64-apple-ios i386-apple-ios
   ```

3. Install [cargo-ndk](https://docs.rs/crate/cargo-ndk/0.6.1) cargo extension:
   
   Android:
   ```
   cargo install cargo-ndk
   ```

   iOS:
   ```
   cargo install cargo-lipo
   ```

4. Set environment variables needed to build rust based library files and
   to run local unit tests. Better yet add these to your `.bash_profile`

   Android:
    ```
    export ANDROID_HOME=$HOME/Library/Android
    export NDK_HOME=$ANDROID_HOME/sdk/ndk/<ndk version, eg. 21.0.6113669>
    export JAVA_LIBRARY_PATH=<project_home>/lib/src/main/jniLibs/x86_64
    ```

    iOS:
    ```
    ## if this fails:
    xcrun -k --sdk iphoneos --show-sdk-path
    ## run this:
    sudo xcode-select --switch /Applications/Xcode.app
    ```

5. Set environment variables needed to build Bitcoin C++ library files. This will be unnecessary after [fix](https://github.com/bbqsrc/cargo-ndk/pull/7) to [cargo-ndk](https://docs.rs/crate/cargo-ndk/0.6.1).

    ```
    export CXX_x86_64-linux-android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android28-clang++

    export CXX_aarch64_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android28-clang++
    
    export CXX_armv7_linux_androideabi=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi28-clang++
    
    export CXX_i686_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android28-clang++
    ```

6. Build Rust library files for all target platform OS architectures:
    
   ```
   ./build-lib.sh
   ```

7. Install Rust library files for all target platform OS architectures:
   ```
   ./install-lib.sh
   ```

   
8. Deploy AAR to local maven repository
   
   ```
   gradle clean build uploadArchives
   ```