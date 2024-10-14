use burn::nn::{
    conv::{Conv1d, Conv1dConfig},
    transformer::{TransformerEncoder, TransformerEncoderConfig, TransformerEncoderInput},
    Relu
};
use burn::prelude::*;
use burn::tensor::Tensor;
//use safetensors::SafeTensors;

fn layer_norm<B: Backend, const D: usize>(x: Tensor<B, D>, dim: usize, eps: f32) -> Tensor<B, D> {
    let (variance, mean) = x.clone().var_mean_bias(dim);
    (x - mean) / (variance + eps).sqrt()
}

#[derive(Module, Debug)]
struct EncoderBlock<B: Backend> {
    f1: Conv1d<B>,
    f2: Conv1d<B>,
    //layernorm: LayerNorm<B>,
    attn_encoder: TransformerEncoder<B>,
}

impl<B: Backend> EncoderBlock<B> {
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 3> {
        let x = self.f1.forward(x);
        let x = Relu.forward(x);
        let x = self.f2.forward(x);
        let x = layer_norm(x, 2, 0.0001);
        let x = x.permute([2, 0, 1]);
        let x = self.attn_encoder.forward(TransformerEncoderInput::new(x));
        x.permute([1, 2, 0])
    }
}

#[derive(Config, Debug)]
struct EncoderBlockConfig {
    n_input: usize,
    n_emb: usize,
    n_fw: usize,
    n_heads: usize,
    n_layers: usize,
}

impl EncoderBlockConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> EncoderBlock<B> {
        EncoderBlock {
            f1: Conv1dConfig::new(self.n_input, self.n_fw, 1).init(device),
            f2: Conv1dConfig::new(self.n_fw, self.n_emb, 1).init(device),
            //layernorm: LayerNormConfig::new(2).init(device), // todo
            attn_encoder: TransformerEncoderConfig::new(
                self.n_emb,
                self.n_fw,
                self.n_heads,
                self.n_layers,
            )
            .init(device),
        }
    }
}

const N_INPUT: usize = 300;
const N_EMB: usize = 256;
const N_FW: usize = 512;
const N_HEADS: usize = 4;
const N_LAYERS: usize = 2;

#[derive(Module, Debug)]
pub struct DiscardModel<B: Backend> {
    encoder_block: EncoderBlock<B>,
    out: Conv1d<B>,
}

impl<B: Backend> DiscardModel<B> {
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let x = self.encoder_block.forward(x);
        self.out.forward(x).squeeze(1)
    }

    pub fn new(device: &B::Device) -> Self {
        Self {
            encoder_block: EncoderBlockConfig::new(N_INPUT, N_EMB, N_FW, N_HEADS, N_LAYERS)
                .init(device),
            out: Conv1dConfig::new(N_EMB, 1, 1)
                .init(device),
        }
    }
}

#[derive(Module, Debug)]
pub struct PickModel<B: Backend> {
    encoder_block: EncoderBlock<B>,
    out: Conv1d<B>,
}

impl<B: Backend> PickModel<B> {
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let x = self.encoder_block.forward(x);
        let x = self.out.forward(x);
        x.squeeze(1)
    }

    pub fn new(device: &B::Device) -> Self {
        Self {
            encoder_block: EncoderBlockConfig::new(N_INPUT, N_EMB, N_FW, N_HEADS, N_LAYERS)
                .init(device),
            out: Conv1dConfig::new(N_EMB, 1, 1)
                .init(device),
        }
    }
}

#[derive(Module, Debug)]
pub struct KoiKoiModel<B: Backend> {
    encoder_block: EncoderBlock<B>,
    out: Conv1d<B>,
}

impl<B: Backend> KoiKoiModel<B> {
    pub fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let x = self.encoder_block.forward(x);
        let dims = x.dims();
        self.out.forward(x.slice([0..dims[0], 0..dims[1], 0..2])).squeeze(1)
    }

    pub fn new(device: &B::Device) -> Self {
        Self {
            encoder_block: EncoderBlockConfig::new(N_INPUT, N_EMB, N_FW, N_HEADS, N_LAYERS)
                .init(device),
            out: Conv1dConfig::new(N_EMB, 1, 1)
                .init(device),
        }
    }
}