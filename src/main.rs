use burn::nn::{
    conv::{Conv1d, Conv1dConfig},
    transformer::{TransformerEncoder, TransformerEncoderConfig, TransformerEncoderInput},
    Relu
};
use burn::prelude::*;
use burn::backend::ndarray::{NdArray, NdArrayDevice};
use burn::tensor::Tensor;
//use safetensors::SafeTensors;
use burn_import::pytorch::{LoadArgs, PyTorchFileRecorder};
use burn::record::{FullPrecisionSettings, Recorder};

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
struct DiscardModel<B: Backend> {
    encoder_block: EncoderBlock<B>,
    out: Conv1d<B>,
}

impl<B: Backend> DiscardModel<B> {
    fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let x = self.encoder_block.forward(x);
        self.out.forward(x).squeeze(1)
    }

    fn new(device: &B::Device) -> Self {
        Self {
            encoder_block: EncoderBlockConfig::new(N_INPUT, N_EMB, N_FW, N_HEADS, N_LAYERS)
                .init(device),
            out: Conv1dConfig::new(N_EMB, 1, 1)
                .init(device),
        }
    }
}

#[derive(Module, Debug)]
struct PickModel<B: Backend> {
    encoder_block: EncoderBlock<B>,
    out: Conv1d<B>,
}

impl<B: Backend> PickModel<B> {
    fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let x = self.encoder_block.forward(x);
        let x = self.out.forward(x);
        x.squeeze(1)
    }

    fn new(device: &B::Device) -> Self {
        Self {
            encoder_block: EncoderBlockConfig::new(N_INPUT, N_EMB, N_FW, N_HEADS, N_LAYERS)
                .init(device),
            out: Conv1dConfig::new(N_EMB, 1, 1)
                .init(device),
        }
    }
}

#[derive(Module, Debug)]
struct KoiKoiModel<B: Backend> {
    encoder_block: EncoderBlock<B>,
    out: Conv1d<B>,
}

impl<B: Backend> KoiKoiModel<B> {
    fn forward(&self, x: Tensor<B, 3>) -> Tensor<B, 2> {
        let x = self.encoder_block.forward(x);
        let dims = x.dims();
        self.out.forward(x.slice([0..dims[0], 0..dims[1], 0..2])).squeeze(1)
    }

    fn new(device: &B::Device) -> Self {
        Self {
            encoder_block: EncoderBlockConfig::new(N_INPUT, N_EMB, N_FW, N_HEADS, N_LAYERS)
                .init(device),
            out: Conv1dConfig::new(N_EMB, 1, 1)
                .init(device),
        }
    }
}



fn main() {
    type MyBackend = NdArray;

    let device = NdArrayDevice::default();

    //let tensor_data =
    //    std::fs::read("tensors/pick_sl.safetensors").expect("Erreur lors du chargement du fichier");

    /* 
    // Lire les tenseurs
    let tensors =
        SafeTensors::deserialize(&tensor_data).expect("Erreur lors de la désérialisation");

    // Accéder aux données du tenseur
    for (name, tensor) in tensors.tensors() {
        println!("Tenseur: {}", name);
        println!("{:?}", tensor.shape().to_vec());
        let data: Vec<f32> = match tensor.dtype() {
            safetensors::tensor::Dtype::F32 => tensor
                .data()
                .chunks(4)
                .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
                .collect(),
            _ => panic!("Type de donnée non supporté"),
        };
        println!("{}", data.len());
    }
    */

    let load_args = LoadArgs::new("./tensors/pick_sl.pt".into()).with_debug_print();
    let record = PyTorchFileRecorder::<FullPrecisionSettings>::new()
        .load(load_args, &device)
        .expect("Should decode state successfully");
    let pick_model: PickModel<MyBackend> = PickModel::new(&device).load_record(record);
    let t = Tensor::zeros([300, 300, 4], &device);
    println!("dims {:?}", pick_model.forward(t).dims());


    let t: Tensor<MyBackend, 3> = Tensor::from_data([[[1, 2, 4], [3, 4, 8]], [[5, 6, 7], [7, 8, 16]]], &device);
    println!("{t}");
    let t = layer_norm(t, 2, 1e-5);
    println!("{t}");

    /* 
    let recorder = PrettyJsonFileRecorder::<FullPrecisionSettings>::new();
    pick_model
        .save_file("tensors/test", &recorder)
        .expect("Should be able to save the model");
    */
}

/*
Tenseur: encoder_block.attn_encoder.layers.1.linear2.weight
[256, 512]  
Tenseur: encoder_block.attn_encoder.layers.1.norm2.bias
[256]
Tenseur: encoder_block.attn_encoder.layers.0.linear2.bias
[256]
Tenseur: encoder_block.attn_encoder.layers.1.self_attn.out_proj.bias
[256]
Tenseur: encoder_block.f1.weight
[512, 300, 1]
Tenseur: out.bias
[1]
Tenseur: encoder_block.attn_encoder.layers.0.self_attn.in_proj_bias
[768]
Tenseur: encoder_block.attn_encoder.layers.1.self_attn.in_proj_bias
[768]
Tenseur: encoder_block.attn_encoder.layers.0.self_attn.in_proj_weight
[768, 256]
Tenseur: encoder_block.attn_encoder.layers.1.norm1.bias
[256]
Tenseur: encoder_block.attn_encoder.layers.1.linear2.bias
[256]
Tenseur: encoder_block.attn_encoder.layers.1.norm1.weight
[256]
Tenseur: encoder_block.attn_encoder.layers.1.self_attn.in_proj_weight
[768, 256]
Tenseur: encoder_block.attn_encoder.layers.0.self_attn.out_proj.weight
[256, 256]
Tenseur: encoder_block.f2.bias
[256]
Tenseur: encoder_block.f1.bias
[512]
Tenseur: encoder_block.attn_encoder.layers.0.norm2.weight
[256]
Tenseur: encoder_block.attn_encoder.layers.1.linear1.bias
[512]
Tenseur: encoder_block.attn_encoder.layers.0.linear1.bias
[512]
Tenseur: encoder_block.attn_encoder.layers.1.norm2.weight
[256]
Tenseur: encoder_block.attn_encoder.layers.0.norm2.bias
[256]
Tenseur: encoder_block.attn_encoder.layers.0.norm1.weight
[256]
Tenseur: encoder_block.attn_encoder.layers.1.linear1.weight
[512, 256]
Tenseur: encoder_block.attn_encoder.layers.0.linear2.weight
[256, 512]
Tenseur: encoder_block.attn_encoder.layers.0.self_attn.out_proj.bias
[256]
Tenseur: encoder_block.attn_encoder.layers.0.linear1.weight
[512, 256]
Tenseur: out.weight
[1, 256, 1]
Tenseur: encoder_block.f2.weight
[256, 512, 1]
Tenseur: encoder_block.attn_encoder.layers.0.norm1.bias
[256]
Tenseur: encoder_block.attn_encoder.layers.1.self_attn.out_proj.weight
[256, 256]
*/