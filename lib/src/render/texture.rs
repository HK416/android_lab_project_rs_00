use std::sync::Arc;
use std::any::TypeId;
use std::collections::HashMap;



/// #### 한국어 </br>
/// Weighted Blended OIT를 구현하기 위한 렌더 타겟 텍스처의 집합입니다. </br>
/// 
/// #### English (Translation) </br>
/// A set of render target textures for implementing Weighted Blended OIT. </br>
/// 
#[derive(Debug, Clone)]
pub struct WeightedBlendedOIT {
    accumulation: Arc<wgpu::TextureView>, 
    revealage: Arc<wgpu::TextureView>, 
    bind_group: Arc<wgpu::BindGroup>, 
}

impl WeightedBlendedOIT {
    pub fn new(
        width: u32, height: u32, 
        layouts: &HashMap<TypeId, wgpu::BindGroupLayout>, 
        device: &wgpu::Device
    ) -> Self {
        let sampler = device.create_sampler(
            &wgpu::SamplerDescriptor {
                label: Some("Sampler(WeightedBlendedOIT)"), 
                address_mode_u: wgpu::AddressMode::ClampToEdge, 
                address_mode_v: wgpu::AddressMode::ClampToEdge, 
                address_mode_w: wgpu::AddressMode::ClampToEdge, 
                mag_filter: wgpu::FilterMode::Linear, 
                min_filter: wgpu::FilterMode::Linear, 
                mipmap_filter: wgpu::FilterMode::Nearest, 
                ..Default::default()
            }
        );

        let accumulation = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Texture(Accumulation)"), 
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                format: wgpu::TextureFormat::Rgba16Float, 
                dimension: wgpu::TextureDimension::D2, 
                mip_level_count: 1, 
                sample_count: 1, 
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, 
                view_formats: &[], 
            }
        )
        .create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

        let revealage = device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some("Texture(Revealage)"), 
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 }, 
                format: wgpu::TextureFormat::R8Unorm, 
                dimension: wgpu::TextureDimension::D2, 
                mip_level_count: 1, 
                sample_count: 1, 
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING, 
                view_formats: &[], 
            }, 
        )
        .create_view(&wgpu::TextureViewDescriptor { ..Default::default() });

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(WeightedBlendedOIT)"), 
                layout: layouts.get(&TypeId::of::<WeightedBlendedOIT>())
                    .expect("WeightedBlendedOIT not found!"), 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0, 
                        resource: wgpu::BindingResource::TextureView(&accumulation), 
                    }, 
                    wgpu::BindGroupEntry {
                        binding: 1, 
                        resource: wgpu::BindingResource::Sampler(&sampler), 
                    }, 
                    wgpu::BindGroupEntry {
                        binding: 2, 
                        resource: wgpu::BindingResource::TextureView(&revealage), 
                    }, 
                    wgpu::BindGroupEntry {
                        binding: 3, 
                        resource: wgpu::BindingResource::Sampler(&sampler), 
                    }, 
                ], 
            }, 
        );

        return Self { 
            accumulation: accumulation.into(), 
            revealage: revealage.into(), 
            bind_group: bind_group.into() 
        };
    }

    pub fn accumulation_attachment(&self) -> wgpu::RenderPassColorAttachment {
        wgpu::RenderPassColorAttachment {
            view: &self.accumulation, 
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }), 
                store: wgpu::StoreOp::Store,
            },
            resolve_target: None,
        }
    }

    pub fn revealage_attachment(&self) -> wgpu::RenderPassColorAttachment {
        wgpu::RenderPassColorAttachment {
            view: &self.revealage, 
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }), 
                store: wgpu::StoreOp::Store, 
            },
            resolve_target: None,
        }
    }

    #[inline]
    pub fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, index: u32) {
        rpass.set_bind_group(index, &self.bind_group, &[]);
    }

    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("BindGroupLayout(WeightedBlendedOIT)"), 
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0, 
                        visibility: wgpu::ShaderStages::FRAGMENT, 
                        ty: wgpu::BindingType::Texture { 
                            sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                            view_dimension: wgpu::TextureViewDimension::D2, 
                            multisampled: false, 
                        }, 
                        count: None, 
                    }, 
                    wgpu::BindGroupLayoutEntry {
                        binding: 1, 
                        visibility: wgpu::ShaderStages::FRAGMENT, 
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering
                        ), 
                        count: None,
                    }, 
                    wgpu::BindGroupLayoutEntry {
                        binding: 2, 
                        visibility: wgpu::ShaderStages::FRAGMENT, 
                        ty: wgpu::BindingType::Texture { 
                            sample_type: wgpu::TextureSampleType::Float { filterable: true }, 
                            view_dimension: wgpu::TextureViewDimension::D2, 
                            multisampled: false, 
                        }, 
                        count: None, 
                    }, 
                    wgpu::BindGroupLayoutEntry {
                        binding: 3, 
                        visibility: wgpu::ShaderStages::FRAGMENT, 
                        ty: wgpu::BindingType::Sampler(
                            wgpu::SamplerBindingType::Filtering
                        ), 
                        count: None,
                    }, 
                ]
            }
        )
    }
}

impl Eq for WeightedBlendedOIT { }

impl PartialEq<Self> for WeightedBlendedOIT {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.accumulation.global_id().eq(&other.accumulation.global_id())
        & self.revealage.global_id().eq(&other.revealage.global_id())
        & self.bind_group.global_id().eq(&other.bind_group.global_id())
    }
}
