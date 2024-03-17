#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f32, 
    pub green: f32, 
    pub blue: f32, 
    pub alpha: f32, 
}

impl Color {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn from_rgb(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue, alpha: 1.0 }
    }

    #[inline]
    pub fn from_rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { red, green, blue, alpha }
    }

    #[inline]
    pub fn as_vec4(&self) -> glam::Vec4 {
        glam::vec4(self.red, self.blue, self.green, self.alpha)
    }
}

impl Default for Color {
    #[inline]
    fn default() -> Self {
        Self { 
            red: 1.0, 
            green: 1.0, 
            blue: 1.0, 
            alpha: 1.0 
        }
    }
}
