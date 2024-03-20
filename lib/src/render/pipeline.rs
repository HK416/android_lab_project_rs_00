use std::hash;
use std::sync::Arc;
use std::any::TypeId;
use std::collections::HashMap;

use crate::app::RENDER_TARGET_FORMAT;
use crate::render::mesh::VERTEX_BUFFER_LAYOUT;
use crate::render::texture::WeightedBlendedOIT;
use crate::render::uniform::CameraUniformLayout;
use crate::render::uniform::EntityUniformLayout;



pub trait GraphicsPipeline {
    fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>);
}


/// #### 한국어 </br>
/// 단일 색상의 오브젝트를 그리는 그래픽스 파이프라인입니다. </br>
/// 
/// #### English (Translation) </br>
/// A graphics pipeline that draws object with a single color. </br>
/// 
#[derive(Debug, Clone)]
pub struct ColoredPipeline {
    inner: Arc<wgpu::RenderPipeline>
}

impl ColoredPipeline {
    pub fn new(
        device: &wgpu::Device, 
        layouts: &HashMap<TypeId, wgpu::BindGroupLayout>
    ) -> Self {
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

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("PipelineLayout(ColoredPipeline)"), 
                bind_group_layouts, 
                push_constant_ranges: &[],
            },
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("RenderPipeline(ColoredPipeline)"), 
                layout: Some(&pipeline_layout), 
                vertex: wgpu::VertexState {
                    module: &vertex_shader, 
                    entry_point: "main", 
                    buffers: &[VERTEX_BUFFER_LAYOUT], 
                }, 
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, 
                    front_face: wgpu::FrontFace::Ccw, 
                    cull_mode: Some(wgpu::Face::Back), 
                    polygon_mode: wgpu::PolygonMode::Fill, 
                    ..Default::default()
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float, 
                    depth_write_enabled: true, 
                    depth_compare: wgpu::CompareFunction::Less, 
                    stencil: wgpu::StencilState::default(), 
                    bias: wgpu::DepthBiasState::default(), 
                }), 
                multisample: wgpu::MultisampleState::default(), 
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_shader, 
                    entry_point: "main", 
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: RENDER_TARGET_FORMAT, 
                            blend: None, 
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
                }),
                multiview: None,
            },
        );

        return Self {
            inner: pipeline.into(), 
        };
    }
}

impl GraphicsPipeline for ColoredPipeline {
    #[inline]
    fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_pipeline(&self.inner)
    }
}

impl Eq for ColoredPipeline { }

impl PartialEq<Self> for ColoredPipeline {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.global_id().eq(&other.inner.global_id())
    }
}

impl hash::Hash for ColoredPipeline {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.global_id().hash(state)
    }
}



/// #### 한국어 </br>
/// 투명한 오브젝트의 누적값과 노출값을 계산하여 저장하는 그래픽스 파이프라인 입니다. </br>
/// 
/// #### English (Translation) </br>
/// A graphics pipeline that computes and stores the accumulated and revealed values of transparent objects. </br>
/// 
#[derive(Debug, Clone)]
pub struct TransparentPipeline {
    inner: Arc<wgpu::RenderPipeline>, 
}

impl TransparentPipeline {
    pub fn new(
        device: &wgpu::Device, 
        layouts: &HashMap<TypeId, wgpu::BindGroupLayout>
    ) -> Self {
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

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("PipelineLayout(TransparentPipeline)"), 
                bind_group_layouts, 
                push_constant_ranges: &[], 
            }, 
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("RenderPipeline(TransparentPipeline)"), 
                layout: Some(&pipeline_layout), 
                vertex: wgpu::VertexState {
                    module: &vertex_shader, 
                    entry_point: "main", 
                    buffers: &[VERTEX_BUFFER_LAYOUT], 
                }, 
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, 
                    front_face: wgpu::FrontFace::Ccw, 
                    cull_mode: Some(wgpu::Face::Back), 
                    polygon_mode: wgpu::PolygonMode::Fill, 
                    ..Default::default()
                }, 
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float, 
                    depth_write_enabled: false, 
                    depth_compare: wgpu::CompareFunction::Less, 
                    stencil: wgpu::StencilState::default(), 
                    bias: wgpu::DepthBiasState::default(), 
                }), 
                multisample: wgpu::MultisampleState::default(), 
                fragment: Some(wgpu::FragmentState {
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
                    ],
                }),
                multiview: None,
            },
        );

        return Self {
            inner: pipeline.into(), 
        };
    }
}

impl GraphicsPipeline for TransparentPipeline {
    #[inline]
    fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_pipeline(&self.inner)
    }
}

impl Eq for TransparentPipeline { }

impl PartialEq<Self> for TransparentPipeline {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.global_id().eq(&other.inner.global_id())
    }
}

impl hash::Hash for TransparentPipeline {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.global_id().hash(state)
    }
}



/// #### 한국어 </br>
/// 불투명한 물체와 투명한 물체를 혼합하는 그래픽스 파이프라인 입니다. </br>
/// 
/// #### English (Translation) </br>
/// A graphics pipeline that blends opaque and transparent objects. </br>
/// 
#[derive(Debug, Clone)]
pub struct CompositePipeline {
    inner: Arc<wgpu::RenderPipeline>, 
}

impl CompositePipeline {
    pub fn new(
        device: &wgpu::Device, 
        layouts: &HashMap<TypeId, wgpu::BindGroupLayout>, 
    ) -> Self {
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

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("PipelineLayout(CompositePipeline)"), 
                bind_group_layouts, 
                push_constant_ranges: &[],
            },
        );

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: Some("RenderPipeline(CompositePipeline)"), 
                layout: Some(&pipeline_layout), 
                vertex: wgpu::VertexState {
                    module: &vertex_shader, 
                    entry_point: "main", 
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleStrip, 
                    strip_index_format: Some(wgpu::IndexFormat::Uint16), 
                    front_face: wgpu::FrontFace::Cw, 
                    cull_mode: Some(wgpu::Face::Back), 
                    polygon_mode: wgpu::PolygonMode::Fill, 
                    ..Default::default()
                }, 
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float, 
                    depth_write_enabled: true, 
                    depth_compare: wgpu::CompareFunction::Less, 
                    stencil: wgpu::StencilState::default(), 
                    bias: wgpu::DepthBiasState::default(), 
                }), 
                multisample: wgpu::MultisampleState::default(), 
                fragment: Some(wgpu::FragmentState {
                    module: &fragment_shader, 
                    entry_point: "main", 
                    targets: &[
                        Some(wgpu::ColorTargetState {
                            format: RENDER_TARGET_FORMAT, 
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING), 
                            write_mask: wgpu::ColorWrites::ALL,
                        }),
                    ],
                }),
                multiview: None,
            },
        );

        return Self {
            inner: pipeline.into(), 
        };
    }
}

impl GraphicsPipeline for CompositePipeline {
    #[inline]
    fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_pipeline(&self.inner)
    }
}

impl Eq for CompositePipeline { }

impl PartialEq<Self> for CompositePipeline {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner.global_id().eq(&other.inner.global_id())
    }
}

impl hash::Hash for CompositePipeline {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.inner.global_id().hash(state)
    }
}
