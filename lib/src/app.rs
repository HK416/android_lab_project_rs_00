use std::sync::Arc;
use winit::window::Window;
use winit::event_loop::EventLoop;

pub fn run(mut window: Arc<Window>, mut event_loop: EventLoop<()>) {
    let (
        instance,
        surface, 
        adapter, 
        device, 
        queue
    ) = init_wgpu_renderer(window.clone());

    event_loop.run(move|event, elwt| {
        // TODO
    })
    .expect("Failed to run event loop!");
}

fn init_wgpu_renderer(window: Arc<Window>) -> (
    Arc<wgpu::Instance>, 
    Arc<wgpu::Surface<'static>>, 
    Arc<wgpu::Adapter>, 
    Arc<wgpu::Device>, 
    Arc<wgpu::Queue>
) {
    let instance: wgpu::Instance = wgpu::Instance::new(
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN, 
            ..Default::default()
        }
    );

    let surface: wgpu::Surface<'_> = instance.create_surface(
        wgpu::SurfaceTarget::from(window)
    )
    .expect("Can't create wgpu surface!");

    let adapter = pollster::block_on(
        instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase {
                compatible_surface: Some(&surface), 
                force_fallback_adapter: false, 
                power_preference: wgpu::PowerPreference::default(), 
            }, 
        ), 
    )
    .expect("Can't create wgpu adapter");

    let (device, queue) = pollster::block_on(
        adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("DeviceDescriptor"), 
                required_features: wgpu::Features::empty(), 
                required_limits: wgpu::Limits::default(), 
            }, 
            None
        )
    )
    .expect("No suitable device was found.");

    return (
        instance.into(), 
        surface.into(), 
        adapter.into(), 
        device.into(), 
        queue.into()
    );
}
