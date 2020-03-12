#!/bin/bash
cd rust
cargo ndk --platform 28 --target x86_64-linux-android build --release
cargo ndk --platform 28 --target aarch64-linux-android build --release
cargo ndk --platform 28 --target armv7-linux-androideabi build --release
cargo ndk --platform 28 --target i686-linux-android build --release
echo built!
