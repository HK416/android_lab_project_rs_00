mod item;
mod render;

mod app;
mod object;
mod scene;
mod timer;

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

    app::run_android(event_loop);
}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
pub fn main() {
    use winit::window::Window;
    use winit::window::WindowBuilder;
    use winit::event_loop::EventLoop;

    let event_loop: EventLoop<()> = EventLoop::new()
        .expect("Can't create event loop!");
    let window: Window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Can't create window");

    app::run_dev(window, event_loop);
}
