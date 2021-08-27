#version 450 core

// In: 
// Out: a vector4 containing the positons generateb by the vertex shader
in vec3 position;
// in layout(location=1) vec4 vertex; This will connect vertex to a color/fragment

void main()
{
    gl_Position = vec4(position, 1.0f);
}