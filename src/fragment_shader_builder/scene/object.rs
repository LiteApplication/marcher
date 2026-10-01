use crate::fragment_shader_builder::materials::{self, Material};
use anyhow::{Result, anyhow};
use std::{rc::Rc, sync::Mutex};

static _FUNCTION_NAME_ID: Mutex<u32> = Mutex::new(0);
fn get_new_id() -> u32 {
    let mut function_id = _FUNCTION_NAME_ID
        .lock()
        .expect("function name id is poisoned");
    *function_id += 1;
    *function_id
}

/// This is implemented by objects we want represented inside the signed distance field
pub trait SceneObject {
    /// Should return the formula for the SDF at a point called "ray".
    /// I recommend putting the formula in parenthesis to avoid problems with the operation order when combining shapes.
    fn get_sdf(&self) -> String;
    fn get_material(&self) -> &Rc<dyn Material>;

    fn get_sdf_function(&self) -> String {
        format!(
            "float {}(vec3 ray) {{
    return {};
}}
        ",
            self.get_function_name(),
            self.get_sdf()
        )
    }
    fn get_function_name(&self) -> String;
}

pub struct ArbitraryObject {
    raw_formula: String,
    material: Rc<dyn Material>,
    id: u32,
}

impl ArbitraryObject {
    fn new(raw_formula: String, material: Rc<dyn Material>) -> Self {
        Self {
            raw_formula,
            material,
            id: get_new_id(),
        }
    }
}

impl SceneObject for ArbitraryObject {
    fn get_material(&self) -> &Rc<dyn Material> {
        &self.material
    }

    fn get_function_name(&self) -> String {
        format!("sdf_arbitrary_{}", self.id)
    }

    fn get_sdf(&self) -> String {
        self.raw_formula.clone()
    }
}

pub struct Sphere {
    center: [f32; 3],
    r: f32,
    id: u32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn build(center: [f32; 3], r: f32, material: Rc<dyn Material>) -> Result<Self> {
        if r < 0.0 {
            return Err(anyhow!("The radius of the sphere should be positive"));
        }
        Ok(Self {
            center,
            r,
            id: get_new_id(),
            material,
        })
    }
}

impl SceneObject for Sphere {
    fn get_material(&self) -> &Rc<dyn Material> {
        &self.material
    }
    fn get_function_name(&self) -> String {
        format!("sdf_sphere_{}", self.id)
    }
    fn get_sdf(&self) -> String {
        format!(
            "(length({ray} - vec3({x},{y},{z})) - {r})",
            ray = "ray",
            x = self.center[0],
            y = self.center[1],
            z = self.center[2],
            r = self.r,
        )
    }
}

pub struct InfiniteYPlane {
    y: f32,
    id: u32,
    material: Rc<dyn Material>,
}

impl InfiniteYPlane {
    pub fn new(y: f32, material: Rc<dyn Material>) -> Self {
        Self {
            y,
            material,
            id: get_new_id(),
        }
    }
}

impl SceneObject for InfiniteYPlane {
    fn get_function_name(&self) -> String {
        format!("sdf_infinite_y_plane_{}", self.id)
    }

    fn get_material(&self) -> &Rc<dyn Material> {
        &self.material
    }
    fn get_sdf(&self) -> String {
        format!("(ray.y - {})", self.y)
    }
}
