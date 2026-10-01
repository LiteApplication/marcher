use std::sync::Mutex;

pub mod object;
pub use object::{InfiniteYPlane, SceneObject, Sphere};

use crate::fragment_shader_builder::materials::MaterialList;

pub struct Scene {
    scene_objects: Vec<Box<dyn SceneObject>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            scene_objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: impl SceneObject + 'static) {
        self.scene_objects.push(Box::new(object));
    }

    pub fn get_global_sdf(&self) -> String {
        self.scene_objects
            .iter()
            .map(|obj| obj.get_sdf())
            .reduce(|e1, e2| format!("min({},{})", e1, e2))
            .unwrap_or("1".to_string())
    }

    pub fn all_object(&self) -> Vec<&dyn SceneObject> {
        self.scene_objects.iter().map(|obj| &**obj).collect()
    }

    pub fn define_collision_ids(&self, material_list: &MaterialList) -> String {
        self.scene_objects
            .iter()
            .map(|obj| {
                format!(
                    "#define COLLISION_{} {}",
                    obj.get_function_name(),
                    &material_list
                        .get_material_index(obj.get_material())
                        .expect("material was not found in the material list")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn collision_checks(&self) -> String {
        self.scene_objects
            .iter()
            .map(|obj| {
                format!(
                    "if ({}(ray) <= SURF_DIST) objectId = {};",
                    obj.get_function_name(),
                    "COLLISION_".to_string() + &obj.get_function_name()
                )
            })
            .collect::<Vec<_>>()
            .join("\n    ")
    }
}
