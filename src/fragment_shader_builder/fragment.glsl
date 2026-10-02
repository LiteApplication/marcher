#version 460

// =========================================================================
// This file is a template that will get filled in at runtime, but it is
// included at compile time so do not forget to recompile if you  modify it.
// Also it requires double curly brackets for templates reasons
// =========================================================================

#define MAX_STEPS 1000
#define MAX_DIST 30.0
#define SURF_DIST (1.0/256.0)
#define SURF_DIST_TOLERENCE 1.01
#define AMBIANT_LIGHT 1.0
#define REFLECTED_LIGHT 0.7
#define MAX_REFLECTION 3
#define NUDGE_AFTER_BOUNCE 0.01
#define NORMAL_CALCULATION_VECTOR vec2(0.001, 0.0)

// The collisions that can happen
{object_collision_id_definition}

uniform vec3 camera_position;
uniform mat3 camera_rotation_mat;
uniform vec2 screen_size;

out vec3 color;

struct Material {{
    vec3 color;
    float reflection;
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

vec3 reflectVector(vec3 Ri, vec3 Normal) {{
    return Ri - 2.0 * dot(Ri, Normal) * Normal;
}}


// A distance function per object
{scene_objects_function_definition}

struct CollisionInfo {{
    uint materialId;
    vec3 lightColor;
    vec3 surfaceNormal;
    bool hit;
}};

CollisionInfo getCollision(vec3 ray) {{
    CollisionInfo collisionInfo;

    // Default values if we miss everything
    collisionInfo.hit = false;
    collisionInfo.lightColor = vec3(0.0);
    collisionInfo.materialId = 0u;
    collisionInfo.surfaceNormal = vec3(0.0, 1.0, 0.0);

    {collision_checks}


    if (collisionInfo.hit) {{
        // Temporary, will change later
        vec3 lightDir = normalize(vec3(0.5, 0.75, -0.4));
        vec3 lightColorSource = vec3(1.0, 0.95, 0.85);
        vec3 ambientLight = vec3(0.15, 0.18, 0.22);

        // Diffuse (Lambertian) shading
        float diffuseIntensity = max(dot(collisionInfo.surfaceNormal, lightDir), 0.0);

        // Combine them into the final lightColor field
        collisionInfo.lightColor = ambientLight + (lightColorSource * diffuseIntensity);
    }}


    return collisionInfo;
}}

//SDF
float sdf_scene(vec3 ray) {{
    return {scene_unified_sdf};
}}


vec3 calculateColor(CollisionInfo collision[MAX_REFLECTION], vec3 bgColor) {{
    vec3 finalColor = vec3(0.0);
    vec3 remaining = vec3(1.0);

    for (int i = 0; i < MAX_REFLECTION; i++) {{
            // If this bounce missed everything, add the sky color multiplied
            // by the current light throughput, and terminate.
            if (!collision[i].hit) {{
                finalColor += remaining * bgColor;
                break;
            }}

            Material mat = MATERIAL_LIST[collision[i].materialId];

            // Collect the local lighting at this surface, tinted by its color
            // and scaled by how much light remains.
            vec3 surfaceLitColor = collision[i].lightColor * mat.color;

            // Add it to the final pixel color, taking diffuse lighting into account
            // for now the inverse of reflection, might change later for a separate field.
            finalColor += remaining * surfaceLitColor * (1.0 - mat.reflection);

            // Reduce the light left for the next bounce based on
            // the reflection factor and surface color.
            remaining *= mat.color * mat.reflection;


            // If there is not enough light remaining for the ray we stop there
            if (dot(remaining, remaining) < 0.0001) {{
                break;
            }}
        }}

    return finalColor;
}}

vec3 raymarch(vec3 ro, vec3 rd) {{
    CollisionInfo[MAX_REFLECTION] collision;
    int reflection = 0;
    int i = 0;
    float dist = 0.0;
    for (int j = 0; j < MAX_REFLECTION; j++) collision[j].hit = false;
    for (reflection = 0; reflection < MAX_REFLECTION; reflection++) {{
        bool hitThisBounce = false;
        float total_dist = 0.0;
        for(i = 0; i < MAX_STEPS; i++) {{
            vec3 p = ro + rd * total_dist;
            dist = sdf_scene(p);
            total_dist += dist;

            if(total_dist >= MAX_DIST) {{
                break;
            }}

            if(dist <= SURF_DIST) {{
                collision[reflection] = getCollision(p);
                collision[reflection].hit = true;
                hitThisBounce = true;

                rd = reflect(rd, collision[reflection].surfaceNormal);
                ro = p + collision[reflection].surfaceNormal * (SURF_DIST * 2.0);
                break;
            }}
        }}

        if (!hitThisBounce) {{
            break;
        }}
    }}

    return calculateColor(collision,vec3(0.21, 0.27, 0.31));
}}


void main() {{
    vec2 uv = (gl_FragCoord.xy - 0.5 * screen_size) / screen_size.y;
    //color = vec3(uv, 0.0);

    vec3 ray_origin = camera_position;
    vec3 fragment_direction = normalize(vec3(uv.x, uv.y, -0.5));//-2.41));
    vec3 ray_direction = rotateVector(fragment_direction);

    color = raymarch(ray_origin, ray_direction);
}}
