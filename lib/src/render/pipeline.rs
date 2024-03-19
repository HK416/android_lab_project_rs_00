use std::hash;
use std::sync::Arc;
use std::num::NonZeroU32;



#[derive(Debug, Clone)]
pub struct GraphicsPipeline {
    inner: Arc<wgpu::RenderPipeline>, 
}

impl GraphicsPipeline {
    pub fn new(
        label: Option<&str>, 
        device: &wgpu::Device, 
        bind_group_layouts: &[&wgpu::BindGroupLayout], 
        vertex: wgpu::VertexState, 
        fragment: Option<wgpu::FragmentState>, 
        primitive: wgpu::PrimitiveState, 
        depth_stencil: Option<wgpu::DepthStencilState>, 
        multisample: wgpu::MultisampleState,
        multiview: Option<NonZeroU32>, 
    ) -> Self {
        let pipeline_layouts = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("PipelineLayout({})", label.unwrap_or("Unknown"))), 
                bind_group_layouts, 
                push_constant_ranges: &[], 
            }, 
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some(&format!("RenderPipeline({})", label.unwrap_or("Unknown"))), 
                layout: Some(&pipeline_layouts), 
                vertex, 
                primitive, 
                depth_stencil, 
                multisample, 
                fragment, 
                multiview
            }
        );

        return Self { inner: pipeline.into() };
    }

    #[inline]
    pub fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_pipeline(&self.inner)
    }
}

impl Eq for GraphicsPipeline { }

impl PartialEq<Self> for GraphicsPipeline {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.global_id().eq(&other.inner.global_id())
    }
}

impl hash::Hash for GraphicsPipeline {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.global_id().hash(state)
    }
}



//-------------------------------------------------------------------------------------------
use std::any::TypeId;
use std::collections::HashMap;

use crate::app::RENDER_TARGET_FORMAT;
use crate::render::mesh::VERTEX_BUFFER_LAYOUT;
use crate::render::texture::WeightedBlendedOIT;
use crate::render::uniform::CameraUniformLayout;
use crate::render::uniform::EntityUniformLayout;



/// #### 한국어 </br>
/// 주어진 엔티티의 색상을 출력하는 그래픽스 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a graphics pipeline that outputs the color of a given entity. </br>
/// 
pub fn create_colored_pipeline(
    layouts: &HashMap<TypeId, wgpu::BindGroupLayout>, 
    device: &wgpu::Device, 
) -> GraphicsPipeline {
    let bind_group_layouts = &[
        layouts.get(&TypeId::of::<CameraUniformLayout>())
            .expect("CameraUniformLayout not found!"), 
        layouts.get(&TypeId::of::<EntityUniformLayout>())
            .expect("EntityUniformLayout not found!"),
    ];

    let vertex_shader = device.create_shader_module(
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/color.vs.spv"))
    );

    let fragment_shader = device.create_shader_module(
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/color.fs.spv"))
    );

    return GraphicsPipeline::new(
        Some("ColoredPipeline"), 
        device, 
        bind_group_layouts, 
        wgpu::VertexState {
            module: &vertex_shader, 
            entry_point: "main", 
            buffers: &[VERTEX_BUFFER_LAYOUT], 
        }, 
        Some(wgpu::FragmentState {
            module: &fragment_shader, 
            entry_point: "main", 
            targets: &[Some(wgpu::ColorTargetState {
                format: RENDER_TARGET_FORMAT,  
                blend: None, 
                write_mask: wgpu::ColorWrites::ALL, 
            })],
        }), 
        wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, 
            polygon_mode: wgpu::PolygonMode::Fill, 
            front_face: wgpu::FrontFace::Ccw, 
            cull_mode: Some(wgpu::Face::Back), 
            ..Default::default()
        }, 
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float, 
            depth_write_enabled: true, 
            depth_compare: wgpu::CompareFunction::Less, 
            stencil: wgpu::StencilState::default(), 
            bias: wgpu::DepthBiasState::default(), 
        }), 
        wgpu::MultisampleState::default(), 
        None
    )
}

/// #### 한국어 </br>
/// 투명한 물체의 데이터를 렌더타겟에 저장하는 그래픽스 파이프라인을 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Create a graphics pipeline that stores data about transparent objects in a render target. </br>
/// 
pub fn create_transparent_pipeline(
    layouts: &HashMap<TypeId, wgpu::BindGroupLayout>, 
    device: &wgpu::Device
) -> GraphicsPipeline {
    let bind_group_layouts = &[
        layouts.get(&TypeId::of::<CameraUniformLayout>())
            .expect("CameraUniformLayout not found!"), 
        layouts.get(&TypeId::of::<EntityUniformLayout>())
            .expect("EntityUniformLayout not found!"),
    ];

    let vertex_shader = device.create_shader_module(
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/transparent.vs.spv"))
    );

    let fragment_shader = device.create_shader_module(
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/transparent.fs.spv"))
    );
    
    return GraphicsPipeline::new(
        Some("Transparent"), 
        device, 
        bind_group_layouts, 
        wgpu::VertexState {
            module: &vertex_shader, 
            entry_point: "main", 
            buffers: &[VERTEX_BUFFER_LAYOUT], 
        }, 
        Some(wgpu::FragmentState {
            module: &fragment_shader, 
            entry_point: "main", 
            targets: &[
                // (한국어) 
                // 첫 번째 렌더 타겟: (RGB * 가중치, Alpha * 가중치)를 RGBA로 저장하하는 누적 값.
                // 최소 `Rgba16Float`의 정밀도를 가져야 한다.
                // 
                // (English Translation)
                // First Render Target: Accumulated value (RGB * Weight, Alpha * Weight) stored as RGBA.
                // It must have a precision of at least `Rgba16Float`.
                //
                Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba16Float, 
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One, 
                            dst_factor: wgpu::BlendFactor::One, 
                            operation: wgpu::BlendOperation::Add, 
                        }, 
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One, 
                            dst_factor: wgpu::BlendFactor::One, 
                            operation: wgpu::BlendOperation::Add, 
                        }, 
                    }), 
                    write_mask: wgpu::ColorWrites::ALL, 
                }), 
                // (한국어)
                // 두 번째 렌더 타겟: 이전의 색이 얼마만큼 노출이 될 수 있는지에 대한 노출 값.
                // 최소 `R8`의 정밀도를 가져야 한다.
                // 
                // (English Translation)
                // Second Render Target: Revealage value of how much of the previous color can be exposed. 
                // It must have a precision of at least `R8`.
                // 
                Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::R8Unorm, 
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::Zero, 
                            dst_factor: wgpu::BlendFactor::OneMinusSrc, 
                            operation: wgpu::BlendOperation::Add, 
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::Zero, 
                            dst_factor: wgpu::BlendFactor::OneMinusSrc, 
                            operation: wgpu::BlendOperation::Add, 
                        }
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }),
            ]
        }), 
        wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, 
            polygon_mode: wgpu::PolygonMode::Fill, 
            front_face: wgpu::FrontFace::Ccw, 
            cull_mode: Some(wgpu::Face::Back), 
            ..Default::default()
        }, 
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float, 
            depth_compare: wgpu::CompareFunction::Less, 
            depth_write_enabled: false, 
            stencil: wgpu::StencilState::default(), 
            bias: wgpu::DepthBiasState::default(), 
        }), 
        wgpu::MultisampleState::default(), 
        None
    );
}

/// #### 한국어 </br>
/// 투명한 물체의 데이터를 기반으로 불투명한 물체와 섞는 그래픽스 파이프라인 입니다. </br>
/// 
/// #### English (Translation) </br>
/// A graphics pipeline that takes data from transparent objects and blends it with opaque objects. </br>
/// 
pub fn create_composite_pipeline(
    layouts: &HashMap<TypeId, wgpu::BindGroupLayout>, 
    device: &wgpu::Device
) -> GraphicsPipeline {
    let bind_group_layouts = &[
        layouts.get(&TypeId::of::<WeightedBlendedOIT>())
            .expect("WeightedBlendedOIT not found!"), 
    ];

    let vertex_shader = device.create_shader_module(
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/composite.vs.spv"))
    );

    let fragment_shader = device.create_shader_module(
        wgpu::include_spirv!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/composite.fs.spv"))
    );


    return GraphicsPipeline::new(
        Some("Composite"), 
        device, 
        bind_group_layouts, 
        wgpu::VertexState {
            module: &vertex_shader, 
            entry_point: "main", 
            buffers: &[], 
        }, 
        Some(wgpu::FragmentState {
            module: &fragment_shader, 
            entry_point: "main", 
            targets: &[
                Some(wgpu::ColorTargetState {
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING), 
                    format: RENDER_TARGET_FORMAT, 
                    write_mask: wgpu::ColorWrites::ALL, 
                }), 
            ], 
        }), 
        wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleStrip, 
            strip_index_format: Some(wgpu::IndexFormat::Uint16), 
            polygon_mode: wgpu::PolygonMode::Fill, 
            front_face: wgpu::FrontFace::Cw, 
            cull_mode: Some(wgpu::Face::Back), 
            ..Default::default()
        }, 
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float, 
            depth_compare: wgpu::CompareFunction::Less, 
            depth_write_enabled: true, 
            stencil: wgpu::StencilState::default(), 
            bias: wgpu::DepthBiasState::default(), 
        }), 
        wgpu::MultisampleState::default(), 
        None
    );
}


