#version 460 core

// In: any additional inputs from vertex shader
// Out: vec4 with color

// Change color over time
in vec4 fragmentColor;
out vec4 color;

// uniform variable
// uniform vec4 u_Color; 


void main()
{   
    // Color red on the RGBA
    color = fragmentColor;
    // color = u_Color
}
