#version 450 core

const float EPSILON = 1.192092896e-07f;

layout (location = 0) out vec4 out_FragColor;

layout (set = 0, binding = 0) uniform texture2D t_Accum;
layout (set = 0, binding = 1) uniform sampler s_Accum;
layout (set = 0, binding = 2) uniform texture2D t_Reveal;
layout (set = 0, binding = 3) uniform sampler s_Reveal;

bool isApproximatelyEqual(float a, float b) {
    return abs(a - b) <= max(abs(a), abs(b)) * EPSILON;
}

void main() {
    ivec2 a_Coords = ivec2(gl_FragCoord.xy);
    float a_Revealage = texelFetch(sampler2D(t_Reveal, s_Reveal), a_Coords, 0).r;
    if (isApproximatelyEqual(a_Revealage, 1.0f)) 
        discard;

    vec4 a_Accumulation = texelFetch(sampler2D(t_Accum, s_Accum), a_Coords, 0);
    if (isinf(max(max(abs(a_Accumulation.x), abs(a_Accumulation.y)), abs(a_Accumulation.z))))
        a_Accumulation.rgb = vec3(a_Accumulation.a);

    vec3 a_AverageColor = a_Accumulation.rgb / max(a_Accumulation.a, EPSILON);
    out_FragColor = vec4(a_AverageColor, 1.0f - a_Revealage);
}
