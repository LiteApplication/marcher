use crate::fragment_shader_builder::{materials::MaterialList, scene::Scene};

pub mod materials;
pub mod object;
pub mod scene;

/// This is the trait that will build the vertex shader, which when executed will do all the rendering of the scene
pub trait FragmentBuilder {
    fn export(&self) -> String;

    fn debug_print(&self) -> () {
        let result = self.export();
        // not going to run this on a 16 bit machine because it probably does not have OpenGL support
        let width: usize = result.lines().count().to_string().len();
        let _ = result
            .lines()
            .enumerate()
            .map(|(number, line)| println!("{number:0width$} |{line}"))
            .collect::<Vec<_>>();
    }
}

pub struct TestFragmentBuilder {
    materials: MaterialList,
    scene: Scene,
}

impl TestFragmentBuilder {
    pub fn new(materials: MaterialList, scene: Scene) -> Self {
        Self { materials, scene }
    }

    fn scene_objects_function_def(&self) -> String {
        self.scene
            .all_object()
            .iter()
            .map(|so| so.get_sdf_function())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn material_list_def(&self) -> String {
        format!(
            "const Material MATERIAL_LIST[{len}] = Material[{len}]({content})",
            len = self.materials.size(),
            content = self
                .materials
                .all_materials()
                .iter()
                .map(|mat| mat.get_struct())
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl FragmentBuilder for TestFragmentBuilder {
    fn export(&self) -> String {
        let result = format!(
            include_str!("fragment_shader_builder/fragment.glsl"),
            material_list_definition = self.material_list_def(),
            scene_objects_function_definition = self.scene_objects_function_def(),
            scene_unified_sdf = self.scene.get_global_sdf(),
            collision_checks = self.scene.collision_checks(),
            object_collision_id_definition = self.scene.define_collision_ids(&self.materials)
        );
        result
    }
}
