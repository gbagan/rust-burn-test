import torch
from safetensors import safe_open

d_model=256

tensors = {}
with safe_open("tensors/pick_sl.safetensors", framework="pt", device="cpu") as f:
    for k in f.keys():
        k2 = k
        k2=k.replace('norm1', 'norm_1')
        k2=k2.replace('norm2', 'norm_2')
        k2=k2.replace("self_attn", "mha")
        k2=k2.replace("linear1", "pwff.linear_inner")
        k2=k2.replace("linear2", "pwff.linear_outer")
        print(k2)
        tensors[k2] = f.get_tensor(k)
    in_weight = tensors["encoder_block.attn_encoder.layers.0.mha.in_proj_weight"]
    in_bias = tensors["encoder_block.attn_encoder.layers.0.mha.in_proj_bias"]
    out_weight = tensors["encoder_block.attn_encoder.layers.0.mha.out_proj.weight"]
    out_bias = tensors["encoder_block.attn_encoder.layers.0.mha.out_proj.bias"]
    b_Q = in_bias[:d_model]
    b_K = in_bias[d_model:2*d_model]
    b_V = in_bias[2*d_model:]
    w_Q = in_weight[:d_model, :]
    w_K = in_weight[d_model:2*d_model, :]
    w_V = in_weight[2*d_model:, :]
    tensors["encoder_block.attn_encoder.layers.0.mha.query.weight"] = w_Q
    tensors["encoder_block.attn_encoder.layers.0.mha.query.bias"] = b_Q
    tensors["encoder_block.attn_encoder.layers.0.mha.key.weight"] = w_K
    tensors["encoder_block.attn_encoder.layers.0.mha.key.bias"] = b_K
    tensors["encoder_block.attn_encoder.layers.0.mha.value.weight"] = w_V
    tensors["encoder_block.attn_encoder.layers.0.mha.value.bias"] = b_V
    tensors["encoder_block.attn_encoder.layers.0.mha.output.weight"] = out_weight
    tensors["encoder_block.attn_encoder.layers.0.mha.output.bias"] = out_bias
    del tensors["encoder_block.attn_encoder.layers.0.mha.in_proj_weight"]
    del tensors["encoder_block.attn_encoder.layers.0.mha.in_proj_bias"]
    del tensors["encoder_block.attn_encoder.layers.0.mha.out_proj.weight"]
    del tensors["encoder_block.attn_encoder.layers.0.mha.out_proj.bias"]

    in_weight = tensors["encoder_block.attn_encoder.layers.1.mha.in_proj_weight"]
    in_bias = tensors["encoder_block.attn_encoder.layers.1.mha.in_proj_bias"]
    out_weight = tensors["encoder_block.attn_encoder.layers.1.mha.out_proj.weight"]
    out_bias = tensors["encoder_block.attn_encoder.layers.1.mha.out_proj.bias"]
    b_Q = in_bias[:d_model]
    b_K = in_bias[d_model:2*d_model]
    b_V = in_bias[2*d_model:]
    w_Q = in_weight[:d_model, :]
    w_K = in_weight[d_model:2*d_model, :]
    w_V = in_weight[2*d_model:, :]
    tensors["encoder_block.attn_encoder.layers.1.mha.query.weight"] = w_Q
    tensors["encoder_block.attn_encoder.layers.1.mha.query.bias"] = b_Q
    tensors["encoder_block.attn_encoder.layers.1.mha.key.weight"] = w_K
    tensors["encoder_block.attn_encoder.layers.1.mha.key.bias"] = b_K
    tensors["encoder_block.attn_encoder.layers.1.mha.value.weight"] = w_V
    tensors["encoder_block.attn_encoder.layers.1.mha.value.bias"] = b_V
    tensors["encoder_block.attn_encoder.layers.1.mha.output.weight"] = out_weight
    tensors["encoder_block.attn_encoder.layers.1.mha.output.bias"] = out_bias
    del tensors["encoder_block.attn_encoder.layers.1.mha.in_proj_weight"]
    del tensors["encoder_block.attn_encoder.layers.1.mha.in_proj_bias"]
    del tensors["encoder_block.attn_encoder.layers.1.mha.out_proj.weight"]
    del tensors["encoder_block.attn_encoder.layers.1.mha.out_proj.bias"]



torch.save(tensors, "tensors/pick_sl.pt")