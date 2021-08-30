#version 460 core

// In: any additional inputs from vertex shader
// Out: vec4 with color
layout(location = 0) out vec4 color;

void main()
{   
    // Color red on the RGBA
    color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}