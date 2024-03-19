#version 450 core

layout (location = 0) in vec4 in_Color;

layout (location = 0) out vec4 out_Accum; // (RGB * Weight, Alpha * Weight)
layout (location = 1) out float out_Reveal;

layout (set = 0, binding = 0) uniform CameraUniformLayout {
    mat4 m_ProjView;
    vec3 m_Position;
} u_Camera;

void main() {
    vec4 a_Color = in_Color;

    // Weight Function. See also (http://casual-effects.blogspot.com/2015/03/implemented-weighted-blended-order.html)
    float a_Weight = clamp(pow(min(1.0, a_Color.a * 10.0) + 0.01, 3.0) * 1e8 * pow(1.0 - gl_FragCoord.z * 0.9, 3.0), 1e-2, 3e3);
	
    out_Accum = vec4(a_Color.rgb * a_Color.a, a_Color.a) * a_Weight;
    out_Reveal = a_Color.a;
}
