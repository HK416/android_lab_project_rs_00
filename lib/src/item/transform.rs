use glam::Vec4Swizzles;



/// #### 한국어 </br>
/// 게임 월드에서의 크기, 회전, 위치를 데이터 입니다. </br>
/// 
/// #### English (Translation) </br>
/// Data about scale, rotation, and translation in the game world. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    inner: glam::Mat4, 
}

impl Transform {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn get_translation(&self) -> glam::Vec3 {
        self.inner.w_axis.xyz()
    }

    #[inline]
    pub fn set_translation(&mut self, translation: glam::Vec3) {
        self.inner.w_axis.x = translation.x;
        self.inner.w_axis.y = translation.y;
        self.inner.w_axis.z = translation.z;
    }

    #[inline]
    pub fn get_rotation(&self) -> glam::Quat {
        glam::Quat::from_mat4(&self.inner).normalize()
    }

    #[inline]
    pub fn set_rotation(&mut self, rotation: glam::Quat) {
        let mat = glam::Mat3::from_quat(rotation.normalize());
        self.inner.x_axis.x = mat.x_axis.x;
        self.inner.x_axis.y = mat.x_axis.y;
        self.inner.x_axis.z = mat.x_axis.z;
        
        self.inner.y_axis.x = mat.y_axis.x;
        self.inner.y_axis.y = mat.y_axis.y;
        self.inner.y_axis.z = mat.y_axis.z;

        self.inner.z_axis.x = mat.z_axis.x;
        self.inner.z_axis.y = mat.z_axis.y;
        self.inner.z_axis.z = mat.z_axis.z;
    }

    #[inline]
    pub fn get_right_vec(&self) -> glam::Vec3 {
        self.inner.x_axis.xyz().normalize_or_zero()
    }

    #[inline]
    pub fn get_up_vec(&self) -> glam::Vec3 {
        self.inner.y_axis.xyz().normalize_or_zero()
    }

    #[inline]
    pub fn get_look_vec(&self) -> glam::Vec3 {
        self.inner.z_axis.xyz().normalize_or_zero()
    }

    #[inline]
    pub fn translate_local(&mut self, distance: glam::Vec3) {
        let right = self.get_right_vec();
        let up = self.get_up_vec();
        let look = self.get_look_vec();
        self.translate_world(right * distance.x + up * distance.y + look * distance.z);
    }

    #[inline]
    pub fn translate_world(&mut self, distance: glam::Vec3) {
        self.inner.w_axis.x += distance.x;
        self.inner.w_axis.y += distance.y;
        self.inner.w_axis.z += distance.z;
    }

    #[inline]
    pub fn rotate(&mut self, rotation: glam::Quat) {
        *self.world_matrix_mut() = self.world_matrix_ref().mul_mat4(&glam::Mat4::from_quat(rotation.normalize()));
    }

    #[inline]
    pub fn rotate_from_axis_angle(&mut self, axis: glam::Vec3, angle: f32) {
        self.rotate(glam::Quat::from_axis_angle(axis.normalize_or_zero(), angle));
    }

    #[inline]
    pub fn rotate_from_x_axis(&mut self, angle: f32) {
        self.rotate(glam::Quat::from_rotation_x(angle));
    }

    #[inline]
    pub fn rotate_from_y_axis(&mut self, angle: f32) {
        self.rotate(glam::Quat::from_rotation_y(angle));
    }

    #[inline]
    pub fn rotate_from_z_axis(&mut self, angle: f32) {
        self.rotate(glam::Quat::from_rotation_z(angle));
    }

    #[inline]
    pub fn world_matrix_ref(&self) -> &glam::Mat4 {
        &self.inner
    }

    #[inline]
    pub fn world_matrix_mut(&mut self) -> &mut glam::Mat4 {
        &mut self.inner
    }

    #[inline]
    pub fn view_matrix(&self) -> glam::Mat4 {
        let translation = self.get_translation();
        let right = self.get_right_vec();
        let up = self.get_up_vec();
        let look = self.get_look_vec();
        return glam::Mat4::from_cols(
            glam::vec4(right.x, up.x, look.x, 0.0), 
            glam::vec4(right.y, up.y, look.y, 0.0), 
            glam::vec4(right.z, up.z, look.z, 0.0), 
            glam::vec4(
                -translation.dot(right), 
                -translation.dot(up), 
                -translation.dot(look), 
                1.0
            )
        )
    }
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Self { inner: glam::Mat4::IDENTITY }
    }
}



/// #### 한국어 </br>
/// `Transform`을 생성하는 빌더입니다. </br>
/// 
/// #### English (Translation) </br>
/// The builder that generates the `Transform`. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformBuilder {
    pub scale: glam::Vec3, 
    pub rotation: glam::Quat, 
    pub translation: glam::Vec3, 
}

impl TransformBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn set_scale(mut self, scale: glam::Vec3) -> Self {
        self.scale = scale;
        self
    }

    #[inline]
    pub fn set_rotation(mut self, rotation: glam::Quat) -> Self {
        self.rotation = rotation.normalize();
        self
    }

    #[inline]
    pub fn set_translation(mut self, translation: glam::Vec3) -> Self {
        self.translation = translation;
        self
    }

    #[inline]
    pub fn translate_local(self, distance: glam::Vec3) -> Self {
        let mat = glam::Mat3::from_quat(self.rotation);
        let right = mat.x_axis.normalize_or_zero();
        let up = mat.y_axis.normalize_or_zero();
        let look = mat.z_axis.normalize_or_zero();
        self.translate_world(right * distance.x + up * distance.y + look * distance.z)
    }

    #[inline]
    pub fn translate_world(mut self, distance: glam::Vec3) -> Self {
        self.translation += distance;
        self
    }

    #[inline]
    pub fn rotate(mut self, rotation: glam::Quat) -> Self {
        self.rotation = self.rotation.mul_quat(rotation.normalize()).normalize();
        self
    }

    #[inline]
    pub fn rotate_from_axis_angle(self, axis: glam::Vec3, angle: f32) -> Self {
        self.rotate(glam::Quat::from_axis_angle(axis.normalize_or_zero(), angle))
    }

    #[inline]
    pub fn rotate_from_x_axis(self, angle: f32) -> Self {
        self.rotate(glam::Quat::from_rotation_x(angle))
    }

    #[inline]
    pub fn rotate_from_y_axis(self, angle: f32) -> Self {
        self.rotate(glam::Quat::from_rotation_y(angle))
    }

    #[inline]
    pub fn rotate_from_z_axis(self, angle: f32) -> Self {
        self.rotate(glam::Quat::from_rotation_z(angle))
    }

    #[inline]
    pub fn build(self) -> Transform {
        Transform { 
            inner: glam::Mat4::from_scale_rotation_translation(
                self.scale, 
                self.rotation, 
                self.translation
            )
        }
    }
}

impl Default for TransformBuilder {
    #[inline]
    fn default() -> Self {
        Self { 
            scale: glam::Vec3::ONE, 
            rotation: glam::Quat::IDENTITY, 
            translation: glam::Vec3::ZERO, 
        }
    }
}
