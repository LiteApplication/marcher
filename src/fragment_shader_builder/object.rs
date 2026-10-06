use crate::fragment_shader_builder::materials::Material;
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
    /// You should always define either [get_sdf] or [get_sdf_function], or the shader will not compile due to recursive function use
    fn get_sdf(&self) -> String {
        format!("{}(ray)", self.get_function_name())
    }
    fn get_material(&self) -> &Rc<dyn Material>;
    /// Same as [get_sdf], here we define a default value to compute the normal automatically for shapes that don't define it
    /// It is still more efficient to define it exactly.
    fn get_normal(&self) -> String {
        let func_name = self.get_function_name();
        format!(
            "(normalize(vec3(
                    {func}(ray + NORMAL_CALCULATION_VECTOR.xyy) - {func}(ray - NORMAL_CALCULATION_VECTOR.xyy),
                    {func}(ray + NORMAL_CALCULATION_VECTOR.yxy) - {func}(ray - NORMAL_CALCULATION_VECTOR.yxy),
                    {func}(ray + NORMAL_CALCULATION_VECTOR.yyx) - {func}(ray - NORMAL_CALCULATION_VECTOR.yyx)
                ))
            )",
            func = func_name
        )
    }

    /// If the SDF function is too complicated to fit into one line, you can define this
    /// in your object and call that function in [get_sdf]
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
    raw_function_body: String,
    material: Rc<dyn Material>,
    id: u32,
}

impl ArbitraryObject {
    fn new(raw_function_body: String, material: Rc<dyn Material>) -> Self {
        Self {
            raw_function_body,
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

    fn get_sdf_function(&self) -> String {
        format!(
            "float {function_name}(vec3) {{
    {body}
}}",
            function_name = self.get_function_name(),
            body = self.raw_function_body
        )
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

    fn vec3_center(&self) -> String {
        format!(
            "vec3({x},{y},{z})",
            x = self.center[0],
            y = self.center[1],
            z = self.center[2],
        )
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
            "(length({ray} - {center}) - {r})",
            ray = "ray",
            center = self.vec3_center(),
            r = self.r,
        )
    }
    fn get_normal(&self) -> String {
        format!(
            "normalize({ray} - {center})",
            ray = "ray",
            center = self.vec3_center()
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

    fn get_normal(&self) -> String {
        "vec3(0.0,1.0,0.0)".to_string()
    }
}

pub struct BoxObject {
    center: [f32; 3],
    extents: [f32; 3], // Half-widths along X, Y, and Z axes (bounds from center)
    id: u32,
    material: Rc<dyn Material>,
}

impl BoxObject {
    pub fn build(center: [f32; 3], extents: [f32; 3], material: Rc<dyn Material>) -> Result<Self> {
        if extents[0] < 0.0 || extents[1] < 0.0 || extents[2] < 0.0 {
            return Err(anyhow!("The box extents must be positive dimensions"));
        }
        Ok(Self {
            center,
            extents,
            id: get_new_id(),
            material,
        })
    }

    fn vec3_center(&self) -> String {
        format!(
            "vec3({x},{y},{z})",
            x = self.center[0],
            y = self.center[1],
            z = self.center[2],
        )
    }

    fn vec3_extents(&self) -> String {
        format!(
            "vec3({x},{y},{z})",
            x = self.extents[0],
            y = self.extents[1],
            z = self.extents[2],
        )
    }
}

impl SceneObject for BoxObject {
    fn get_material(&self) -> &Rc<dyn Material> {
        &self.material
    }

    fn get_function_name(&self) -> String {
        format!("sdf_box_{}", self.id)
    }

    // Uses Inigo Quilez's standard 3D Box SDF algorithm
    fn get_sdf_function(&self) -> String {
        let center = self.vec3_center();
        let extents = self.vec3_extents();
        format!(
            "
float {func_name}(vec3 ray){{
    vec3 q = abs(ray - {center}) - {extents};
    return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);
}}",
            func_name = self.get_function_name(),
            center = center,
            extents = extents
        )
    }

    fn get_normal(&self) -> String {
        let q = format!(
            "(({var} - {c}) / {e})",
            var = "ray",
            c = self.vec3_center(),
            e = self.vec3_extents()
        );
        format!("(sign({q}) * step(abs({q}).yzx, abs({q})) * step(abs({q}).zxy, abs({q})))")
    }
}
