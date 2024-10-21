# android-rust-java-test

Minimum code for Rust crates that supports Android through JNI and Java helper. Extracted from <https://github.com/slint-ui/slint>.

Without the Java helper, the library crate needs `jni`, `ndk` and `ndk-context` for the Android target, and the application crate (which uses the library) might rely on `android-actifity` and `cargo-apk` for Android.

However, it's likely that JNI cannot call everything at the Java side, and a small portion of the program has to be written in Java, which is called "helper" here.

Instead of using Gradle, there is a `build.rs` that compiles the Java code by `javac` and generates the dex file by `d8` in Android SDK `build-tools` (latest version installed). At runtime, the Rust code creates the helper object (after attaching to the JVM) by obtaining some "DexClassLoader" through JNI, loading the dex data, and finding the correct helper class.

## Current status
In the current version, no special options are passed to `d8`, and the Rust code loads the dex data by `dalvik.system.InMemoryDexClassLoader` introduced in API Level 26 (Android 8.0). In fact it doesn't work on Android 8.0 or 9.0 (that's the same problem described in <https://github.com/slint-ui/slint/issues/4936>).
