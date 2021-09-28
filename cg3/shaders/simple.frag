#version 460 core

// uniform float opacity;

// Change color over time
in vec4 fragmentColor;
in vec3 fragmentNormal;
out vec4 color;

void main()
{   
    vec3 light_direction = normalize(vec3(0.8, -0.5, 0.6));
    vec3 c = max(vec3(0.0,0.0,0.0), fragmentNormal * -light_direction);
    vec4 lambertarian = vec4(c, 1.0);
    color = fragmentColor * lambertarian;
    // color = vec4(fragmentNormal, 1.0);
}
