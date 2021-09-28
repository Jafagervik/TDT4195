#version 460 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 vertexColor;
layout(location = 2) in vec3 normal;

out vec4 fragmentColor;
out vec3 fragmentNormal;
uniform mat4 transformation;
uniform mat4 time;

void main()  
{
    vec4 orig_position = vec4(position, 1.0);
    gl_Position = transformation*orig_position;
    fragmentColor = vertexColor;
    fragmentNormal = normal;
}

