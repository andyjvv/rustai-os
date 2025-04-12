use alloc::vec::Vec;
use core::ops::{Add, Mul};
use ndarray::{Array, ArrayD, IxDyn};

#[derive(Debug, Clone)]
pub struct Tensor {
    data: ArrayD<f32>,
}

impl Tensor {
    pub fn zeros(shape: &[usize]) -> Self {
        let data = Array::zeros(IxDyn(shape));
        Tensor { data }
    }
    
    pub fn ones(shape: &[usize]) -> Self {
        let data = Array::ones(IxDyn(shape));
        Tensor { data }
    }
    
    pub fn from_vec(values: Vec<f32>, shape: &[usize]) -> Self {
        let data = Array::from_shape_vec(IxDyn(shape), values)
            .expect("Error al crear tensor desde vector");
        Tensor { data }
    }
    
    pub fn shape(&self) -> Vec<usize> {
        self.data.shape().to_vec()
    }
    
    pub fn matmul(&self, other: &Tensor) -> Tensor {
        // Implementación simplificada para demostración
        // En un sistema real, usaría BLAS o una implementación optimizada
        let result = self.data.dot(&other.data);
        Tensor { data: result.into_dyn() }
    }
    
    pub fn add(&self, other: &Tensor) -> Tensor {
        let result = &self.data + &other.data;
        Tensor { data: result }
    }
    
    pub fn relu(&self) -> Tensor {
        let result = self.data.map(|&x| if x > 0.0 { x } else { 0.0 });
        Tensor { data: result }
    }
    
    pub fn sigmoid(&self) -> Tensor {
        let result = self.data.map(|&x| 1.0 / (1.0 + (-x).exp()));
        Tensor { data: result }
    }
    
    pub fn tanh(&self) -> Tensor {
        let result = self.data.map(|&x| x.tanh());
        Tensor { data: result }
    }
    
    pub fn softmax(&self) -> Tensor {
        // Implementación simplificada para demostración
        let max_val = self.data.fold(f32::MIN, |a, &b| a.max(b));
        let exp = self.data.map(|&x| (x - max_val).exp());
        let sum = exp.sum();
        let result = exp.map(|&x| x / sum);
        Tensor { data: result }
    }
}