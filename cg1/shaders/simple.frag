#version 460 core

// In: any additional inputs from vertex shader
// Out: vec4 with color

// Change color over time
layout(location = 0) out vec4 color;

// uniform variable
// uniform vec4 u_Color; 

void main()
{   
    // Color red on the RGBA
    color = vec4(1.0f, 0.5f, 0.0f, 1.0f);
    // color = u_Color
}
