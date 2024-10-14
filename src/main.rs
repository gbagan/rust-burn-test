use burn::prelude::*;
use burn::backend::candle::{Candle, CandleDevice};
use burn::tensor::Tensor;
//use safetensors::SafeTensors;
use burn_import::pytorch::{LoadArgs, PyTorchFileRecorder};
use burn::record::{FullPrecisionSettings, Recorder};
use rust_burn_test::model::*;
use rust_burn_test::game_tensor::*;

type B = Candle<f32, i64>;

fn main() {
    let device = CandleDevice::default();

    //type MyBackend = NdArray;
    //let device = NdArrayDevice::default();
    

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
    println!("loading pick_sl.pt");
    let load_args = LoadArgs::new("./tensors/pick_sl.pt".into()).with_debug_print();
    let record = PyTorchFileRecorder::<FullPrecisionSettings>::new()
        .load(load_args, &device)
        .expect("Should decode state successfully");
    println!("creating the model");
    let pick_model: PickModel<B> = PickModel::new(&device).load_record(record);
    let t = Tensor::zeros([1, 300, 48], &device);
    println!("forward");
    println!("dims {:?}", pick_model.forward(t).dims());

    println!("test");
    let arr = suit_array();
    let (dimx, dimy) = arr.dim();
    let flat_arr: Vec<f32> = arr
        .outer_iter().flat_map(|row| row.to_vec())
        .collect();

    let t = Tensor::<B,1>::from_data(flat_arr.as_slice(), &device)
        .reshape([dimx, dimy]);
    let t2 = Tensor::cat(vec!(t.clone(), t.clone()), 0);

    println!("{t2}");

    //let t: Tensor<MyBackend, 3> = Tensor::from_data([[[1, 2, 4], [3, 4, 8]], [[5, 6, 7], [7, 8, 16]]], &device);
    //println!("{t}");
    //let t = layer_norm(t, 2, 1e-5);
    //println!("{t}");

    /* 
    let recorder = PrettyJsonFileRecorder::<FullPrecisionSettings>::new();
    pick_model
        .save_file("tensors/test", &recorder)
        .expect("Should be able to save the model");
    */
}