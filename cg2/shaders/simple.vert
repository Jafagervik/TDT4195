#version 460 core

// Out: a vector4 containing the positons generateb by the vertex shader
layout(location = 0) in vec3 position;

uniform layout(location = 2) in vec4 aVal;
uniform layout(location = 3) in vec4 bVal;
uniform layout(location = 4) in vec4 cVal;
uniform layout(location = 5) in vec4 dVal;
uniform layout(location = 6) in vec4 eVal;
uniform layout(location = 7) in vec4 fVal;

// uniform mat4 MVP

void main()
{
    mat4 MVP;
    MVP[0] = vec4(1.0, 0.0, 0.0, 0.0);
    MVP[1] = vec4(0.0, 1.0, 0.0, 0.0);
    MVP[2] = vec4(1.0, 0.0, 1.0, 0.0);
    MVP[3] = vec4(1.0, 0.0, 0.0, 1.0);
    gl_Position = MVP * vec4(position, 1.0);
}

