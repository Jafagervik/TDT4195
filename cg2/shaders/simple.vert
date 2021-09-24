#version 460 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 vertexColor;

out vec4 fragmentColor;
uniform mat4 transformation;
uniform mat4 time;

void main()  
{
    // a scales to smaller the less the value
    // d turns right side more up, looks like translate
    // b moves top vertices more to the right the lesser the value
    // e scales down, mostly height
    // c translates to right
    // f translates upwards

    /*
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
}

