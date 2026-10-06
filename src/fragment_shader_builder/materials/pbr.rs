use anyhow::{Result, anyhow};

use crate::fragment_shader_builder::materials::Material;

pub enum MaterialPreset {
    Gold,
    Copper,
    Aluminum,
    Iron,
    Silver,
    PlasticRed,
    PlasticWhite,
    Rubber,
    Glass,
    Water,
    EmissiveLight,
}

pub struct PresetSurface {
    albedo: (f32, f32, f32),
    metallic: f32,
    roughness: f32,
    ior: f32,
    emission: (f32, f32, f32),
}

impl PresetSurface {
    pub fn from_preset(preset: MaterialPreset) -> Self {
        match preset {
            MaterialPreset::Gold => Self {
                albedo: (1.000, 0.766, 0.336),
                metallic: 1.0,
                roughness: 0.15,
                ior: 1.0,
                emission: (0.0, 0.0, 0.0),
            },
            MaterialPreset::Copper => Self {
                albedo: (0.955, 0.637, 0.538),
                metallic: 1.0,
                roughness: 0.2,
                ior: 1.0,
                emission: (0.0, 0.0, 0.0),
            },
            MaterialPreset::Aluminum => Self {
                albedo: (0.913, 0.921, 0.925),
                metallic: 1.0,
                roughness: 0.25,
                ior: 1.0,
                emission: (0.0, 0.0, 0.0),
            },
            MaterialPreset::Iron => Self {
                albedo: (0.560, 0.570, 0.580),
                metallic: 1.0,
                roughness: 0.35,
                ior: 1.0,
                emission: (0.0, 0.0, 0.0),
            },
            MaterialPreset::Silver => Self {
                albedo: (0.972, 0.960, 0.915),
                metallic: 1.0,
                roughness: 0.1,
                ior: 1.0,
                emission: (0.0, 0.0, 0.0),
            },

            MaterialPreset::PlasticRed => Self {
                albedo: (0.85, 0.05, 0.05),
                metallic: 0.0,
                roughness: 0.3,
                ior: 1.5,
                emission: (0.0, 0.0, 0.0),
            },
            MaterialPreset::PlasticWhite => Self {
                albedo: (0.9, 0.9, 0.9),
                metallic: 0.0,
                roughness: 0.25,
                ior: 1.5,
                emission: (0.0, 0.0, 0.0),
            },
            MaterialPreset::Rubber => Self {
                albedo: (0.02, 0.02, 0.02),
                metallic: 0.0,
                roughness: 0.85,
                ior: 1.52,
                emission: (0.0, 0.0, 0.0),
            },

            MaterialPreset::Glass => Self {
                albedo: (1.0, 1.0, 1.0),
                metallic: 0.0,
                roughness: 0.02,
                ior: 1.52,
                emission: (0.0, 0.0, 0.0),
            },
            MaterialPreset::Water => Self {
                albedo: (1.0, 1.0, 1.0),
                metallic: 0.0,
                roughness: 0.05,
                ior: 1.333,
                emission: (0.0, 0.0, 0.0),
            },

            MaterialPreset::EmissiveLight => Self {
                albedo: (1.0, 1.0, 1.0),
                metallic: 0.0,
                roughness: 1.0,
                ior: 1.0,
                emission: (10.0, 9.5, 8.5),
            },
        }
    }

    pub fn with_roughness(mut self, roughness: f32) -> Result<Self> {
        if !(0.0..=1.0).contains(&roughness) {
            return Err(anyhow!("Roughness must be between 0.0 and 1.0"));
        }
        self.roughness = roughness;
        Ok(self)
    }

    pub fn with_albedo(mut self, r: f32, g: f32, b: f32) -> Result<Self> {
        if r < 0.0 || r > 1.0 || g < 0.0 || g > 1.0 || b < 0.0 || b > 1.0 {
            return Err(anyhow!("Albedo components must be between 0.0 and 1.0"));
        }
        self.albedo = (r, g, b);
        Ok(self)
    }
}

impl Material for PresetSurface {
    fn get_albedo(&self) -> (f32, f32, f32) {
        self.albedo
    }

    fn get_metallic(&self) -> f32 {
        self.metallic
    }

    fn get_emission(&self) -> (f32, f32, f32) {
        self.emission
    }

    fn get_index_of_refraction(&self) -> f32 {
        self.ior
    }

    fn get_roughness(&self) -> f32 {
        self.roughness
    }
}
