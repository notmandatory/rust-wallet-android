Rust Wallet Android AAR
=======================

This is a very WIP project to wrap the 
[rust-wallet](https://github.com/rust-bitcoin/rust-wallet) library in to an 
android aar library to be used in an Android bitcoin wallet app.

## Install rust-wallet-android libraries

1. Install rust targets (if not already installed)
   
   ```
   rustup target add aarch64-linux-android
   rustup target add armv7-linux-androideabi
   rustup target add i686-linux-android
   ```

1. Set environment variables needed to build rust based library files and
   to run local unit tests. Better yet add these to your `.bash_profile`

    ```
    export JAVA_LIBRARY_PATH=<project_home>/lib/src/main/jniLibs/x86_64
    
    export CC_aarch64_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android24-clang
    export CXX_aarch64_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android24-clang++
    export AR_aarch64_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/aarch64-linux-android-ar
    
    export CC_armv7_linux_androideabi=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi24-clang
    export CXX_armv7_linux_androideabi=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/armv7a-linux-androideabi24-clang++
    export AR_armv7_linux_androideabi=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/arm-linux-androideabi-ar
    
    export CC_i686_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android24-clang
    export CXX_i686_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android24-clang++
    export AR_i686_linux_android=$NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin/i686-linux-android-ar
    ```

1. Build and install rust based library files for all target platform os architectures
    
   ```
   ./build-lib.sh
   ./install-lib.sh
   ```
   
1. Deploy AAR to local maven repository
   
   ```
   gradle clean build uploadArchives
   ```