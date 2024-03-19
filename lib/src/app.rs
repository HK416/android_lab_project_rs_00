use std::sync::Arc;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::event_loop::ControlFlow;
use winit::window::Window;

use crate::scene::GameScene;
use crate::scene::SampleScene;
use crate::timer::GameTimer;


#[cfg(target_os = "android")]
pub const RENDER_TARGET_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
#[cfg(not(target_os = "android"))]
pub const RENDER_TARGET_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;


/// #### 한국어 </br>
/// 이벤트 루프를 돌며 Android 애플리케이션을 동작하는 함수입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a function that runs an Android application while running an event loop. </br>
/// 
#[allow(dead_code)]
#[cfg(target_os = "android")]
pub fn run_android<'a>(event_loop: EventLoop<()>) {
    let mut window: Option<Arc<Window>> = None;
    let mut surface: Option<Arc<wgpu::Surface<'a>>> = None;
    let mut depth_stencil_view: Option<Arc<wgpu::TextureView>> = None;

    let (
        instance, 
        _adapter, 
        device, 
        queue
    ) = init_wgpu_renderer_without_surface();

    let mut sample_scene = SampleScene::new(&device, &queue);

    let mut timer = GameTimer::<50>::new();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run(|event, elwt| {
        match event {
            Event::AboutToWait => {
                timer.tick();
                sample_scene.on_update(timer.elapsed_time_sec(), &device, &queue);
                if let Some(window) = window.as_ref() {
                    window.request_redraw();
                }
            },
            Event::WindowEvent { window_id, event } 
            if window.as_ref().is_some_and(|window| window.id() == window_id) => match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    elwt.exit();
                }, 
                WindowEvent::RedrawRequested => {
                    if let Some((window, surface, depth_stencil_view)) = window.as_ref().and_then(|w| 
                        surface.as_ref().and_then(|s| 
                            depth_stencil_view.as_ref().and_then(|d| Some((w, s, d)))
                        )
                    ) {
                        // (한국어) `winit`에게 다음 프레임을 그릴 준비가 되었음을 알립니다.
                        // (English Translation) Notifies `winit` that the next frame is ready to be drawn.
                        window.pre_present_notify();

                        // (한국어) 이전 작업이 끝날 때 까지 기다립니다.
                        // (English Translation) Wait until the previous operation is finished. 
                        device.poll(wgpu::Maintain::Wait);

                        // (한국어) 다음 프레임을 가져옵니다.
                        // (English Translation) Get the next frame.
                        let frame = surface.get_current_texture()
                            .expect("Could not get the following frame buffer!");

                        // (한국어) 렌더 타겟의 텍스처 뷰를 생성합니다.
                        // (English Translation) Creates a texture view of render target.
                        let render_target_view = frame.texture.create_view(
                            &wgpu::TextureViewDescriptor { 
                                ..Default::default() 
                            }
                        );

                        sample_scene.on_draw(&render_target_view, &depth_stencil_view, &device, &queue);

                        // (한국어) 프레임 버퍼를 화면에 출력합니다.
                        // (English Translation) Prints the framebuffer to the screen. 
                        frame.present();
                    }
                }, 
                WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                    if let Some((window, surface)) = window.as_ref().zip(surface.as_ref()) {
                        // (한국어) 모든 작업이 끝날 때 까지 기다립니다.
                        // (English Translation) Wait until all operations are completed.
                        instance.poll_all(true);

                        setup_swapchain(size.width, size.height, &device, surface);
                        depth_stencil_view = Some(setup_depth_stencil_view(size.width, size.height, &device));

                        sample_scene.on_resized(&window, &device, &queue);
                    }
                },
                WindowEvent::Touch(touch) => {
                    sample_scene.on_touch_event(touch, &device, &queue);
                },
                _ => { /* empty */ }
            },
            Event::Suspended => {
                // (한국어) 기존의 윈도우 핸들과 `surface`를 제거합니다. 
                // (English Translation) Removes the existing window handle and `surface`.
                window = None;
                surface = None;
                depth_stencil_view = None;

                sample_scene.on_paused(&device, &queue);
            }, 
            Event::Resumed => {
                // (한국어) 새로운 윈도우 핸들과 `surface`를 생성합니다.
                // (English Translation) Creates a new window handle and `surface`.
                let a_window: Arc<Window> = Window::new(elwt)
                    .expect("Can't create window!")
                    .into();
                let target_window = a_window.clone();
                let a_surface: Arc<wgpu::Surface<'a>> = instance.create_surface(
                    wgpu::SurfaceTarget::from(target_window)
                    )
                    .expect("Can't create wgpu surface!")
                    .into();

                setup_swapchain(a_window.inner_size().width, a_window.inner_size().height, &device, &a_surface);
                depth_stencil_view = Some(setup_depth_stencil_view(
                    a_window.inner_size().width, 
                    a_window.inner_size().height, 
                    &device
                ));

                sample_scene.on_resumed(&a_window, &device, &queue);

                window = Some(a_window);
                surface = Some(a_surface);
            },
            _ => { /* empty */ }
        };
    })
    .expect("Failed to run event loop!");
}

#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
pub fn run_dev(window: Window, event_loop: EventLoop<()>) {
    let (
        instance, 
        surface, 
        _adapter, 
        device, 
        queue
    ) = init_wgpu_renderer(&window);

    let mut depth_stencil_view = setup_depth_stencil_view(
        window.inner_size().width, 
        window.inner_size().height, 
        &device
    );

    let mut sample_scene = SampleScene::new(&device, &queue);

    let mut timer = GameTimer::<50>::new();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run(|event, elwt| {
        match event {
            Event::AboutToWait => {
                timer.tick();
                sample_scene.on_update(timer.elapsed_time_sec(), &device, &queue);
                window.request_redraw();
            }
            Event::WindowEvent { window_id, event } 
            if window.id() == window_id => match event {
                WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                    elwt.exit();
                }, 
                WindowEvent::RedrawRequested => {
                    // (한국어) `winit`에게 다음 프레임을 그릴 준비가 되었음을 알립니다.
                    // (English Translation) Notifies `winit` that the next frame is ready to be drawn.
                    window.pre_present_notify();

                    // (한국어) 이전 작업이 끝날 때 까지 기다립니다.
                    // (English Translation) Wait until the previous operation is finished. 
                    device.poll(wgpu::Maintain::Wait);

                    // (한국어) 다음 프레임을 가져옵니다.
                    // (English Translation) Get the next frame.
                    let frame = surface.get_current_texture()
                        .expect("Could not get the following frame buffer!");

                    // (한국어) 렌더 타겟의 텍스처 뷰를 생성합니다.
                    // (English Translation) Creates a texture view of render target.
                    let render_target_view = frame.texture.create_view(
                        &wgpu::TextureViewDescriptor { 
                            ..Default::default() 
                        }
                    );

                    sample_scene.on_draw(&render_target_view, &depth_stencil_view, &device, &queue);

                    // (한국어) 프레임 버퍼를 화면에 출력합니다.
                    // (English Translation) Prints the framebuffer to the screen. 
                    frame.present();
                }, 
                WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                    // (한국어) 모든 작업이 끝날 때 까지 기다립니다.
                    // (English Translation) Wait until all operations are completed.
                    instance.poll_all(true);

                    setup_swapchain(size.width, size.height, &device, &surface);
                    depth_stencil_view = setup_depth_stencil_view(size.width, size.height, &device);

                    sample_scene.on_resized(&window, &device, &queue);
                },
                WindowEvent::KeyboardInput { event, .. } => {
                    sample_scene.on_keyboard_event(event, &device, &queue);
                }
                _ => { /* empty */ }
            },
            Event::Suspended => {
                sample_scene.on_paused(&device, &queue);
            }, 
            Event::Resumed => {
                sample_scene.on_resumed(&window, &device, &queue);
            },
            _ => { /* empty */ }
        };
    })
    .expect("Failed to run event loop!");
}

/// #### 한국어 </br>
/// `surface`를 제외한 wgpu 렌더링 인스턴스들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create wgpu rendering instances excluding `surface`. </br>
/// 
#[allow(dead_code)]
#[cfg(target_os = "android")]
fn init_wgpu_renderer_without_surface() -> (
    Arc<wgpu::Instance>, 
    Arc<wgpu::Adapter>, 
    Arc<wgpu::Device>, 
    Arc<wgpu::Queue>
) {
    let instance = wgpu::Instance::new(
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN, 
            ..Default::default()
        }, 
    );

    let adapter = pollster::block_on(
        instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                compatible_surface: None, 
                force_fallback_adapter: false, 
                power_preference: wgpu::PowerPreference::default(), 
            }, 
        )
    )
    .expect("Can't create wgpu adapter!");

    let (device, queue) = pollster::block_on(
        adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("GpuDevice"), 
                required_features: wgpu::Features::empty(), 
                required_limits: wgpu::Limits::default(), 
            }, 
            None
        )
    )
    .expect("No suitable device was found!");

    return (
        instance.into(), 
        adapter.into(), 
        device.into(), 
        queue.into()
    );
}

/// #### 한국어 </br>
/// wgpu 렌더링 인스턴스들을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create wgpu rendering instances. </br>
/// 
#[allow(dead_code)]
#[cfg(not(target_os = "android"))]
fn init_wgpu_renderer<'a>(window: &'a Window) -> (
    Arc<wgpu::Instance>, 
    Arc<wgpu::Surface<'a>>, 
    Arc<wgpu::Adapter>, 
    Arc<wgpu::Device>, 
    Arc<wgpu::Queue>
) {
    let instance = wgpu::Instance::new(
        wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY, 
            ..Default::default()
        }, 
    );

    let surface = instance.create_surface(
        wgpu::SurfaceTarget::from(window)
    )
    .expect("Can't create wgpu surface!");

    let adapter = pollster::block_on(
        instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                compatible_surface: None, 
                force_fallback_adapter: false, 
                power_preference: wgpu::PowerPreference::default(), 
            }, 
        )
    )
    .expect("Can't create wgpu adapter!");

    let (device, queue) = pollster::block_on(
        adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("GpuDevice"), 
                required_features: wgpu::Features::empty(), 
                required_limits: wgpu::Limits::default(), 
            }, 
            None
        )
    )
    .expect("No suitable device was found!");

    return (
        instance.into(), 
        surface.into(), 
        adapter.into(), 
        device.into(), 
        queue.into()
    );
}

/// #### 한국어 </br>
/// 스왑체인을 설정합니다. </br>
/// 
/// #### English (Translation) </br>
/// Setup the swapchain. </br>
/// 
#[allow(dead_code)]
fn setup_swapchain(width: u32, height: u32, device: &wgpu::Device, surface: &wgpu::Surface<'_>) {
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT, 
        format: RENDER_TARGET_FORMAT, 
        width, 
        height, 
        present_mode: wgpu::PresentMode::AutoVsync, 
        desired_maximum_frame_latency: 2, 
        alpha_mode: wgpu::CompositeAlphaMode::Auto, 
        view_formats: vec![],
    };
    surface.configure(device, &config);
}

/// #### 한국어 </br>
/// 깊이-스텐실 뷰를 생성합니다. </br>
/// 
/// English Translation </br> 
/// Create a depth-stencil view. </br>
/// 
#[allow(dead_code)]
fn setup_depth_stencil_view(width: u32, height: u32, device: &wgpu::Device) -> Arc<wgpu::TextureView> {
    device.create_texture(
        &wgpu::TextureDescriptor {
            label: Some("DepthStencilBuffer"), 
            format: wgpu::TextureFormat::Depth32Float, 
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            dimension: wgpu::TextureDimension::D2, 
            mip_level_count: 1, 
            sample_count: 1,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT, 
            view_formats: &[],
        }
    ).create_view(&wgpu::TextureViewDescriptor { 
        ..Default::default()
    }).into()
}
