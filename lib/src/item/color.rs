/// #### 한국어 </br>
/// 물체의 색상 데이터 입니다. </br>
/// 
/// #### English (Translation) </br>
/// The color data for the object. </br>
/// 
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Rgb {
        red: f32, 
        green: f32, 
        blue: f32,
    }, 
    Rgba {
        red: f32, 
        green: f32, 
        blue: f32, 
        alpha: f32, 
    }
}

impl Color {
    #[inline]
    pub fn as_vec4(&self) -> glam::Vec4 {
        match self {
            Color::Rgb { red, green, blue } => {
                glam::vec4(*red, *green, *blue, 1.0)
            },
            Color::Rgba { red, green, blue, alpha } => {
                glam::vec4(*red, *green, *blue, *alpha)
            }
        }
    }
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self::Rgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }
    }
}
