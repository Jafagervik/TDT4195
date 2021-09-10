#version 460 core

// In: any additional inputs from vertex shader
// Out: vec4 with color

// Change color over time
layout(location = 0) out vec4 color;

// uniform variable
// uniform vec4 u_Color; 

void main()
{   
    // Color red on the RGBA
    color = vec4(1.0f, 0.5f, 0.0f, 1.0f);
    // color = u_Color
}


/* FOR CIRCLE


varying vec2 val;
void main() {
    float R = 1.0;
    float R2 = 0.5;
    float dist = sqrt(dot(val,val));
    if (dist >= R || dist <= R2) {
        discard;
    }
    float sm = smoothstep(R,R-0.01,dist);
    float sm2 = smoothstep(R2,R2+0.01,dist);
    float alpha = sm*sm2;
    gl_FragColor = vec4(0.0, 0.0, 1.0, alpha);
}



*/