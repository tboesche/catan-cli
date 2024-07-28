use tch::{nn, nn::Module, nn::OptimizerConfig, Device, Tensor};

use crate::{ai::simple_nn::encode::{self, encode_action}, backend::{round::action::Action, setup::game::Game}};

#[derive(Debug)]
pub struct SimpleNN {
    layer1: nn::Linear,
    layer2: nn::Linear,
    layer3: nn::Linear,
    layer4: nn::Linear,
    layer5: nn::Linear,
    layer6: nn::Linear,
    output_layer: nn::Linear,
}

impl Clone for SimpleNN {
    fn clone(&self) -> Self {

        let layer1_clone: nn::Linear;
        if let Some(bs) = &self.layer1.bs {
            layer1_clone = nn::Linear{
                ws: self.layer1.ws.shallow_clone(),
                bs: Some(bs.shallow_clone())
            };
        } else {
            layer1_clone = nn::Linear{
                ws: self.layer1.ws.shallow_clone(),
                bs: None
            };
        }

       

        let layer2_clone: nn::Linear;
        if let Some(bs) = &self.layer2.bs {
            layer2_clone = nn::Linear{
                ws: self.layer2.ws.shallow_clone(),
                bs: Some(bs.shallow_clone())
            };
        } else {
            layer2_clone = nn::Linear{
                ws: self.layer2.ws.shallow_clone(),
                bs: None
            };
        }


        let layer3_clone: nn::Linear;
        if let Some(bs) = &self.layer3.bs {
            layer3_clone = nn::Linear{
                ws: self.layer3.ws.shallow_clone(),
                bs: Some(bs.shallow_clone())
            };
        } else {
            layer3_clone = nn::Linear{
                ws: self.layer3.ws.shallow_clone(),
                bs: None
            };
        }


        let layer4_clone: nn::Linear;
        if let Some(bs) = &self.layer4.bs {
            layer4_clone = nn::Linear{
                ws: self.layer4.ws.shallow_clone(),
                bs: Some(bs.shallow_clone())
            };
        } else {
            layer4_clone = nn::Linear{
                ws: self.layer4.ws.shallow_clone(),
                bs: None
            };
        }


        let layer5_clone: nn::Linear;
        if let Some(bs) = &self.layer5.bs {
            layer5_clone = nn::Linear{
                ws: self.layer5.ws.shallow_clone(),
                bs: Some(bs.shallow_clone())
            };
        } else {
            layer5_clone = nn::Linear{
                ws: self.layer5.ws.shallow_clone(),
                bs: None
            };
        }


        let layer6_clone: nn::Linear;
        if let Some(bs) = &self.layer6.bs {
            layer6_clone = nn::Linear{
                ws: self.layer6.ws.shallow_clone(),
                bs: Some(bs.shallow_clone())
            };
        } else {
            layer6_clone = nn::Linear{
                ws: self.layer6.ws.shallow_clone(),
                bs: None
            };
        }

        let output_layer_clone: nn::Linear;
        if let Some(bs) = &self.output_layer.bs {
            output_layer_clone = nn::Linear{
                ws: self.output_layer.ws.shallow_clone(),
                bs: Some(bs.shallow_clone())
            };
        } else {
            output_layer_clone = nn::Linear{
                ws: self.output_layer.ws.shallow_clone(),
                bs: None
            };
        }

        Self { layer1: layer1_clone, layer2: layer2_clone, layer3: layer3_clone, layer4: layer4_clone, layer5: layer5_clone, layer6: layer6_clone, output_layer: output_layer_clone }
    }
}

impl SimpleNN {
    pub fn new(vs: &nn::Path, input_size: i64, num_classes: i64) -> SimpleNN {
        let layer1 = nn::linear(vs / "layer1", input_size, 1024, Default::default());
        let layer2 = nn::linear(vs / "layer2", 1024, 512, Default::default());
        let layer3 = nn::linear(vs / "layer3", 512, 256, Default::default());
        let layer4 = nn::linear(vs / "layer4", 256, 128, Default::default());
        let layer5 = nn::linear(vs / "layer5", 128, 64, Default::default());
        let layer6 = nn::linear(vs / "layer6", 64, 32, Default::default());
        let output_layer = nn::linear(vs / "output_layer", 32, num_classes, Default::default());
        SimpleNN {
            layer1,
            layer2,
            layer3,
            layer4,
            layer5,
            layer6,
            output_layer,
        }
    }
}

impl nn::Module for SimpleNN {
    fn forward(&self, xs: &Tensor) -> Tensor {
        xs.view([-1, 2368])
            .apply(&self.layer1).relu()
            .apply(&self.layer2).relu()
            .apply(&self.layer3).relu()
            .apply(&self.layer4).relu()
            .apply(&self.layer5).relu()
            .apply(&self.layer6).relu()
            .apply(&self.output_layer)
            .view([-1, 1])
    }
}


pub fn evaluate_actions(game: &Game, legal_actions: &Vec<Action>) -> Vec<f32> {

    // Set the device (use CUDA if available)
    let device = if tch::Cuda::is_available() {
        Device::Cuda(0)
    } else {
        Device::Cpu
    };

    // Load the model
    let mut vs = nn::VarStore::new(device);
    let model = SimpleNN::new(&vs.root(), 2368, 1);
    let model_path = "src/ai/simple_nn/weights_short.safetensors";
    vs.load(model_path).expect("PyTorch model could not be loaded");

    let encoded_actions: Vec<Vec<u32>> = legal_actions.iter()
                                                        .map(|action| {
                                                            let encoded = encode_action(game, action);
                                                            encoded[1..].to_vec()
                                                        })
                                                        .collect();

    let formated_actions: Vec<Vec<f32>> = encoded_actions.iter()
                            .map(|vu| vu.iter().map(|&u| u as f32).collect::<Vec<f32>>())
                            .collect();

    formated_actions.iter()
                .map(|vu| model.forward(&Tensor::from_slice(vu).to(device)).double_value(&[0]) as f32)
                .collect()
}