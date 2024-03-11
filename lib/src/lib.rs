mod app;

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: AndroidApp) {
    use std::sync::Arc;
    use winit::window::Window;
    use winit::window::WindowBuilder;
    use winit::event_loop::EventLoopBuilder;
    use winit::platform::android::EventLoopBuilderExtAndroid;

    let event_loop = EventLoopBuilder::new()
        .with_android_app(app)
        .build()
        .expect("Can't create event loop!");

    let window: Arc<Window> = WindowBuilder::new()
        .build(&event_loop)
        .expect("Can't create window!")
        .into();

    app::run(window, event_loop);
}

#[allow(dead_code)]
fn main() {
    use std::sync::Arc;
    use winit::window::Window;
    use winit::window::WindowBuilder;
    use winit::event_loop::EventLoop;

    let event_loop = EventLoop::new()
        .expect("Can't create event loop!");
    
    let window: Arc<Window> = WindowBuilder::new()
        .build(&event_loop)
        .expect("Can't create window!")
        .into();

    app::run(window, event_loop);
}
