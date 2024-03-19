/// #### 한국어 </br>
/// 원근 투영 행렬 데이터입니다. </br>
/// 
/// #### English (Translation) </br>
/// Perspective projection matrix data. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Perspective {
    transform: glam::Mat4, 
    fov_y_radians: f32,
    aspect_ratio: f32, 
    z_near: f32, 
    z_far: f32, 
}

impl AsRef<glam::Mat4> for Perspective {
    #[inline]
    fn as_ref(&self) -> &glam::Mat4 {
        &self.transform
    }
}

impl Into<PerspectiveBuilder> for Perspective {
    #[inline]
    fn into(self) -> PerspectiveBuilder {
        PerspectiveBuilder {
            fov_y_radians: self.fov_y_radians, 
            aspect_ratio: self.aspect_ratio, 
            z_near: self.z_near, 
            z_far: self.z_far, 
        }
    }
}

impl Default for Perspective {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: glam::Mat4::perspective_rh(
                60.0f32.to_radians(), 
                1.0, 
                0.0001, 
                1000.0
            ), 
            fov_y_radians: 60.0f32.to_radians(), 
            aspect_ratio: 1.0, 
            z_near: 0.0001, 
            z_far: 1000.0 
        }
    }
}



/// #### 한국어 </br>
/// 원근 투영 행렬을 생성하는 빌더입니다. </br>
/// 
/// #### English (Translation) </br>
/// A builder that generates perspective projection matrices. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerspectiveBuilder {
    pub fov_y_radians: f32, 
    pub aspect_ratio: f32, 
    pub z_near: f32, 
    pub z_far: f32, 
}

#[allow(dead_code)]
impl PerspectiveBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn set_fov_y_radians(mut self, fov_y_radians: f32) -> Self {
        self.fov_y_radians = fov_y_radians;
        self
    }

    #[inline]
    pub fn set_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    #[inline]
    pub fn set_z_near(mut self, z_near: f32) -> Self {
        self.z_near = z_near;
        self
    }

    #[inline]
    pub fn set_z_far(mut self, z_far: f32) -> Self {
        self.z_far = z_far;
        self
    }

    #[inline]
    pub fn build(self) -> Perspective {
        Perspective { 
            transform: glam::Mat4::perspective_rh(
                self.fov_y_radians, 
                self.aspect_ratio, 
                self.z_near, 
                self.z_far
            ), 
            fov_y_radians: self.fov_y_radians, 
            aspect_ratio: self.aspect_ratio, 
            z_near: self.z_near, 
            z_far: self.z_far 
        }
    }
}

impl Default for PerspectiveBuilder {
    #[inline]
    fn default() -> Self {
        Self { 
            fov_y_radians: 60.0f32.to_radians(), 
            aspect_ratio: 1.0, 
            z_near: 0.0001, 
            z_far: 1000.0 
        }
    }
}



/// #### 한국어 </br>
/// 정사영 투영 행렬 데이터입니다. </br>
/// 
/// #### English (Translation) </br>
/// Orthographic projection matrix data. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Orthographic {
    transform: glam::Mat4, 
    left: f32, 
    right: f32, 
    bottom: f32, 
    top: f32, 
    z_near: f32, 
    z_far: f32, 
}

impl AsRef<glam::Mat4> for Orthographic {
    #[inline]
    fn as_ref(&self) -> &glam::Mat4 {
        &self.transform
    }
}

impl Into<OrthographicBuilder> for Orthographic {
    #[inline]
    fn into(self) -> OrthographicBuilder {
        OrthographicBuilder {
            left: self.left, 
            right: self.right, 
            bottom: self.bottom, 
            top: self.top, 
            z_near: self.z_near, 
            z_far: self.z_far, 
        }
    }
}

impl Default for Orthographic {
    #[inline]
    fn default() -> Self {
        Self { 
            transform: glam::Mat4::orthographic_rh(
                -1.0, 
                1.0, 
                -1.0, 
                1.0, 
                0.0001, 
                1000.0
            ), 
            left: -1.0, 
            right: 1.0, 
            bottom: -1.0, 
            top: 1.0, 
            z_near: 0.0001, 
            z_far: 1000.0 
        }
    }
}



/// #### 한국어 </br>
/// 정사영 투영 행렬을 생성하는 빌더입니다. </br>
/// 
/// #### English (Translation) </br>
/// A builder that generates orthographic projection matrices. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrthographicBuilder {
    pub left: f32, 
    pub right: f32, 
    pub bottom: f32, 
    pub top: f32, 
    pub z_near: f32, 
    pub z_far: f32, 
}

#[allow(dead_code)]
impl OrthographicBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn set_left(mut self, left: f32) -> Self {
        self.left = left;
        self
    }

    #[inline]
    pub fn set_right(mut self, right: f32) -> Self {
        self.right = right;
        self
    }

    #[inline]
    pub fn set_bottom(mut self, bottom: f32) -> Self {
        self.bottom = bottom;
        self
    }

    #[inline]
    pub fn set_top(mut self, top: f32) -> Self {
        self.top = top;
        self
    }

    #[inline]
    pub fn set_z_near(mut self, z_near: f32) -> Self {
        self.z_near = z_near;
        self
    }

    #[inline]
    pub fn set_z_far(mut self, z_far: f32) -> Self {
        self.z_far = z_far;
        self
    }

    #[inline]
    pub fn build(self) -> Orthographic {
        Orthographic { 
            transform: glam::Mat4::orthographic_rh(
                self.left, 
                self.right, 
                self.bottom, 
                self.top, 
                self.z_near, 
                self.z_far
            ), 
            left: self.left, 
            right: self.right, 
            bottom: self.bottom, 
            top: self.top, 
            z_near: self.z_near, 
            z_far: self.z_far 
        }
    }
}

impl Default for OrthographicBuilder {
    #[inline]
    fn default() -> Self {
        Self { 
            left: -1.0, 
            right: 1.0, 
            bottom: -1.0, 
            top: 1.0, 
            z_near: 0.0001, 
            z_far: 1000.0 
        }
    }
}



#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Projection {
    Perspective(Perspective), 
    Orthographic(Orthographic), 
}

impl AsRef<glam::Mat4> for Projection {
    #[inline]
    fn as_ref(&self) -> &glam::Mat4 {
        match self {
            Self::Perspective(proj) => proj.as_ref(), 
            Self::Orthographic(ortho) => ortho.as_ref(), 
        }
    }
}
