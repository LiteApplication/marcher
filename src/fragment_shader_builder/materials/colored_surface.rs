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
    fn get_color(&self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    fn get_reflection(&self) -> f32 {
        0.0
    }
}

pub struct ReflectiveColoredSurface {
    r: f32,
    g: f32,
    b: f32,
    reflection: f32,
}

impl ReflectiveColoredSurface {
    pub fn build(r: f32, g: f32, b: f32, reflection: f32) -> Result<Self> {
        if r > 1.0 || g > 1.0 || b > 1.0 {
            return Err(anyhow!(
                "red, green and blue should be floating numbers between 0 and 1"
            ));
        }
        Ok(Self {
            r,
            g,
            b,
            reflection,
        })
    }
}

impl Material for ReflectiveColoredSurface {
    fn get_color(&self) -> (f32, f32, f32) {
        (self.r, self.g, self.b)
    }

    fn get_reflection(&self) -> f32 {
        self.reflection
    }
}
