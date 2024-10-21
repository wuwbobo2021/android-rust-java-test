use android_activity::AndroidApp;

#[no_mangle]
fn android_main(_app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let helper = android_rust_java_test::JavaHelper::new().unwrap_or_else(|e| panic!("{e}"));
    let s = helper.test_get_string();
    println!("{:?}", s);
}
