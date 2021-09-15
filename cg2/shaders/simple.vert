#version 460 core

// Out: a vector4 containing the positons generateb by the vertex shader
layout(location = 0) in vec3 position;
mat4 MVP;
MVP[0] = vec4(1.0, 0.0, 0.0, 0.0);
MVP[1] = vec4(0.0, 1.0, 0.0, 0.0);
MVP[2] = vec4(1.0, 0.0, 1.0, 0.0);
MVP[3] = vec4(1.0, 0.0, 0.0, 1.0);

void main()
{
    gl_Position = MVP * vec4(position, 1.0);
}

