#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Projection {
    inner: glam::Mat4, 
}

impl Projection {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn perspective(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        Self { 
            inner: glam::Mat4::perspective_rh(
                fov_y_radians, 
                aspect_ratio, 
                z_near, 
                z_far
            )
        }
    }

    #[inline]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self {
            inner: glam::Mat4::orthographic_rh(
                left, 
                right, 
                bottom, 
                top, 
                near, 
                far
            )
        }
    }
}

impl AsRef<glam::Mat4> for Projection {
    #[inline]
    fn as_ref(&self) -> &glam::Mat4 {
        &self.inner
    }
}

impl AsMut<glam::Mat4> for Projection {
    #[inline]
    fn as_mut(&mut self) -> &mut glam::Mat4 {
        &mut self.inner
    }
}

impl Default for Projection {
    #[inline]
    fn default() -> Self {
        Self { inner: glam::Mat4::IDENTITY }
    }
}