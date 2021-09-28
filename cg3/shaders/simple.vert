#version 460 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 vertexColor;
layout(location = 2) in vec3 normal;

out vec4 fragmentColor;
out vec3 fragmentNormal;
uniform mat4 transformation;
uniform mat4 time;

void main()  
{
    // a scales to smaller the less the value
    // d turns right side more up, looks like shear
    // b moves top vertices more to the right the lesser the value, shear
    // e scales down, mostly height
    // c translates to right
    // f translates upwards

    /* Task 3a - used to take in 6 uniforms, now removed
    mat4 MVP;
    MVP[0] = vec4(aVal, dVal, 0.0, 0.0);
    MVP[1] = vec4(bVal, eVal, 0.0, 0.0);
    MVP[2] = vec4(0.0, 0.0, 1.0, 0.0);
    MVP[3] = vec4(cVal, fVal, 0.0, 1.0);
    */

    // gl_Position = vec4(position, 1.0);
    vec4 orig_position = vec4(position, 1.0);
    gl_Position = transformation*orig_position;
    fragmentColor = vertexColor;
    fragmentNormal = normal;
}

