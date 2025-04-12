use super::tensor::Tensor;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Debug)]
pub enum ActivationFunction {
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
}

pub struct NeuralNetwork {
    layers: Vec<Layer>,
    name: String,
}

impl NeuralNetwork {
    pub fn new(name: &str) -> Self {
        NeuralNetwork {
            layers: Vec::new(),
            name: String::from(name),
        }
    }
    
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }
    
    pub fn forward(&self, input: Tensor) -> Tensor {
        let mut current = input;
        
        for layer in &self.layers {
            current = layer.forward(current);
        }
        
        current
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct Layer {
    weights: Tensor,
    bias: Tensor,
    activation: ActivationFunction,
}

impl Layer {
    pub fn new(input_size: usize, output_size: usize, activation: ActivationFunction) -> Self {
        // Inicialización simple para demostración
        let weights = Tensor::zeros(&[input_size, output_size]);
        let bias = Tensor::zeros(&[output_size]);
        
        Layer {
            weights,
            bias,
            activation,
        }
    }
    
    pub fn forward(&self, input: Tensor) -> Tensor {
        // z = input * weights + bias
        let z = input.matmul(&self.weights).add(&self.bias);
        
        // Aplicar función de activación
        match self.activation {
            ActivationFunction::ReLU => z.relu(),
            ActivationFunction::Sigmoid => z.sigmoid(),
            ActivationFunction::Tanh => z.tanh(),
            ActivationFunction::Softmax => z.softmax(),
        }
    }
}