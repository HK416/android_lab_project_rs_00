use std::fmt;
use winit::window::Window;



/// #### 한국어 </br>
/// 게임 장면의 trait입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a trait in the game scene. </br>
/// 
#[allow(unused_variables)]
pub trait GameScene : fmt::Debug {
    #[inline]
    fn on_paused(
        &mut self, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) { /* empty */ }

    #[inline]
    fn on_resumed(
        &mut self, 
        window: &Window, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) { /* empty */ }

    #[inline]
    fn on_resized(
        &mut self, 
        window: &Window, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) { /* empty */ }

    #[inline]
    fn on_update(
        &mut self, 
        elapsed_time_sec: f32, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) { /* empty */ }

    #[inline]
    fn on_draw(
        &self, 
        render_target_view: &wgpu::TextureView, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) { /* empty */ }
}

#[derive(Debug)]
pub struct SampleScene {
}

impl SampleScene {
    pub fn new(_device: &wgpu::Device, _queue: &wgpu::Queue) -> Self {
        Self {  }
    }
}

impl GameScene for SampleScene {
    fn on_draw(
        &self, 
        render_target_view: &wgpu::TextureView, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let _rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(Test)"), 
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: render_target_view, 
                            resolve_target: None, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), 
                                store: wgpu::StoreOp::Store, 
                            }, 
                        }), 
                    ], 
                    depth_stencil_attachment: None, 
                    occlusion_query_set: None, 
                    timestamp_writes: None, 
                }, 
            );
        }

        // (한국어) 명령어 대기열에 명령어 목록을 제출합니다. </br>
        // (English Translation) Submit a list of commands to the command queue. </br>
        queue.submit(Some(encoder.finish()));
    }
}
