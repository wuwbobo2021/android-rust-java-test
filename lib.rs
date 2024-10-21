use jni::{
    objects::{JClass, JObject, JValue},
    JNIEnv,
};

pub struct JavaHelper(jni::objects::GlobalRef);

impl JavaHelper {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let ctx = ndk_context::android_context();
        // Safety: as documented in ndk-context to obtain a jni::JavaVM
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let context = unsafe { JObject::from_raw(ctx.context().cast()) }; // native activity
        let env = &mut vm.attach_current_thread()?;

        let dex_data = include_bytes!(concat!(env!("OUT_DIR"), "/classes.dex"));

        // Safety: dex_data is 'static and the InMemoryDexClassLoader will not mutate it it
        let dex_buffer = unsafe {
            env.new_direct_byte_buffer(dex_data.as_ptr() as *mut _, dex_data.len())
                .unwrap()
        };

        let dex_loader = env.new_object(
            "dalvik/system/InMemoryDexClassLoader",
            "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V",
            &[
                JValue::Object(&dex_buffer),
                JValue::Object(&JObject::null()),
            ],
        )?;

        let class_name = env.new_string("JavaHelper")?;
        let helper_class = env
            .call_method(
                dex_loader,
                "findClass",
                "(Ljava/lang/String;)Ljava/lang/Class;",
                &[JValue::Object(&class_name)],
            )?
            .l()?;
        let helper_class: JClass = helper_class.into();

        let helper_instance = env.new_object(
            helper_class,
            "(Landroid/app/Activity;)V",
            &[JValue::Object(&context)],
        )?;
        Ok(Self(env.new_global_ref(&helper_instance)?))
    }

    pub fn test_get_string(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.with_jni_env(|env, helper| {
            let j_string = env
                .call_method(helper, "test_get_string", "()Ljava/lang/String;", &[])?
                .l()?;
            let string: String = env.get_string(&j_string.into())?.into();
            Ok(string)
        })
        .map_err(|e| e.into())
    }

    fn with_jni_env<R>(
        &self,
        f: impl FnOnce(&mut JNIEnv, &JObject<'static>) -> Result<R, jni::errors::Error>,
    ) -> Result<R, jni::errors::Error> {
        let ctx = ndk_context::android_context();
        // Safety: as documented in ndk-context to obtain a jni::JavaVM
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }?;
        let env = &mut vm.attach_current_thread()?;
        let helper = self.0.as_obj();
        f(env, helper)
    }
}
