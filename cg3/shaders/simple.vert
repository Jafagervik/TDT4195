#version 460 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 vertexColor;
layout(location = 2) in vec3 normal;

out vec4 fragmentColor;
out vec3 fragmentNormal;
out vec3 fragmentPosition;

uniform float time;
uniform mat4 modelMatrix;
uniform mat4 MVP;

vec4 orig_position;

void main()  
{
    orig_position = vec4(position, 1.0);
    gl_Position = MVP*orig_position;
    fragmentPosition = vec3(modelMatrix * orig_position);
    fragmentColor = vertexColor;
    fragmentNormal = normalize(mat3(modelMatrix)*normal);
}

