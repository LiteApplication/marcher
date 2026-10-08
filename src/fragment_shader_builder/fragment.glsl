#version 460

// =========================================================================
// This file is a template that will get filled in at runtime, but it is
// included at compile time so do not forget to recompile if you  modify it.
// Also it requires double curly brackets for templates reasons
// =========================================================================

#define MAX_STEPS 100
#define MAX_DIST 30.0
#define SURF_DIST (1.0/128.0)
#define SURF_DIST_TOLERENCE 1.01
#define MAX_REFLECTION 20
#define NUM_RAYS 10
#define NUDGE_AFTER_BOUNCE (SURF_DIST * 2)
#define NORMAL_CALCULATION_VECTOR vec2(0.001, 0.0)

// The collisions that can happen
{object_collision_id_definition}

uniform vec3 camera_position;
uniform mat3 camera_rotation_mat;
uniform vec2 screen_size;
uniform uint seed_time;

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

uint rngState = 0;
uint pcgHash(uint inputVal) {{
    uint state = inputVal * 747796405u + 2891336453u;
    uint word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}}

float randomFloat() {{
    rngState = pcgHash(rngState);
    return float(rngState) / 4294967295.0;
}}

vec3 cosineWeightedSample(vec3 N) {{
    float u1 = randomFloat();
    float u2 = randomFloat();

    float r = sqrt(u1);
    float theta = 2.0 * PI * u2;

    float x = r * cos(theta);
    float y = r * sin(theta);
    float z = sqrt(max(0.0, 1.0 - u1));

    vec3 up = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
    vec3 tangent = normalize(cross(up, N));
    vec3 bitangent = cross(N, tangent);

    return normalize(tangent * x + bitangent * y + N * z);
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

// Frankensteining pieces of the learnopengl code
vec3 evalBRDF(Material mat, vec3 N, vec3 V, vec3 L, vec3 F0) {{
    vec3 H = normalize(V + L);
    float NdotL = max(dot(N, L), 0.0);
    float NdotV = max(dot(N, V), 0.0);

    if (NdotL <= 0.0 || NdotV <= 0.0) {{
        return vec3(0.0);
    }}

    float NDF = DistributionGGX(N, H, mat.roughness);
    float G   = GeometrySmith(N, V, L, mat.roughness);
    vec3 F    = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0);

    vec3 numerator = NDF * G * F;
    float denominator = 4.0 * NdotV * NdotL + 0.0001;
    vec3 specular = numerator / denominator;

    vec3 kS = F;
    vec3 kD = (vec3(1.0) - kS) * (1.0 - mat.metallic);
    vec3 diffuse = kD * mat.albedo / PI;

    return diffuse + specular;
}}
// ----------------------------------------------------------------------------

// taken straight from wikipedia
vec3 raymarch(vec3 ro, vec3 rd, vec3 bgColor) {{
    vec3 throughput = vec3(1.0);
    vec3 radiance = vec3(0.0);

    for (int bounce = 0; bounce < MAX_REFLECTION; bounce++) {{
        bool hit = false;
        float total_dist = 0.0;
        CollisionInfo col;

        for (int step = 0; step < MAX_STEPS; step++) {{
            vec3 p = ro + rd * total_dist;
            float dist = sdf_scene(p);

            if (dist <= SURF_DIST) {{
                col = getCollision(p, rd);
                col.hit = true;
                hit = true;
                break;
            }}

            total_dist += dist;

            if (total_dist >= MAX_DIST) {{
                break;
            }}
        }}

        if (!hit) {{
            radiance += throughput * bgColor;
            break;
        }}

        Material mat = MATERIAL_LIST[col.materialId];

        // 1. Accumulate emissive light
        radiance += throughput * mat.emission;

        // 2. Sample new direction via cosine-weighted distribution
        vec3 N = col.surfaceNormal;
        vec3 V = -rd;
        vec3 newDir = cosineWeightedSample(N);

        // 3. Update throughput: throughput *= PI * BRDF
        vec3 F0 = mix(vec3(0.04), mat.albedo, mat.metallic);
        vec3 brdf = evalBRDF(mat, N, V, newDir, F0);
        throughput *= PI * brdf;

        // 4. Update ray state for the next bounce
        ro = col.hitPoint + N * NUDGE_AFTER_BOUNCE;
        rd = newDir;

        // 5. Russian Roulette path termination
        if (bounce > 2) {{
            float pSurvive = clamp(max(throughput.r, max(throughput.g, throughput.b)), 0.05, 0.95);
            if (randomFloat() > pSurvive) {{
                break;
            }}
            throughput /= pSurvive;
        }}
    }}

    return radiance;
}}

void main() {{
    uvec2 pixelCoord = uvec2(gl_FragCoord.xy);
    uint pixelIndex = pixelCoord.x + pixelCoord.y * uint(screen_size.x);
    rngState = pcgHash(pixelIndex ^ pcgHash(seed_time));

    vec3 background_color = vec3(0.0);
    vec3 total_radiance = vec3(0.0);

    for (int s = 0; s < NUM_RAYS; s++) {{
        vec2 jitter = vec2(randomFloat() - 0.5, randomFloat() - 0.5);
        vec2 uv = ((gl_FragCoord.xy + jitter) - 0.5 * screen_size) / screen_size.y;

        vec3 ray_origin = camera_position;
        vec3 fragment_direction = normalize(vec3(uv.x, uv.y, -1.0));
        vec3 ray_direction = rotateVector(fragment_direction);

        total_radiance += raymarch(ray_origin, ray_direction, background_color);
    }}

    vec3 col = total_radiance / float(NUM_RAYS);

    // Reinhard tonemapping and gamma correction
    col = col / (col + vec3(1.0));
    col = pow(col, vec3(1.0 / 2.2));

    color = vec4(col, 1.0);
}}
