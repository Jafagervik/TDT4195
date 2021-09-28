#version 460 core

// uniform float opacity;

// Change color over time
in vec4 fragmentColor;
in vec3 fragmentNormal;
out vec4 color;

void main()
{   
    color = vec4(fragmentNormal, 1.0);
}
