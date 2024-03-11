use std::sync::Arc;
use winit::window::Window;
use winit::event_loop::EventLoop;

pub fn run(mut window: Arc<Window>, mut event_loop: EventLoop<()>) {
    event_loop.run(move|event, elwt| {

    })
    .expect("Failed to run event loop!");
}
