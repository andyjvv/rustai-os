use super::nn::NeuralNetwork;
use super::tensor::Tensor;
use alloc::vec::Vec;
use alloc::string::String;

pub struct InferenceEngine {
    models: Vec<NeuralNetwork>,
    current_model: Option<usize>,
}

impl InferenceEngine {
    pub fn new() -> Self {
        InferenceEngine {
            models: Vec::new(),
            current_model: None,
        }
    }
    
    pub fn load_model(&mut self, model: NeuralNetwork) -> usize {
        let model_id = self.models.len();
        self.models.push(model);
        self.current_model = Some(model_id);
        model_id
    }
    
    pub fn set_current_model(&mut self, model_id: usize) -> Result<(), &'static str> {
        if model_id < self.models.len() {
            self.current_model = Some(model_id);
            Ok(())
        } else {
            Err("ID de modelo no válido")
        }
    }
    
    pub fn predict(&self, input: Tensor) -> Result<Tensor, &'static str> {
        match self.current_model {
            Some(model_id) => {
                let model = &self.models[model_id];
                Ok(model.forward(input))
            },
            None => Err("No hay modelo seleccionado para inferencia"),
        }
    }
    
    pub fn get_model_names(&self) -> Vec<String> {
        self.models.iter()
            .map(|model| String::from(model.name()))
            .collect()
    }
}