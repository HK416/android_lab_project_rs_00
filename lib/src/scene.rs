use std::any::Any;
use std::sync::Arc;
use std::any::TypeId;
use std::collections::HashMap;
use hecs::World;
use hecs::Entity;
use winit::window::Window;
use winit::event::Touch;
use winit::event::TouchPhase;
use winit::event::KeyEvent;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey;

use crate::item::color::Color;
use crate::item::projection::PerspectiveBuilder;
use crate::item::projection::Projection;
use crate::item::transform::Transform;
use crate::item::transform::TransformBuilder;
use crate::render::mesh::ModelMesh;
use crate::render::pipeline::GraphicsPipeline;
use crate::render::pipeline::ColoredPipeline;
use crate::render::pipeline::TransparentPipeline;
use crate::render::pipeline::CompositePipeline;
use crate::render::texture::WeightedBlendedOIT;
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
    fn on_touch_event(
        &mut self, 
        touch: Touch, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) { /* empty */}

    #[inline]
    fn on_keyboard_event(
        &mut self, 
        event: KeyEvent, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) { /* empty */}

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
    camera_force: f32, 

    touch_id: Option<u64>, 
    touch_prev_x: f32, 

    layouts: Arc<HashMap<TypeId, wgpu::BindGroupLayout>>, 
    weighted_blended_oit: Option<WeightedBlendedOIT>,

    graphics_pipelines: HashMap<TypeId, Box<dyn GraphicsPipeline>>,
}

impl SampleScene {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        use crate::render::mesh::create_plane_mesh;
        use crate::render::mesh::create_cube_mesh;

        // (한국어) 바인드 그룹 레이아웃들을 생성합니다.
        // (English Translation) Create a bind group layouts. 
        let layouts: Arc<HashMap<TypeId, wgpu::BindGroupLayout>> = HashMap::from([
            (TypeId::of::<CameraUniformLayout>(), CameraUniformLayout::layout(device)),
            (TypeId::of::<EntityUniformLayout>(), EntityUniformLayout::layout(device)),
            (TypeId::of::<WeightedBlendedOIT>(), WeightedBlendedOIT::layout(device)), 
        ]).into();

        // (한국어) 메쉬들을 생성합니다.
        // (English Translation) Create the meshes.
        let plane_mesh = create_plane_mesh(5.0, 5.0, device, queue);
        let cube_mesh_0 = create_cube_mesh(0.7, 0.7, 0.7, device, queue);
        let cube_mesh_1 = create_cube_mesh(0.3, 0.3, 0.3, device, queue);

        // (한국어) 그래픽스 파이프라인들을 생성합니다. 
        // (English Translation) Create graphics pipelines. 
        let colored_pipeline = ColoredPipeline::new(device, &layouts);
        let transparent_pipeline = TransparentPipeline::new(device, &layouts);
        let composite_pipeline = CompositePipeline::new(device, &layouts);

        // (한국어) 엔티티들을 생성합니다.
        // (English Translation) Create entities.
        let mut world = World::new();
        let camera = world.spawn((
            Projection::Perspective(
                PerspectiveBuilder::new()
                    .set_aspect_ratio(16.0 / 9.0)
                    .build()
            ), 
            TransformBuilder::new()
                .set_translation((0.0, 2.0, 5.0).into())
                .rotate_from_x_axis(-22.0f32.to_radians())
                .build(), 
            CameraUniform::new(&layouts, device), 
        ));

        let _plane = world.spawn((
            Color::Rgb { red: 0.68, green: 0.68, blue: 0.68 }, 
            Transform::new(), 
            EntityUniform::new(&layouts, device), 
            plane_mesh.clone(), 
            colored_pipeline.clone(), 
        ));

        let _red_cube = world.spawn((
            Color::Rgb { red: 0.8, green: 0.2, blue: 0.2 },
            TransformBuilder::new()
                .set_translation((0.0, 0.5, 0.0).into())
                .build(), 
            EntityUniform::new(&layouts, device), 
            cube_mesh_0.clone(), 
            colored_pipeline.clone(), 
        ));

        let _green_cube = world.spawn((
            Color::Rgb { red: 0.2, green: 0.8, blue: 0.2 }, 
            TransformBuilder::new()
                .set_translation((-1.0, 0.88, 0.67).into())
                .rotate_from_axis_angle((1.0, 1.0, 0.0).into(), 30.0f32.to_radians())
                .build(), 
            EntityUniform::new(&layouts, device), 
            cube_mesh_0.clone(), 
            colored_pipeline.clone(), 
        ));

        let _blue_cube = world.spawn((
            Color::Rgb { red: 0.2, green: 0.2, blue: 0.8 }, 
            TransformBuilder::new()
                .set_translation((1.33, 1.2, -0.25).into())
                .rotate_from_axis_angle((1.0, 0.0, 1.0).into(), 60.0f32.to_radians())
                .build(), 
            EntityUniform::new(&layouts, device), 
            cube_mesh_0.clone(), 
            colored_pipeline.clone(), 
        ));

        let _yellow_cube = world.spawn((
            Color::Rgba { red: 0.8, green: 0.8, blue: 0.2, alpha: 0.3 }, 
            TransformBuilder::new()
                .set_translation((1.2, 1.0, 1.5).into())
                .build(),
            EntityUniform::new(&layouts, device), 
            cube_mesh_1.clone(), 
            transparent_pipeline.clone(), 
        ));

        let _magenta_cube = world.spawn((
            Color::Rgba { red: 0.8, green: 0.2, blue: 0.8, alpha: 0.8 }, 
            TransformBuilder::new()
                .set_translation((1.3, 1.1, 1.6).into())
                .build(), 
            EntityUniform::new(&layouts, device), 
            cube_mesh_1.clone(), 
            transparent_pipeline.clone(), 
        ));

        let _cyan_cube = world.spawn((
            Color::Rgba { red: 0.2, green: 0.8, blue: 0.8, alpha: 0.5 }, 
            TransformBuilder::new()
                .set_translation((1.1, 0.9, 1.4).into())
                .build(),
            EntityUniform::new(&layouts, device), 
            cube_mesh_1.clone(), 
            transparent_pipeline.clone(), 
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
            camera_force: 0.0, 
            touch_id: None, 
            touch_prev_x: 0.0, 
            layouts, 
            weighted_blended_oit: None, 
            graphics_pipelines: HashMap::from([
                (colored_pipeline.type_id(), Box::new(colored_pipeline) as Box<_>),
                (transparent_pipeline.type_id(), Box::new(transparent_pipeline) as Box<_>), 
                (composite_pipeline.type_id(), Box::new(composite_pipeline) as Box<_>),
            ]), 
        }
    }

    fn rotate_main_camera(&mut self, angle: f32) {
        let query = self.world.query_one_mut::<&mut Transform>(self.main_camera);
        if let Ok(transform) = query {
            let rotation = glam::Quat::from_rotation_y(angle);
            *transform.world_matrix_mut() = glam::Mat4::from_quat(rotation).mul_mat4(transform.world_matrix_ref());
        }
    }

    fn update_camera_uniform(&mut self, queue: &wgpu::Queue, entity: Entity) {
        let query = self.world.query_one::<(&Projection, &Transform, &CameraUniform)>(entity);
        if let Ok(mut query_one) = query {
            if let Some((projection, transform, uniform)) = query_one.get() {
                uniform.update(
                    queue, 
                    CameraUniformLayout { 
                        proj_view: projection.as_ref().mul_mat4(&transform.view_matrix()), 
                        position: transform.get_translation(), 
                        ..Default::default() 
                    }
                );
            };
        };
    }
}

impl GameScene for SampleScene {
    fn on_paused(
        &mut self, 
        _device: &wgpu::Device, 
        _queue: &wgpu::Queue
    ) {
        self.weighted_blended_oit = None;
    }

    fn on_resumed(
        &mut self, 
        window: &Window, 
        device: &wgpu::Device, 
        _queue: &wgpu::Queue
    ) {
        self.weighted_blended_oit = Some(WeightedBlendedOIT::new(
            window.inner_size().width, 
            window.inner_size().height, 
            &self.layouts, 
            device
        ))
    }

    fn on_resized(
        &mut self, 
        window: &Window, 
        device: &wgpu::Device, 
        _queue: &wgpu::Queue
    ) {
        self.weighted_blended_oit = Some(WeightedBlendedOIT::new(
            window.inner_size().width, 
            window.inner_size().height, 
            &self.layouts, 
            device
        ))
    }

    fn on_touch_event(
        &mut self, 
        touch: Touch, 
        _device: &wgpu::Device, 
        _queue: &wgpu::Queue
    ) {
        match touch.phase {
            TouchPhase::Started if self.touch_id.is_none() => {
                self.touch_id = Some(touch.id);
                self.touch_prev_x = touch.location.x as f32;
            },
            TouchPhase::Moved => if let Some(touch_id) = self.touch_id {
                if touch_id == touch.id {
                    self.camera_force = (self.touch_prev_x - touch.location.x as f32).to_radians();
                    self.touch_prev_x = touch.location.x as f32;
                };
            },
            TouchPhase::Ended => if let Some(touch_id) = self.touch_id {
                if touch_id == touch.id {
                    self.touch_id = None;
                    self.touch_prev_x = 0.0;
                    self.camera_force = 0.0;
                }
            },
            _ => { /* empty */ }
        };
    }

    fn on_keyboard_event(
        &mut self, 
        event: KeyEvent, 
        _device: &wgpu::Device, 
        _queue: &wgpu::Queue
    ) {
        if let PhysicalKey::Code(code) = &event.physical_key {
            if KeyCode::ArrowLeft == *code && !event.repeat && event.state.is_pressed() {
                self.camera_force -= 180.0f32.to_radians();
            } else if KeyCode::ArrowRight == *code && !event.repeat && event.state.is_pressed() {
                self.camera_force += 180.0f32.to_radians();
            } else if KeyCode::ArrowLeft == *code && !event.repeat && !event.state.is_pressed() {
                self.camera_force += 180.0f32.to_radians();
            } else if KeyCode::ArrowRight == *code && !event.repeat && !event.state.is_pressed() {
                self.camera_force -= 180.0f32.to_radians();
            }
        };
    }

    fn on_update(
        &mut self, 
        elapsed_time_sec: f32, 
        _device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) {
        self.rotate_main_camera(self.camera_force * elapsed_time_sec);
        self.update_camera_uniform(queue, self.main_camera);
    }

    fn on_draw(
        &self, 
        render_target_view: &wgpu::TextureView, 
        depth_stencil_view: &wgpu::TextureView, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) {
        let weighted_blended_oit = self.weighted_blended_oit.as_ref().unwrap();

        let mut query = self.world.query_one::<&CameraUniform>(self.main_camera).unwrap();
        let main_camera = query.get().unwrap();

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        let pipeline = self.graphics_pipelines.get(&TypeId::of::<ColoredPipeline>())
            .expect("ColoredPipeline not found!");
        let mut query = self.world.query::<(&ModelMesh, &EntityUniform)>().with::<&ColoredPipeline>();
        let mut model_mesh_map: HashMap<ModelMesh, Vec<EntityUniform>> = HashMap::new();
        for (_id, (model_mesh, uniform)) in query.iter() {
            model_mesh_map.entry(model_mesh.clone())
                .and_modify(|vec| vec.push(uniform.clone()))
                .or_insert(vec![uniform.clone()]);
        }

        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(ColoredPipeline)"), 
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

            pipeline.bind(&mut rpass);
            main_camera.bind(&mut rpass, 0);
            for (model_mesh, entities) in model_mesh_map.iter() {
                model_mesh.bind(&mut rpass, 0);
                for entity in entities.iter() {
                    entity.bind(&mut rpass, 1);
                    model_mesh.draw(&mut rpass, 0..1);
                }
            }
        }

        
        let pipeline = self.graphics_pipelines.get(&TypeId::of::<TransparentPipeline>())
            .expect("ColoredPipeline not found!");
        let mut query = self.world.query::<(&ModelMesh, &EntityUniform)>().with::<&TransparentPipeline>();
        let mut model_mesh_map: HashMap<ModelMesh, Vec<EntityUniform>> = HashMap::new();
        for (_id, (model_mesh, uniform)) in query.iter() {
            model_mesh_map.entry(model_mesh.clone())
                .and_modify(|vec| vec.push(uniform.clone()))
                .or_insert(vec![uniform.clone()]);
        }

        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(Transparent)"), 
                    color_attachments: &[
                        Some(weighted_blended_oit.accumulation_attachment()),  
                        Some(weighted_blended_oit.revealage_attachment()),
                    ],
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment {
                            view: &depth_stencil_view, 
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Load, 
                                store: wgpu::StoreOp::Store,
                            }),
                            stencil_ops: None, 
                        }, 
                    ), 
                    timestamp_writes: None, 
                    occlusion_query_set: None, 
                }
            );

            pipeline.bind(&mut rpass);
            main_camera.bind(&mut rpass, 0);
            for (model_mesh, entities) in model_mesh_map.iter() {
                model_mesh.bind(&mut rpass, 0);
                for entity in entities.iter() {
                    entity.bind(&mut rpass, 1);
                    model_mesh.draw(&mut rpass, 0..1);
                }
            }
        }

        let pipeline = self.graphics_pipelines.get(&TypeId::of::<CompositePipeline>())
            .expect("ColoredPipeline not found!");

        {
            let mut rpass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("RenderPass(Composite)"), 
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment {
                            view: render_target_view, 
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Load, 
                                store: wgpu::StoreOp::Store, 
                            },
                            resolve_target: None,
                        }), 
                    ], 
                    depth_stencil_attachment: Some(
                        wgpu::RenderPassDepthStencilAttachment { 
                            view: depth_stencil_view, 
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Load, 
                                store: wgpu::StoreOp::Store, 
                            }), 
                            stencil_ops: None 
                        }, 
                    ), 
                    timestamp_writes: None,
                    occlusion_query_set: None, 
                }
            );

            pipeline.bind(&mut rpass);
            weighted_blended_oit.bind(&mut rpass, 0);
            rpass.draw(0..4, 0..1);
        }

        // (한국어) 명령어 대기열에 명령어 목록을 제출합니다. </br>
        // (English Translation) Submit a list of commands to the command queue. </br>
        queue.submit(Some(encoder.finish()));
    }
}
