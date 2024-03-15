use std::fmt;
use glam::Vec4Swizzles;

/// #### 한국어 </br>
/// 게임 월드에 존재하는 오브젝트들의 trait입니다. </br>
/// 
/// #### English (Translation) </br>
/// These are traits of objects that exist in the game world. </br>
/// 
pub trait GameObject : fmt::Debug {
    #[inline]
    fn get_translation(&self) -> glam::Vec3 {
        self.world_transform_ref().w_axis.xyz()
    }

    #[inline]
    fn set_translation(&mut self, translation: glam::Vec3) {
        self.world_transform_mut().w_axis.x = translation.x;
        self.world_transform_mut().w_axis.y = translation.y;
        self.world_transform_mut().w_axis.z = translation.z;
    }

    #[inline]
    fn translate_world(&mut self, distance: glam::Vec3) {
        self.world_transform_mut().w_axis.x += distance.x;
        self.world_transform_mut().w_axis.y += distance.y;
        self.world_transform_mut().w_axis.z += distance.z;
    }

    #[inline]
    fn translate_local(&mut self, distance: glam::Vec3) {
        let right = self.world_transform_ref().x_axis.xyz().normalize_or_zero();
        let up = self.world_transform_ref().y_axis.xyz().normalize_or_zero();
        let look = self.world_transform_ref().z_axis.xyz().normalize_or_zero();
        self.translate_world(right * distance.x + up * distance.y + look * distance.z)
    }

    #[inline]
    fn get_rotation(&self) -> glam::Quat {
        glam::Quat::from_mat4(self.world_transform_ref()).normalize()
    }

    #[inline]
    fn set_rotation(&mut self, rotation: glam::Quat) {
        let mat = glam::Mat3::from_quat(rotation.normalize());
        self.world_transform_mut().x_axis.x = mat.x_axis.x;
        self.world_transform_mut().x_axis.y = mat.x_axis.y;
        self.world_transform_mut().x_axis.z = mat.x_axis.z;

        self.world_transform_mut().y_axis.x = mat.y_axis.x;
        self.world_transform_mut().y_axis.y = mat.y_axis.y;
        self.world_transform_mut().y_axis.z = mat.y_axis.z;

        self.world_transform_mut().z_axis.x = mat.z_axis.x;
        self.world_transform_mut().z_axis.y = mat.z_axis.y;
        self.world_transform_mut().z_axis.z = mat.z_axis.z;
    }

    #[inline]
    fn get_right_vec(&self) -> glam::Vec3 {
        self.world_transform_ref().x_axis.xyz().normalize_or_zero()
    }

    #[inline]
    fn get_up_vec(&self) -> glam::Vec3 {
        self.world_transform_ref().y_axis.xyz().normalize_or_zero()
    }

    #[inline]
    fn get_look_vec(&self) -> glam::Vec3 {
        self.world_transform_ref().z_axis.xyz().normalize_or_zero()
    }

    #[inline]
    fn rotate(&mut self, rotation: glam::Quat) {
        let mat = glam::Mat4::from_quat(rotation.normalize());
        *self.world_transform_mut() = self.world_transform_ref().mul_mat4(&mat);
    }

    #[inline]
    fn rotate_from_axis_angle(&mut self, axis: glam::Vec3, angle: f32) {
        let mat = glam::Mat4::from_axis_angle(axis.normalize_or_zero(), angle);
        *self.world_transform_mut() = self.world_transform_ref().mul_mat4(&mat);
    }

    #[inline]
    fn rotate_from_rotation_x(&mut self, angle: f32) {
        let mat = glam::Mat4::from_rotation_x(angle);
        *self.world_transform_mut() = self.world_transform_ref().mul_mat4(&mat);
    }

    #[inline]
    fn rotate_from_rotation_y(&mut self, angle: f32) {
        let mat = glam::Mat4::from_rotation_y(angle);
        *self.world_transform_mut() = self.world_transform_ref().mul_mat4(&mat);
    }

    #[inline]
    fn rotate_from_rotation_z(&mut self, angle: f32) {
        let mat = glam::Mat4::from_rotation_z(angle);
        *self.world_transform_mut() = self.world_transform_ref().mul_mat4(&mat);
    }

    #[inline]
    fn bytes(&self) -> Option<&[u8]> { None }
    
    fn world_transform_ref(&self) -> &glam::Mat4;

    fn world_transform_mut(&mut self) -> &mut glam::Mat4;
}

/// #### 한국어 </br>
/// 게임 월드에 존재하는 카메라 오브젝트들의 trait입니다. </br>
/// 
/// #### English (Translation) </br>
/// These are traits of camera objects that exist in the game world. </br>
/// 
pub trait CameraObject : GameObject {
    fn view_transform(&self) -> glam::Mat4;
    
    fn projection_transform(&self) -> glam::Mat4;
}
