import torch

# x = torch.ones(2, 2, dtype=torch.float16)
# x = torch.tensor([2.5, 0.1], dtype=float16)
x = torch.rand(2, 2)
y = torch.rand(2, 2)
y.add_(x)  # Inplace
print(y)

z = torch.mul(x, y)
print(z)
