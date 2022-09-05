# usefull when you work on a patch in quad-cargo-apk itself
#cd ../quad-cargo-apk
#cargo build
#cd -

# https://macroquad.rs/articles/android/#building-an-apk-a-manual-way
export ANDROID_HOME=PATH_TO_ANDROID_HOME
export NDK_HOME=PATH_TO_ANDROID_SDK
../quad-cargo-apk/target/debug/cargo-quad-apk build --nostrip
adb install target/android-artifacts/debug/apk/quad-android-playground.apk

