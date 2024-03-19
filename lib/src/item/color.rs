/// #### 한국어 </br>
/// 불투명한 물체의 색상 데이터 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is color data for opaque objects. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub red: f32, 
    pub green: f32, 
    pub blue: f32, 
}

#[allow(dead_code)]
impl Rgb {
    #[inline]
    pub fn as_vec4(&self) -> glam::Vec4 {
        glam::vec4(self.red, self.green, self.blue, 1.0)
    }
}

impl Default for Rgb {
    #[inline]
    fn default() -> Self {
        Self { red: 1.0, green: 1.0, blue: 1.0 }
    }
}



/// #### 한국어 </br>
/// 투명한 물체의 색상 데이터 입니다. </br>
/// 
/// #### English (Translation) </br>
/// This is color data for transparent objects. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgba {
    pub red: f32, 
    pub green: f32, 
    pub blue: f32, 
    pub alpha: f32, 
}

#[allow(dead_code)]
impl Rgba {
    #[inline]
    pub fn as_vec4(&self) -> glam::Vec4 {
        glam::vec4(self.red, self.green, self.blue, self.alpha)
    }
}

impl Default for Rgba {
    #[inline]
    fn default() -> Self {
        Self { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
    }
}
