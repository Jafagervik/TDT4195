---
# These are meta-variables defined with YAML syntax, change them as you wish.
# see https://pandoc.org/MANUAL.html#variables
title: TDT4195 Assignment 1
author:
  - Jørgen Aleksander Fagervik
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=4cm
toc: false
toc-title: 'List of Contents'
toc-depth: 2
numbersections: true
colorlinks: true
links-as-notes: true
# The document is following the break written using Markdown syntax
---

# TDT4195: Visual Computong Dundamentals

## Computer Graphics Assignment 1

1.  ![](images/o1c.png)

2.  a)![](images/o2a.png)
    Exercise 2a
    i) The name of this phenomenon is clipping.
    ii) It occurs after the assembly and vertex shader(simple.vert in this case) stages.
    iii) The purpose of this is to discard/remove the primitives where vertices are outside bounds (normailized space viewing volume). The ones that are inside go through to the next stage in the pipeline. I knew that openGL uses a coordinate system for [-1.0,1.0] for all axes, and "manully clipping" the coordinates above or below the bounds made the shape now look like a triangle.

b) ![](images/o2b.png)
Exercise 2b
i) This phenomenon is also known as back face culling. It disappears.
ii) It happens due to the direction the normal of the vertices are pointing to. Certain orderings of the index buffer makes opengl think it follows a clockwise order, thus the normal points away from the screen, and the triangle then dissapears.
iii) The ordering of the index buffer, following the use of the right hand rule, needs to make the normal of e.g a triangle points outward toward the screen to avoid back face culling.

c)
Exercise 2c
i) The reason that the depth buffer needs to be reset each frame is to avoid comparing the new pixels with the depth values from last frame.
ii) If you have multiple overlapping shapes, you could use the fragment shader multiple times for the same pixel.
iii) Fragment/pixel shader, which is responsible for giving colors to each pixel. The second common shader is the vertex shader, which is responsible for determining positions of elements on the screen.
iv) The index buffer is used to optimize how we draw shapes. We use triangles in graphics since it requires the least amounts of vertices to achive a plane with a normal,and lets say we want to draw a square: then we can use two of our previous vertices to draw it. we use bottom left and top right again, and only need to specify top left.
Index buffer for this would be:

```rust
// Example of index buffer for a square, using only 4 vertices instead of 6.
let indexBuffer: Vec<f32> = vec![0,1,2,1,2,3];
```

v) We would pass in a non zero pointer when we want to start from an offset in our buffer. Perhaps our buffer contains other information prior to the vertex info.
Exercise 3 3. ![
    Colored Square using only 4 indices in the index buffer - 3e)
](images/o3square.png)
![
    Colored triangle which I'm reading in from a file - 3f)
](images/o3f.png)

Also have commented out how I used uniform variable to change the color of the drawn triangle(s)/square, but this isn't quite stable and crashes "randomly".
