use android_activity::AndroidApp;
use android_rust_java_test::JavaHelper;

#[no_mangle]
fn android_main(_app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let helper_1 = JavaHelper::new().unwrap_or_else(|e| panic!("{e}"));
    let helper_2 = JavaHelper::new().unwrap_or_else(|e| panic!("{e}"));
    for _ in 0..3 {
        println!("helper_1: {:?}", helper_1.test_get_string());
        println!("helper_2: {:?}", helper_2.test_get_string());
    }
}
