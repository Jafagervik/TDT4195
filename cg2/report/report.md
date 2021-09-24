---
# These are meta-variables defined with YAML syntax, change them as you wish.
# see https://pandoc.org/MANUAL.html#variables
title: TDT4195 Assignment 2
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

i) The name of this phenomenon is clipping.
ii) It occurs after the assembly and vertex shader(simple.vert in this case) stages.
iii) The purpose of this is to discard/remove the primitives where vertices are outside bounds (normailized space viewing volume). The ones that are inside go through to the next stage in the pipeline. I knew that openGL uses a coordinate system for [-1.0,1.0] for all axes, and "manully clipping" the coordinates above or below the bounds made the shape now look like a triangle.

b) ![](images/o2b.png)

i) The triangle completely disappears if I give it the index buffer [1, 0, 2], [0, 2, 1] or [2, 1, 0]
ii) It happens due to
iii) The effect occurs when the indices 0 2 and 1 come after each other, despite which one you start from. The rule is that "".

c)
i) The reason that the depth buffer needs to be reset each frame is to avoid comparing the new pixels with the depth values from last frame.
ii)
iii) Fragment/pixel shader, which is responsible for giving colors to each pixel. The second common shader is the vertex shader, which is responsible for determining positions of elements on the screen.
iv) The index buffer is used to optimize how we draw shapes. We use triangles in graphics since it requires the least amounts of vertices to achive a plane with a normal,
and lets say we want to draw a square: then we can use two of our previous vertices to draw it. we use bottom left and top right again, and only need to specify top left.
Index buffer for this would be [0, 1, 2, 2, 3, 0]. we now need less vertices.
v) We would pass in a non zero pointer when we want to start from an offset in our buffer. Perhaps our buffer contains other information prior to the vertex info.
How would I call VertexAttribPointer

3.  ![
    Colored Square using only 4 indices in the index buffer - 3e)
](images/o3square.png)
    ![
    Colored triangle which I'm reading in from a file - 3f)
](images/o3f.png)

Also have commented out how I used uniform variable to change the color of the drawn triangle(s)/square, but this isn't quite stable and crashes "randomly".

## Exercise 1
b) 3 different triangles with different z-indices and colors
![](images/o1b1.png)
![](images/o1b2.png)
![](images/o1b3.png)


## Exercise 2
a)
image of alpha blending
![](images/o2a.png)


b) 
i) Blended colors and exchanging triangles
![](images/o2b1.png)

ii) z index Blended colors and exchanging z-coordinates
![](images/o2b2.png)



## Exercise 3
b) a = scale, gets smaller the lesser the value | axis = x, b = shear | axis = y, c = translation, to right | axis = none, d = shear, looked like right side got turnt more up | axis = x, e = scale, height | axis = y, f = translates upwards | axis = none

c) To have rotation we have to have [cos ø, sin ø]^T , [-sin ø, cos ø]^T in some way or another in the matrix. For Rot_x, e doesn't have to be 1, and sin ø and - sin ø doesn't both give 0. Rot_y: e don't have to be 1, and since sin ø have to be 1, a have to be 1, but cos90 is 1, and this can't be true. Rot_z: d and f would have to be 0.
## Exercise 5
b)
d)