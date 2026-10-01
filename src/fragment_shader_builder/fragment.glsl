#version 460

// =========================================================================
// This file is a template that will get filled in at runtime, but it is
// included at compile time so do not forget to recompile if you  modify it.
// Also it requires double curly brackets for templates reasons
// =========================================================================

#define MAX_STEPS 1000
#define MAX_DIST 30.0
#define SURF_DIST (1.0/256.0)

// The collisions that can happen
#define COLLISION_NONE 0
{object_collision_id_definition}

uniform vec3 camera_position;
uniform mat3 camera_rotation_mat;
uniform vec2 screen_size;

out vec3 color;

struct Material {{
    vec3 color;
}};

//const Material MATERIAL_LIST[...] = ...
{material_list_definition};


vec3 rotateVector(vec3 vec) {{
    return camera_rotation_mat * vec;
}}

float smoothMerge(float shapeA, float shapeB, float k) {{
    float h = clamp(0.5 + 0.5 * (shapeB - shapeA) / k, 0.0, 1.0);
    return mix(shapeB, shapeA, h) - k * h * (1.0 - h);
}}


// A distance function per object
{scene_objects_function_definition}


uint getCollisionMaterial(vec3 ray) {{
    uint objectId = 0;

    {collision_checks}

    return objectId;
}}

//SDF
float sdf_scene(vec3 ray) {{
    return {scene_unified_sdf};
}}

vec3 raymarch(vec3 ro, vec3 rd) {{
    float total_dist = 0.0;
    uint collision = COLLISION_NONE;
    int i = 0;
    float dist = 0.0;
    for(i = 0; i < MAX_STEPS; i++) {{
        vec3 p = ro + rd * total_dist;
        dist = sdf_scene(p);
        total_dist += dist;

        if(total_dist >= MAX_DIST) {{
            break;
        }}

        if(dist <= SURF_DIST) {{
            collision = getCollisionMaterial(p);

            return MATERIAL_LIST[collision].color;

        }}
    }}
    //if (i > MAX_STEPS) {{
    //    dist = MAX_DIST;
    //}}

    return vec3(0.21, 0.27, 0.31);
    //return vec3(total_dist);
}}

void main() {{
    vec2 uv = (gl_FragCoord.xy - 0.5 * screen_size) / screen_size.y;

    vec3 ray_origin = camera_position;
    vec3 fragment_direction = normalize(vec3(uv.x, uv.y, -1.0));
    vec3 ray_direction = rotateVector(fragment_direction);

    color = raymarch(ray_origin, ray_direction);
}}
