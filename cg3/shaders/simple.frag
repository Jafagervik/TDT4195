#version 460 core

// Change color over time
in vec4 fragmentColor;
in vec3 fragmentNormal;
in vec3 fragmentPosition;
uniform vec3 cameraPosition;

out vec4 color;

vec3 lightDir;

void main()
{   
    vec3 light_col = vec3(1.0f, 0.2f, 0.0f);
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength*light_col;
    lightDir = normalize(vec3(0.8, -0.5, 0.6));

    float diff = max(0.0, dot(fragmentNormal, -lightDir));
    vec3 diffuse = diff*light_col;

    vec3 view_dir = normalize(cameraPosition - fragmentPosition);
    vec3 reflect_dir = reflect(-lightDir, fragmentNormal);

    float spec = pow(max(dot(view_dir, reflect_dir), 0.0), 256);
    float specular_strength = 1.0;
    vec3 specular = specular_strength*spec*light_col;

    vec3 result = (ambient+diffuse+specular)*fragmentColor.rgb;
    color = vec4(result, 1.0f);
}
 