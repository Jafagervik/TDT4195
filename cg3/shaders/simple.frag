#version 460 core

// uniform float opacity;

// Change color over time
in vec4 fragmentColor;
out vec4 color;

void main()
{   
    color = fragmentColor;
}
