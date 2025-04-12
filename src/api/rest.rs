use alloc::string::String;
use alloc::vec::Vec;
use crate::api::{ApiRequest, ApiResponse, HttpMethod};

pub struct RestEndpoint {
    path: String,
    method: HttpMethod,
    handler: fn(&RestRequest) -> RestResponse,
}

pub struct RestRequest {
    path: String,
    method: HttpMethod,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
    path_params: Vec<(String, String)>,
    query_params: Vec<(String, String)>,
}

pub struct RestResponse {
    status: u16,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

pub struct RestRouter {
    endpoints: Vec<RestEndpoint>,
}

impl RestRouter {
    pub fn new() -> Self {
        RestRouter {
            endpoints: Vec::new(),
        }
    }
    
    pub fn add_endpoint(&mut self, endpoint: RestEndpoint) {
        self.endpoints.push(endpoint);
    }
    
    pub fn route(&self, api_request: &ApiRequest) -> ApiResponse {
        // Convertir ApiRequest a RestRequest
        let rest_request = self.parse_request(api_request);
        
        // Buscar un endpoint que coincida
        for endpoint in &self.endpoints {
            if self.matches_route(&endpoint.path, &rest_request.path) && endpoint.method == rest_request.method {
                // Encontrado un endpoint que coincide, llamar al manejador
                let rest_response = (endpoint.handler)(&rest_request);
                
                // Convertir RestResponse a ApiResponse
                return ApiResponse {
                    status: rest_response.status,
                    headers: rest_response.headers,
                    body: rest_response.body,
                };
            }
        }
        
        // Si no se encuentra ningún endpoint, devolver 404
        ApiResponse {
            status: 404,
            headers: Vec::new(),
            body: Some(b"{\"error\":\"Not Found\"}".to_vec()),
        }
    }
    
    fn parse_request(&self, api_request: &ApiRequest) -> RestRequest {
        let (path_params, query_params) = self.extract_params(&api_request.path);
        
        RestRequest {
            path: api_request.path.clone(),
            method: api_request.method,
            headers: api_request.headers.clone(),
            body: api_request.body.clone(),
            path_params,
            query_params,
        }
    }
    
    fn extract_params(&self, path: &str) -> (Vec<(String, String)>, Vec<(String, String)>) {
        let mut path_params = Vec::new();
        let mut query_params = Vec::new();
        
        // Extraer parámetros de consulta
        if let Some(query_index) = path.find('?') {
            let query_part = &path[query_index + 1..];
            
            for param in query_part.split('&') {
                if let Some(eq_index) = param.find('=') {
                    let name = param[..eq_index].to_string();
                    let value = param[eq_index + 1..].to_string();
                    query_params.push((name, value));
                }
            }
        }
        
        // Nota: Los parámetros de ruta normalmente se extraerían al hacer coincidir
        // la ruta con una plantilla de ruta, pero esto se simplifica para la demostración
        
        (path_params, query_params)
    }
    
    fn matches_route(&self, route_template: &str, actual_path: &str) -> bool {
        // Simplificado: para una implementación completa, compararía segmentos de ruta
        // y extraería parámetros variables
        
        // Ignorar parámetros de consulta para la coincidencia
        let path_without_query = if let Some(query_index) = actual_path.find('?') {
            &actual_path[..query_index]
        } else {
            actual_path
        };
        
        // Para esta demostración, solo hacemos una coincidencia exacta (sin variables)
        route_template == path_without_query
    }
}

impl RestRequest {
    pub fn get_path_param(&self, name: &str) -> Option<&str> {
        self.path_params.iter()
            .find(|(param_name, _)| param_name == name)
            .map(|(_, value)| value.as_str())
    }
    
    pub fn get_query_param(&self, name: &str) -> Option<&str> {
        self.query_params.iter()
            .find(|(param_name, _)| param_name == name)
            .map(|(_, value)| value.as_str())
    }
    
    pub fn json_body<T>(&self) -> Result<T, &'static str> 
    where 
        T: serde::de::DeserializeOwned
    {
        // Nota: En un sistema real, esto utilizaría serde para deserializar JSON
        // Aquí simplemente devolvemos un error para la demostración
        Err("Deserialización JSON no implementada en esta demo")
    }
}