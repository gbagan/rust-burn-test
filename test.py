import torch
import torch.nn.functional as F

a = torch.Tensor([[[1, 2, 4], [3, 4, 8]], [[5, 6, 7], [7, 8, 16]]])
b = F.layer_norm(a, [3])
print(b)

def layer_norm(x, dim, eps):
    mean = x.mean(dim)
    variance = x.var(dim)
    return (x - mean) / (variance + eps).sqrt()

print(layer_norm(a, [2], 1e-5))