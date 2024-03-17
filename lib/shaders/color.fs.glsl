#version 450 core

layout (location = 0) in vec4 in_Color;

layout (location = 0) out vec4 out_FragColor;

layout (set = 0, binding = 0) uniform CameraUniformLayout {
    mat4 m_ProjView;
    vec3 m_Position;
} u_Camera;

void main() {
    out_FragColor = in_Color;
}
