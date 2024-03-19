use std::mem;
use std::sync::Arc;
use std::any::TypeId;
use std::collections::HashMap;
use bytemuck::Pod;
use bytemuck::Zeroable;



/// #### 한국어 </br>
/// 카메라 유니폼 데이터의 레이아웃 입니다. </br>
/// 
/// #### English (Translation) </br>
/// The layout of the camera uniform data. </br>
/// 
#[repr(C, align(16))]
#[derive(Pod, Zeroable)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraUniformLayout {
    pub proj_view: glam::Mat4, 
    pub position: glam::Vec3, 
    pub _padding0: [u8; 4], 
}

impl CameraUniformLayout {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("BindGroupLayout(CameraUniformLayout)"), 
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0, 
                        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT, 
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Uniform, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        }, 
                        count: None, 
                    },
                ], 
            }, 
        )
    }
}

impl Default for CameraUniformLayout {
    #[inline]
    fn default() -> Self {
        Self {
            proj_view: glam::Mat4::IDENTITY, 
            position: glam::Vec3::ZERO, 
            _padding0: [0; 4], 
        }
    }
}



/// #### 한국어 </br>
/// 카메라 유니폼 데이터 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Camera uniform data. </br>
/// 
#[derive(Debug, Clone)]
pub struct CameraUniform {
    buffer: Arc<wgpu::Buffer>, 
    bind_group: Arc<wgpu::BindGroup>, 
}

impl CameraUniform {
    pub fn new(
        layouts: &HashMap<TypeId, wgpu::BindGroupLayout>, 
        device: &wgpu::Device
    ) -> Self {
        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("UniformBuffer(CameraUniformLayout)"), 
                mapped_at_creation: false, 
                size: mem::size_of::<CameraUniformLayout>() as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
            }, 
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(CameraUniform)"), 
                layout: layouts.get(&TypeId::of::<CameraUniformLayout>())
                    .expect("CameraUniformLayout not found!"), 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0, 
                        resource: wgpu::BindingResource::Buffer(
                            buffer.as_entire_buffer_binding()
                        ),
                    }, 
                ], 
            }, 
        );

        Self { 
            buffer: buffer.into(), 
            bind_group: bind_group.into() 
        }
    }

    #[inline]
    pub fn update(&self, queue: &wgpu::Queue, data: CameraUniformLayout) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&data));
    }

    #[inline]
    pub fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, index: u32) {
        rpass.set_bind_group(index, &self.bind_group, &[]);
    }
}

impl Eq for CameraUniform { }

impl PartialEq<Self> for CameraUniform {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer.global_id().eq(&other.buffer.global_id())
        && self.bind_group.global_id().eq(&other.bind_group.global_id())
    }
}



/// #### 한국어 </br>
/// 엔티티 유니폼 데이터의 레이아웃 입니다. </br>
/// 
/// #### English (Translation) </br>
/// The layout of the entity uniform data. </br>
/// 
#[repr(C, align(16))]
#[derive(Pod, Zeroable)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityUniformLayout {
    pub world: glam::Mat4, 
    pub color: glam::Vec4, 
}

impl EntityUniformLayout {
    pub fn layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("BindGroupLayout(EntityUniformLayout)"), 
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0, 
                        visibility: wgpu::ShaderStages::VERTEX, 
                        ty: wgpu::BindingType::Buffer { 
                            ty: wgpu::BufferBindingType::Uniform, 
                            has_dynamic_offset: false, 
                            min_binding_size: None 
                        }, 
                        count: None, 
                    }, 
                ], 
            }, 
        )
    }
}

impl Default for EntityUniformLayout {
    #[inline]
    fn default() -> Self {
        Self { 
            world: glam::Mat4::IDENTITY, 
            color: glam::Vec4::ONE, 
        }
    }
}



/// #### 한국어 </br>
/// 엔티티 유니폼 데이터 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Entity uniform data. </br>
/// 
#[derive(Debug, Clone)]
pub struct EntityUniform {
    buffer: Arc<wgpu::Buffer>, 
    bind_group: Arc<wgpu::BindGroup>, 
}

impl EntityUniform {
    pub fn new(
        layouts: &HashMap<TypeId, wgpu::BindGroupLayout>, 
        device: &wgpu::Device
    ) -> Self {
        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("UniformBuffer(EntityUniformLayout)"), 
                mapped_at_creation: false, 
                size: mem::size_of::<EntityUniformLayout>() as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST, 
            }, 
        );

        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("BindGroup(EntityUniform)"), 
                layout: layouts.get(&TypeId::of::<EntityUniformLayout>())
                    .expect("EntityUniformLayout not found!"), 
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0, 
                        resource: wgpu::BindingResource::Buffer(
                            buffer.as_entire_buffer_binding()
                        ), 
                    }, 
                ], 
            }, 
        );

        Self { 
            buffer: buffer.into(), 
            bind_group: bind_group.into() 
        }
    }

    #[inline]
    pub fn update(&self, queue: &wgpu::Queue, data: EntityUniformLayout) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&data));
    }

    #[inline]
    pub fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, index: u32) {
        rpass.set_bind_group(index, &self.bind_group, &[]);
    }
}

impl Eq for EntityUniform { }

impl PartialEq<Self> for EntityUniform {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer.global_id().eq(&other.buffer.global_id())
        && self.bind_group.global_id().eq(&other.bind_group.global_id())
    }
}
