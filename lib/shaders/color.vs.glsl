#version 450 core

layout (location = 0) in vec3 in_Position;
layout (location = 1) in vec3 in_Normal;

layout (location = 0) out vec4 out_Color;

layout (set = 0, binding = 0) uniform CameraUniformLayout {
    mat4 m_ProjView;
    vec3 m_Position;
} u_Camera;

layout (set = 1, binding = 0) uniform EntityUniformLayout {
    mat4 m_World;
    vec4 m_Color;
} u_Entity;

void main() {
    out_Color = u_Entity.m_Color;
    gl_Position = u_Camera.m_ProjView * u_Entity.m_World * vec4(in_Position, 1.0);
}
