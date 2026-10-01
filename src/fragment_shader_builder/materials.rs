mod colored_surface;

use std::{collections::HashMap, rc::Rc};

pub use colored_surface::SolidColoredSurface;

/// This trait represents some material that we will use inside the verted shader to
pub trait Material {
    /// Get the code to define the struct for this material
    ///
    /// This should be compatible with what is defined in [`MATERIAL_STRUCT_DEF`]
    /// ```c
    /// struct {
    ///     vec3 color;
    /// } Material;
    /// ```
    fn get_struct(&self) -> String {
        let (r, g, b) = self.get_color();
        return format!("Material(vec3({r}, {g}, {b}))");
    }

    fn get_color(&self) -> (f32, f32, f32);
}

pub struct MaterialList {
    materials: Vec<(String, Rc<dyn Material>)>,
}

impl MaterialList {
    pub fn new() -> Self {
        MaterialList {
            materials: Vec::new(),
        }
    }
    pub fn get_material(&self, name: &String) -> Option<Rc<dyn Material>> {
        self.materials
            .iter()
            .find(|(n, _)| name == n)
            .map(|(_, mat)| mat.clone())
    }
    pub fn add_material(&mut self, name: String, material: impl Material + 'static) {
        self.materials.push((name, Rc::new(material)));
    }
    pub fn get_material_index(&self, material: &Rc<dyn Material>) -> Option<usize> {
        self.all_materials()
            .iter()
            .enumerate()
            .filter(|(_, mat)| Rc::ptr_eq(mat, material))
            .map(|(i, _)| i)
            .next()
    }

    pub fn size(&self) -> usize {
        self.materials.iter().count()
    }

    pub fn all_materials(&self) -> Vec<Rc<dyn Material>> {
        self.materials.iter().map(|(_, mat)| mat.clone()).collect()
    }
}
