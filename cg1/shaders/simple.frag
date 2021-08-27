#version 430 core

// In: any additional inputs from vertex shader
// Out: vec4 with color
out vec4 color;
// layout(location=1) out vec4 colour;

void main()
{
    color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}