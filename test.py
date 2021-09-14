import torch

# x = torch.ones(2, 2, dtype=torch.float16)
# x = torch.tensor([2.5, 0.1], dtype=float16)
x = torch.rand(2, 2)
y = torch.rand(2, 2)
y.add_(x)  # Inplace
print(y)

z = torch.mul(x, y)
print(z)

"""Line rasterization algorithm."""


def line_rast(xs: int, ys: int, xe: int, ye: int, c: str):
    s = (ye - ys) / (xe - xs)
    x, y = xs, ys
    while x <= xe:
        setPixel(x, y, c)
        x += 1
        y += round(s * (x - xs))


def line_rast2(xs: int, ys: int, xe: int, ye: int, c: str):
    """
    Avoid rounding operation by splitting y value into an integer and a float part e.
    """
    e = 0
    s = (ye - ys) / (xe - xs)
    x, y = xs, ys
    while x <= xe:
        setPixel(x, y, c)
        x += 1
        e += s
        if e >= 0.5:
            y += 1
            e -= 1


def bresenham(xs: int, ys: int, xe: int, ye: int, c: str):
    """Floating point variables are replaced by integers."""
    dx = xe - xs
    dy = ye - ys
    x, y = xs, ys
    e = -(dx >> 1)
    while x <= xe:
        setPixel(x, y, c)
        x += 1
        e += dy
        if e >= 0:
            y += 1
            e -= dx
