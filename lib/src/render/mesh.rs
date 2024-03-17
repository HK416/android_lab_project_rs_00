use std::mem;
use std::hash;
use std::ops::Range;
use std::sync::Arc;
use bytemuck::Pod;
use bytemuck::Zeroable;

pub const VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: mem::size_of::<VertexInputLayout>() as wgpu::BufferAddress, 
    step_mode: wgpu::VertexStepMode::Vertex, 
    attributes: &[
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3, 
            offset: 0x00 as wgpu::BufferAddress, 
            shader_location: 0, 
        }, 
        wgpu::VertexAttribute {
            format: wgpu::VertexFormat::Float32x3, 
            offset: 0x0C as wgpu::BufferAddress, 
            shader_location: 1, 
        }, 
    ],
};



/// #### 한국어 </br>
/// 메쉬의 버텍스 입력 레이아웃 입니다. </br>
/// 
/// #### English (Translation) </br>
/// The vertex input layout for a mesh. </br>
/// 
#[repr(C)]
#[derive(Pod, Zeroable)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexInputLayout {
    pub position: glam::Vec3, 
    pub normal: glam::Vec3, 
}

impl Default for VertexInputLayout {
    #[inline]
    fn default() -> Self {
        Self { 
            position: glam::Vec3::ZERO, 
            normal: glam::Vec3::ZERO, 
        }
    }
}



/// #### 한국어 </br>
/// 3차원 모델 메쉬의 인덱스 버퍼입니다. </br>
/// 
/// #### English (Translation) </br>
/// The index buffer of the 3D model mesh. </br>
/// 
#[derive(Debug, Clone)]
pub struct IndexBuffer {
    num_indices: u32, 
    buffer: Arc<wgpu::Buffer>, 
    format: wgpu::IndexFormat,
}

impl IndexBuffer {
    pub fn from_uint16_indices(
        label: Option<&str>, 
        indices: &[u32], 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> Self {
        let label = format!("IndexBuffer({})", label.unwrap_or("Unknown"));

        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some(&label), 
                mapped_at_creation: false, 
                size: mem::size_of_val(indices) as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST, 
            }, 
        );
        queue.write_buffer(&buffer, 0, bytemuck::cast_slice(indices));

        return Self { 
            num_indices: indices.len() as u32, 
            buffer: buffer.into(), 
            format: wgpu::IndexFormat::Uint16 
        };
    }

    pub fn from_uint32_indices(
        label: Option<&str>, 
        indices: &[u32], 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> Self {
        let label = format!("IndexBuffer({})", label.unwrap_or("Unknown"));

        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some(&label), 
                mapped_at_creation: false, 
                size: mem::size_of_val(indices) as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST, 
            }, 
        );
        queue.write_buffer(&buffer, 0, bytemuck::cast_slice(indices));

        return Self { 
            num_indices: indices.len() as u32, 
            buffer: buffer.into(), 
            format: wgpu::IndexFormat::Uint32 
        };
    }

    #[inline]
    pub fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>) {
        rpass.set_index_buffer(self.buffer.slice(..), self.format);
    }

    #[inline]
    pub fn num_indices(&self) -> u32 {
        self.num_indices
    }
}

impl Eq for IndexBuffer { }

impl PartialEq<Self> for IndexBuffer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer.global_id().eq(&other.buffer.global_id())
    }
}

impl hash::Hash for IndexBuffer {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.buffer.global_id().hash(state)
    }
}



/// #### 한국어 </br>
/// 3차원 모델 메쉬의 버텍스 버퍼입니다. </br>
/// 
/// #### English (Translation) </br>
/// The vertex buffer of the 3D model mesh. </br>
/// 
#[derive(Debug, Clone)]
pub struct VertexBuffer {
    num_vertices: u32, 
    buffer: Arc<wgpu::Buffer>, 
}

impl VertexBuffer {
    pub fn from_vertices(
        label: Option<&str>, 
        vertices: &[VertexInputLayout], 
        device: &wgpu::Device, 
        queue: &wgpu::Queue
    ) -> Self {
        let label = format!("VertexBuffer({})", label.unwrap_or("Unknown"));

        let buffer = device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some(&label), 
                mapped_at_creation: false, 
                size: mem::size_of_val(vertices) as wgpu::BufferAddress, 
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST, 
            }, 
        );
        queue.write_buffer(&buffer, 0, bytemuck::cast_slice(vertices));

        return Self {
            num_vertices: vertices.len() as u32, 
            buffer: buffer.into(), 
        }
    }

    #[inline]
    pub fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, slot: u32) {
        rpass.set_vertex_buffer(slot, self.buffer.slice(..));
    }

    #[inline]
    pub fn num_vertices(&self) -> u32 {
        self.num_vertices
    }
}

impl Eq for VertexBuffer { }

impl PartialEq<Self> for VertexBuffer {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.buffer.global_id().eq(&other.buffer.global_id())
    }
}

impl hash::Hash for VertexBuffer {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.buffer.global_id().hash(state)
    }
}



/// #### 한국어 </br>
/// 3차원 모델 메쉬 입니다. </br>
/// 
/// #### English (Translation) </br>
/// The 3D model mesh. </br>
/// 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelMesh {
    vertex_buffer: VertexBuffer, 
    index_buffer: Option<IndexBuffer>, 
}

impl ModelMesh {
    #[inline]
    pub fn new(vertex_buffer: VertexBuffer) -> Self {
        Self { vertex_buffer, index_buffer: None }
    }

    #[inline]
    pub fn new_with_index_buffer(
        vertex_buffer: VertexBuffer, 
        index_buffer: IndexBuffer
    ) -> Self {
        Self { vertex_buffer, index_buffer: Some(index_buffer) }
    }

    #[inline]
    pub fn bind<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, slot: u32) {
        self.vertex_buffer.bind(rpass, slot);
        if let Some(index_buffer) = &self.index_buffer {
            index_buffer.bind(rpass);
        }
    }

    pub fn draw<'a>(&'a self, rpass: &mut wgpu::RenderPass<'a>, instances: Range<u32>) {
        if let Some(index_buffer) = &self.index_buffer {
            rpass.draw_indexed(0..index_buffer.num_indices(), 0, instances);
        } else {
            rpass.draw(0..self.vertex_buffer.num_vertices(), instances);
        }
    }
}



//-------------------------------------------------------------------------------------------



/// #### 한국어 </br>
/// 주어진 가로와 세로 크기의 3차원 평면 메쉬를 생성합니다. </br>
/// 
/// #### English (Translation) </br>
/// Creates a 3d plane mesh with a given width and height size. </br>
/// 
pub fn create_plane_mesh(
    w: f32, 
    h: f32, 
    device: &wgpu::Device, 
    queue: &wgpu::Queue
) -> ModelMesh {
    assert!(w > 0.0 && h > 0.0);

    let hw = 0.5 * w;
    let hh = 0.5 * h;
    let vertices = [
        VertexInputLayout { position: (-hw,  0.0, -hh).into(), normal: ( 0.0,  1.0,  0.0).into(), ..Default::default() },
        VertexInputLayout { position: (-hw,  0.0,  hh).into(), normal: ( 0.0,  1.0,  0.0).into(), ..Default::default() },
        VertexInputLayout { position: ( hw,  0.0, -hh).into(), normal: ( 0.0,  1.0,  0.0).into(), ..Default::default() },

        VertexInputLayout { position: ( hw,  0.0, -hh).into(), normal: ( 0.0,  1.0,  0.0).into(), ..Default::default() },
        VertexInputLayout { position: (-hw,  0.0,  hh).into(), normal: ( 0.0,  1.0,  0.0).into(), ..Default::default() },
        VertexInputLayout { position: ( hw,  0.0,  hh).into(), normal: ( 0.0,  1.0,  0.0).into(), ..Default::default() },
    ];

    let vertex_buffer = VertexBuffer::from_vertices(
        Some("PlaneMesh"), 
        &vertices, 
        device, 
        queue
    );

    return ModelMesh::new(vertex_buffer);
}
