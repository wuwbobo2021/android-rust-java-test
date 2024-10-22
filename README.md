# android-rust-java-test

Minimum code for Rust crates that supports Android through JNI and Java helper. It was initially extracted from <https://github.com/slint-ui/slint>, crate `i-slint-backend-android-activity`.

Without the Java helper, the library crate needs `jni`, `ndk` and `ndk-context` for the Android target, and the application crate (which uses the library) might rely on `android-actifity` and `cargo-apk` for Android.

However, many Android features requires using Java API, and it is likely that JNI cannot do everything at the Java side, thus a small portion of the library crate has to be written in Java, which is called "helper" here.

Instead of using Gradle, there is a `build.rs` that compiles the Java code by `javac` and generates the dex file by `d8` in Android SDK `build-tools` (latest version installed). The dex file is embeded into the `cdylib` binary at compile time by Rust macro `include_bytes!()`. At runtime, the Rust program creates the helper object (after attaching to the JVM) by obtaining some "DexClassLoader" through JNI, loading the dex data, and finding the correct helper class.

In the initial version extracted from Slint 1.8, no special options are passed to `d8`, and the Rust code loads the dex data by `dalvik.system.InMemoryDexClassLoader` introduced in API Level 26 (Android 8.0). In fact it doesn't work on Android 8.0 or 9.0 (that's the same problem described in <https://github.com/slint-ui/slint/issues/4936>).

The problem with Android 8.0 is fixed here by obtaining a parent class loader from the android context (native activity), passing it (instead of `null`) to the constructor of `InMemoryDexClassLoader`.

Android 6.0 or 7.0 can be supported by a workaround. The program checks the SDK version at runtime, if it is below 26, it falls back to the old way of writing the dex data into the application internal storage before loading it with `DexClassLoader`. And `--min-api <sdk_version>` is needed for executing `d8` at compile time, here the version should be lower than 21 (Android 5.0), because the multidex format is supported since Android 5.0, but somehow the `DexClassLoader` cannot load such file successfully (a clue: <https://github.com/getodk/collect/issues/1300>). Generally, support of multiple dex files is not needed here.
