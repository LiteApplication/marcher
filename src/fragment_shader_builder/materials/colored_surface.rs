use anyhow::{Result, anyhow};

use super::Material;

pub struct SolidColoredSurface {
    r: f32,
    g: f32,
    b: f32,
}

impl SolidColoredSurface {
    pub fn build(r: f32, g: f32, b: f32) -> Result<Self> {
        if r > 1.0 || g > 1.0 || b > 1.0 {
            return Err(anyhow!(
                "red, green and blue should be floating numbers between 0 and 1"
            ));
        }
        Ok(Self { r, g, b })
    }
}

impl Material for SolidColoredSurface {
    fn get_albedo(&self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    fn get_metallic(&self) -> f32 {
        0.0
    }
    fn get_emission(&self) -> (f32, f32, f32) {
        (0.0, 0.0, 0.0)
    }
    fn get_index_of_refraction(&self) -> f32 {
        0.0
    }
    fn get_roughness(&self) -> f32 {
        1.0
    }
}
