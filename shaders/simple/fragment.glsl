#version 460

#define MAX_STEPS 100
#define MAX_DIST 10000.0
#define SURF_DIST 0.001
#define TAU 6.283185
#define PI 3.141592

uniform vec3 camera_position;
uniform mat3 camera_rotation_mat;
uniform vec2 screen_size;

out vec3 color;

float somevalue;

vec3 rotateVector(vec3 vec) {
    return camera_rotation_mat * vec;
}

//SDF
float getSD(vec3 p) {
    vec4 s = vec4(0, 0, 6, 2);
    float sphere_dist = length(p - s.xyz) - s.w;
    float plane_dist = p.y;
    float d = min(sphere_dist, plane_dist);

    return d;
}

float getDist(vec3 p) {
    float dist = 0;

    //SDF
    dist = getSD(p);

    return dist;
}

float raymarch(vec3 ro, vec3 rd) {
    float dist = 0.0;

    for(int i = 0; i < MAX_STEPS; i++) {
        vec3 p = ro + rd * dist;
        dist += getDist(p);

        if(dist >= MAX_DIST || dist <= SURF_DIST) {
            break;
        }
    }

    return dist;
}

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * screen_size) / screen_size.y;

    vec3 ray_origin = camera_position;
    vec3 fragment_direction = normalize(vec3(uv.x, uv.y, -1.0));
    vec3 ray_direction = rotateVector(fragment_direction);
    somevalue = 0.0;

    color = vec3(vec2(1 - raymarch(ray_origin, ray_direction) / 6.0), somevalue);
}
