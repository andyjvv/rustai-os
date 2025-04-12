use alloc::string::String;
use alloc::vec::Vec;
use crate::api::{ApiRequest, ApiResponse, HttpMethod};

pub struct HttpServer {
    port: u16,
    host: String,
    running: bool,
}

impl HttpServer {
    pub fn new(host: &str, port: u16) -> Self {
        HttpServer {
            port,
            host: String::from(host),
            running: false,
        }
    }
    
    pub fn start(&mut self) -> Result<(), &'static str> {
        // En un sistema real, esto configuraría un socket TCP y comenzaría a escuchar
        self.running = true;
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    pub fn parse_request(raw_request: &[u8]) -> Option<ApiRequest> {
        // Convertir bytes a string para análisis
        if let Ok(request_str) = core::str::from_utf8(raw_request) {
            // Dividir en líneas
            let lines: Vec<&str> = request_str.split("\r\n").collect();
            
            // Verificar que hay al menos una línea para la línea de solicitud
            if lines.is_empty() {
                return None;
            }
            
            // Analizar la línea de solicitud (METHOD PATH HTTP/x.x)
            let request_parts: Vec<&str> = lines[0].split_whitespace().collect();
            if request_parts.len() < 2 {
                return None;
            }
            
            // Determinar el método HTTP
            let method = match request_parts[0] {
                "GET" => HttpMethod::GET,
                "POST" => HttpMethod::POST,
                "PUT" => HttpMethod::PUT,
                "DELETE" => HttpMethod::DELETE,
                _ => return None, // Método no soportado
            };
            
            // Obtener la ruta
            let path = String::from(request_parts[1]);
            
            // Analizar los encabezados
            let mut headers = Vec::new();
            let mut body_start = 0;
            
            for (i, line) in lines.iter().enumerate().skip(1) {
                if line.is_empty() {
                    // Una línea vacía separa los encabezados del cuerpo
                    body_start = i + 1;
                    break;
                }
                
                if let Some(colon_pos) = line.find(':') {
                    let name = line[..colon_pos].trim();
                    let value = line[colon_pos + 1..].trim();
                    headers.push((String::from(name), String::from(value)));
                }
            }
            
            // Extraer el cuerpo si existe
            let body = if body_start < lines.len() {
                let body_str = lines[body_start..].join("\r\n");
                Some(body_str.as_bytes().to_vec())
            } else {
                None
            };
            
            // Crear y devolver la solicitud API
            Some(ApiRequest {
                path,
                method,
                headers,
                body,
            })
        } else {
            None
        }
    }
    
    pub fn format_response(response: &ApiResponse) -> Vec<u8> {
        let mut http_response = String::new();
        
        // Línea de estado
        http_response.push_str(&format!("HTTP/1.1 {} {}\r\n", 
            response.status, 
            status_text(response.status)));
        
        // Encabezados
        for (name, value) in &response.headers {
            http_response.push_str(&format!("{}: {}\r\n", name, value));
        }
        
        // Línea vacía que separa encabezados del cuerpo
        http_response.push_str("\r\n");
        
        // Convertir a bytes
        let mut bytes = http_response.into_bytes();
        
        // Agregar cuerpo si existe
        if let Some(body) = &response.body {
            bytes.extend_from_slice(body);
        }
        
        bytes
    }
}

// Función auxiliar para obtener texto de estado HTTP
fn status_text(code: u16) -> &'static str {
    match code {
        100 => "Continue",
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
}