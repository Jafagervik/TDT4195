#version 460 core

// Change color over time
in vec4 fragmentColor;
out vec4 color;



void main()
{   
    // Color red on the RGBA
    color = fragmentColor;
}
