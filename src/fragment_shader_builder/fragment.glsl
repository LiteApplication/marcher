#version 460

// =========================================================================
// This file is a template that will get filled in at runtime, but it is
// included at compile time so do not forget to recompile if you  modify it.
// Also it requires double curly brackets for templates reasons
// =========================================================================

#define MAX_STEPS 1000
#define MAX_DIST 30.0
#define SURF_DIST (1.0/128.0)
#define SURF_DIST_TOLERENCE 1.01
#define MAX_REFLECTION 20
#define NUM_RAYS 1
#define NUDGE_AFTER_BOUNCE (SURF_DIST * 2)
#define NORMAL_CALCULATION_VECTOR vec2(0.001, 0.0)

// The collisions that can happen
{object_collision_id_definition}

uniform vec3 camera_position;
uniform mat3 camera_rotation_mat;
uniform vec2 screen_size;

out vec4 color;

struct Material {{
    vec3  albedo;
    float metallic;
    float roughness;
    float ior;
    vec3  emission;
}};


struct CollisionInfo {{
    // Index into the MATERIAL array
    uint materialId;
    // Vector perpendicular to the surface
    vec3 surfaceNormal;
    // Self explanatory
    vec3 hitPoint;
    // Vector pointing towards the camera / previous hit point
    vec3 viewDir;
    // Did this ray hit a target
    bool hit;
}};

const float PI = 3.14159265359;


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

CollisionInfo getCollision(vec3 ray, vec3 rd) {{
    CollisionInfo collisionInfo;

    // Default values if we miss everything
    collisionInfo.hit = false;
    collisionInfo.materialId = 0u;
    collisionInfo.surfaceNormal = vec3(0.0, 1.0, 0.0);
    collisionInfo.hitPoint = ray;
    collisionInfo.viewDir = -rd;

    {collision_checks}

    return collisionInfo;
}}

// SDF
float sdf_scene(vec3 ray) {{
    return {scene_unified_sdf};
}}

// Shamelessly yoinked from https://learnopengl.com/PBR/Lighting
// ----------------------------------------------------------------------------
float DistributionGGX(vec3 N, vec3 H, float roughness)
{{
    float a = roughness*roughness;
    float a2 = a*a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return nom / denom;
}}

float GeometrySchlickGGX(float NdotV, float roughness)
{{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float nom   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}}

vec3 fresnelSchlick(float cosTheta, vec3 F0)
{{
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cosTheta, 0.0, 1.0), 5.0);
}}

// Frankensteining pieces of the
vec3 evaluateBRDF(Material mat,vec3 N,vec3 V, vec3 lightDir,vec3 lightRadiance, vec3 F0){{
    vec3 H = normalize(V + lightDir);
    float NdotL = dot(N, lightDir);

    if (NdotL <= 0.0) {{ // let's not calculate anything if the surface faces away from the light
        return vec3(0.0);
    }}


    float NDF = DistributionGGX(N, H, mat.roughness);
    float G   = GeometrySmith(N, V, lightDir, mat.roughness);
    vec3 F    = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0);

    vec3 numerator    = NDF * G * F;
    // TODO: try with min(0.0001)
    float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, lightDir), 0.0) + 0.0001; // + 0.0001 to prevent divide by zero
    vec3 specular = numerator / denominator;

    // kS is equal to Fresnel
    vec3 kS = F;
    // for energy conservation, the diffuse and specular light can't
    // be above 1.0 (unless the surface emits light); to preserve this
    // relationship the diffuse component (kD) should equal 1.0 - kS.
    // multiply kD by the inverse metalness such that only non-metals
    // have diffuse lighting, or a linear blend if partly metal (pure metals
    // have no diffuse light).
    vec3 kD = (vec3(1.0) - kS) * (1.0 - mat.metallic);

    vec3 diffuse = kD * mat.albedo / PI;

    return (diffuse + specular) * lightRadiance * NdotL;

}}
// ----------------------------------------------------------------------------




vec3 calculateColor(CollisionInfo collision[NUM_RAYS][MAX_REFLECTION], vec3 bgColor) {{
    vec3 finalColor = vec3(0.0);
    vec3 remaining = vec3(1.0);

    uint ray = 0;

    // TODO: uniform or something
    vec3 lightDir = normalize(vec3(0.5, 0.75, -0.4));
    vec3 lightRadiance = vec3(1.0, 0.93, 0.83) * 3.0; // somewhat sunny, multiplied by 3 for HDR
    vec3 ambientRadiance = vec3(0.03);

    for (int i = 0; i < MAX_REFLECTION; i++) {{
        // If this bounce missed everything, add the sky color multiplied
        // by the current light throughput, and terminate.
        if (!collision[ray][i].hit) {{
            finalColor += remaining * bgColor;
            break;
        }}

        Material mat = MATERIAL_LIST[collision[ray][i].materialId];
        vec3 N =collision[ray][i].surfaceNormal;
        vec3 V =collision[ray][i].viewDir;

        // 4% reflectance seems to be a common value for dielectric materials so we use that as base.
        //water is around 0.02,glass/plastic around 0.04, dense glass 0.05. If I end up really tweaking the values, maybe I'll use 0.2.
        //
        vec3 F0 = mix(vec3(0.04), mat.albedo, mat.metallic);
        vec3 direct = evaluateBRDF(mat, N, V, lightDir, lightRadiance, F0);
        vec3 ambient = ambientRadiance * mat.albedo * (1.0 - mat.metallic);
        vec3 surfaceColor = direct + ambient + mat.emission;
        finalColor += remaining * surfaceColor;
        vec3 F = fresnelSchlick(max(dot(N, V), 0.0), F0);
        remaining *= mix(F, mat.albedo * F, mat.metallic) * (1.0 - mat.roughness);

        // If there is not enough light remaining for the ray we stop there
        if (dot(remaining, remaining) < 0.0001) {{
            break;
        }}
    }}

    return finalColor;
}}

vec3 raymarch(vec3 ro, vec3 rd) {{
    CollisionInfo collision[NUM_RAYS][MAX_REFLECTION];

    for (int ray = 0; ray < NUM_RAYS; ray++) {{
        for (int j = 0; j < MAX_REFLECTION; j++) {{
            collision[ray][j].hit = false;
        }}

        for (int reflection = 0; reflection < MAX_REFLECTION; reflection++) {{
            bool hitThisBounce = false;
            float total_dist = 0.0;

            for (int i = 0; i < MAX_STEPS; i++) {{
                vec3 p = ro + rd * total_dist;
                float dist = sdf_scene(p);

                if (dist <= SURF_DIST) {{
                    collision[ray][reflection] = getCollision(p, rd);
                    collision[ray][reflection].hit = true;
                    hitThisBounce = true;

                    rd = reflect(rd, collision[ray][reflection].surfaceNormal);
                    ro = p + collision[ray][reflection].surfaceNormal * NUDGE_AFTER_BOUNCE;
                    break;
                }}

                total_dist += dist;

                if (total_dist >= MAX_DIST) {{
                    break;
                }}
            }}

            if (!hitThisBounce) {{
                break;
            }}
        }}
    }}

    return calculateColor(collision, vec3(0.21, 0.27, 0.31));
}}

void main() {{
    vec2 uv = (gl_FragCoord.xy - 0.5 * screen_size) / screen_size.y;

    vec3 ray_origin = camera_position;
    vec3 fragment_direction = normalize(vec3(uv.x, uv.y, -1.0));
    vec3 ray_direction = rotateVector(fragment_direction);

    color = vec4(raymarch(ray_origin, ray_direction), 1.0);
}}
