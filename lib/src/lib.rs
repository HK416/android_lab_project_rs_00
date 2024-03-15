mod app;
mod object;
mod scene;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;



#[no_mangle]
#[allow(dead_code)]
#[cfg(target_os = "android")]
pub fn android_main(app: AndroidApp) {
    use winit::event_loop::EventLoop;
    use winit::event_loop::EventLoopBuilder;
    use winit::platform::android::EventLoopBuilderExtAndroid;
    use android_logger::Config;

    android_logger::init_once(Config::default());

    let event_loop: EventLoop<()> = EventLoopBuilder::new()
        .with_android_app(app)
        .build()
        .expect("Can't create event loop!");

    app::run(event_loop);
}
