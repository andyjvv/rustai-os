// src/ai/tensor.rs - Implementación de tensores
pub type TensorId = u64;

pub struct Tensor {
    id: TensorId,
    shape: Vec<usize>,
    strides: Vec<usize>,
    data: Vec<f32>,
    requires_grad: bool,
    grad: Option<Box<Tensor>>,
}

impl Tensor {
    pub fn new(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        let mut strides = Vec::with_capacity(shape.len());
        let mut stride = 1;
        
        for &dim in shape.iter().rev() {
            strides.push(stride);
            stride *= dim;
        }
        
        strides.reverse();
        
        static mut NEXT_ID: TensorId = 0;
        let id = unsafe {
            NEXT_ID += 1;
            NEXT_ID
        };
        
        Self {
            id,
            shape,
            strides,
            data: vec![0.0; size],
            requires_grad: false,
            grad: None,
        }
    }
    
    pub fn from_data(shape: Vec<usize>, data: Vec<f32>) -> Self {
        let mut tensor = Self::new(shape);
        tensor.data = data;
        tensor
    }
    
    pub fn zeros(shape: Vec<usize>) -> Self {
        Self::new(shape)
    }
    
    pub fn ones(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        Self::from_data(shape, vec![1.0; size])
    }
    
    pub fn random(shape: Vec<usize>) -> Self {
        let size: usize = shape.iter().product();
        let mut data = Vec::with_capacity(size);
        
        // Generador de números pseudoaleatorios simple
        let mut seed = 42;
        for _ in 0..size {
            seed = (seed * 1103515245 + 12345) & 0x7fffffff;
            data.push((seed as f32) / (0x7fffffff as f32));
        }
        
        Self::from_data(shape, data)
    }
    
    pub fn set_requires_grad(&mut self, requires_grad: bool) {
        self.requires_grad = requires_grad;
        if requires_grad && self.grad.is_none() {
            self.grad = Some(Box::new(Self::zeros(self.shape.clone())));
        }
    }
    
    pub fn backward(&mut self) {
        if !self.requires_grad {
            return;
        }
        
        // Implementación simplificada
        if let Some(ref mut grad) = self.grad {
            // Inicializar gradiente a 1.0 para el tensor raíz
            if grad.data.iter().all(|&x| x == 0.0) {
                grad.data = vec![1.0; grad.data.len()];
            }
        }
        
        // Aquí iría la propagación real del gradiente
    }
}
