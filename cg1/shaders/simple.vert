#version 460 core

// In: 
// Out: a vector4 containing the positons generateb by the vertex shader
layout(location = 0) in vec3 position;

void main()
{
    gl_Position = vec4(position, 1.0);
}


/* FOR CIRCLE 

attribute vec2 value;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;
varying vec2 val;
void main() {
    val = value;
    gl_Position = projectionMatrix*viewMatrix*vertex;
}

*/