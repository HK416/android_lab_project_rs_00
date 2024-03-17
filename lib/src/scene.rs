use std::sync::Arc;
use std::any::TypeId;
use std::collections::HashMap;
use hecs::World;
use hecs::Entity;
use winit::window::Window;

use crate::item::color::Color;
use crate::item::projection::Projection;
use crate::item::transform::Transform;
use crate::item::transform::TransformBuilder;
use crate::render::mesh::ModelMesh;
use crate::render::pipeline::GraphicsPipeline;
use crate::render::uniform::CameraUniformLayout;
use crate::render::uniform::CameraUniform;
use crate::render::uniform::EntityUniformLayout;
use crate::render::uniform::EntityUniform;



/// #### 한국어 </br>
/// 게임 장면의 trait입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is a trait in the game scene. </br>
/// 
#[allow(unused_variables)]
pub trait GameScene {
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
        depth_stencil_view: &wgpu::TextureView, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) { /* empty */ }
}

pub struct SampleScene {
    world: World, 
    main_camera: Entity, 
}

impl SampleScene {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        use crate::render::mesh::create_plane_mesh;
        use crate::render::pipeline::create_colored_pipeline;

        // (한국어) 바인드 그룹 레이아웃들을 생성합니다.
        // (English Translation) Create a bind group layouts. 
        let layouts: Arc<HashMap<TypeId, wgpu::BindGroupLayout>> = HashMap::from([
            (TypeId::of::<CameraUniformLayout>(), CameraUniformLayout::layout(device)),
            (TypeId::of::<EntityUniformLayout>(), EntityUniformLayout::layout(device)),
        ]).into();

        // (한국어) 메쉬들을 생성합니다.
        // (English Translation) Create the meshes.
        let plane_mesh = create_plane_mesh(5.0, 5.0, device, queue);

        // (한국어) 그래픽스 파이프라인들을 생성합니다. 
        // (English Translation) Create graphics pipelines. 
        let colored_pipeline = create_colored_pipeline(&layouts, device);

        // (한국어) 엔티티들을 생성합니다.
        // (English Translation) Create entities.
        let mut world = World::new();
        let camera = world.spawn((
            Projection::perspective(
                60.0f32.to_radians(), 
                1920.0 / 1080.0, 
                0.0001, 
                1000.0
            ), 
            TransformBuilder::new()
                .set_translation((0.0, 3.0, 5.0).into())
                .rotate_from_x_axis(-30.0f32.to_radians())
                .build(), 
            CameraUniform::new(&layouts, device), 
        ));

        let _plane = world.spawn((
            Color::from_rgb(0.8, 0.2, 0.2), 
            Transform::new(), 
            EntityUniform::new(&layouts, device), 
            plane_mesh.clone(), 
            colored_pipeline.clone(), 
        ));

        // (한국어) 카메라의 유니폼 버퍼를 갱신합니다.
        // (English Translation) Updates the camera's uniform buffer.
        for (_, (projection, transform, uniform)) in world.query::<(&Projection, &Transform, &CameraUniform)>().iter() {
            uniform.update(
                queue, 
                CameraUniformLayout {
                    proj_view: projection.as_ref().mul_mat4(&transform.view_matrix()), 
                    position: transform.get_translation(), 
                    ..Default::default()
                }
            );
        }

        // (한국어) 엔티티의 유니폼 버퍼를 갱신합니다.
        // (English Translation) Updates the entity's uniform buffer. 
        for (_, (color, transform, uniform)) in world.query::<(&Color, &Transform, &EntityUniform)>().iter() {
            uniform.update(
                queue,
                EntityUniformLayout {
                    color: color.as_vec4(), 
                    world: transform.world_matrix_ref().clone(), 
                }
            );
        }

        Self { 
            world, 
            main_camera: camera, 
        }
    }
}

impl GameScene for SampleScene {
    fn on_update(
        &mut self, 
        elapsed_time_sec: f32, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) {

    }

    fn on_draw(
        &self, 
        render_target_view: &wgpu::TextureView, 
        depth_stencil_view: &wgpu::TextureView, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) {
        let mut query = self.world.query_one::<&CameraUniform>(self.main_camera).unwrap();
        let main_camera = query.get().unwrap();

        let mut render_map: HashMap<GraphicsPipeline, HashMap<ModelMesh, Vec<EntityUniform>>> = HashMap::new();
        for (_, (pipeline, model_mesh, uniform)) in self.world.query::<(&GraphicsPipeline, &ModelMesh, &EntityUniform)>().iter() {
            render_map.entry(pipeline.clone())
                .and_modify(|map| {
                    map.entry(model_mesh.clone())
                        .and_modify(|vec| {
                            vec.push(uniform.clone())
                        })
                        .or_insert(vec![uniform.clone()]);
                })
                .or_insert(HashMap::from([(
                    model_mesh.clone(), 
                    vec![uniform.clone()]
                )]));
        }

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(Draw)"), 
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
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment { 
                        view: &depth_stencil_view , 
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0), 
                            store: wgpu::StoreOp::Store, 
                        }), 
                        stencil_ops: None, 
                    }), 
                    occlusion_query_set: None, 
                    timestamp_writes: None, 
                }, 
            );

            for (pipeline, map) in render_map.iter() {
                pipeline.bind(&mut rpass);
                main_camera.bind(&mut rpass, 0);
                for (model_mesh, entities) in map.iter() {
                    model_mesh.bind(&mut rpass, 0);
                    for entity in entities.iter() {
                        entity.bind(&mut rpass, 1);
                        model_mesh.draw(&mut rpass, 0..1);
                    }
                }
            }
        }

        // (한국어) 명령어 대기열에 명령어 목록을 제출합니다. </br>
        // (English Translation) Submit a list of commands to the command queue. </br>
        queue.submit(Some(encoder.finish()));
    }
}
