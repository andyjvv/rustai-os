// Definición de protocolos básicos para comunicación en red
use alloc::string::String;

// Protocolo HTTP simplificado
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct HttpRequest {
    method: HttpMethod,
    path: String,
    headers: Vec<(String, String)>,
    body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, path: &str) -> Self {
        HttpRequest {
            method,
            path: String::from(path),
            headers: Vec::new(),
            body: None,
        }
    }
    
    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.push((String::from(name), String::from(value)));
    }
    
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = Some(body);
    }
    
    pub fn to_string(&self) -> String {
        let method_str = match self.method {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        };
        
        let mut request = format!("{} {} HTTP/1.1\r\n", method_str, self.path);
        
        for (name, value) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", name, value));
        }
        
        request.push_str("\r\n");
        
        if let Some(body) = &self.body {
            // Convertir cuerpo a cadena (si es texto)
            // En un caso real, habría que manejar datos binarios adecuadamente
            if let Ok(body_str) = core::str::from_utf8(body) {
                request.push_str(body_str);
            }
        }
        
        request
    }
}

// Definición simple de MQTT para comunicación IoT
pub enum MqttMessageType {
    CONNECT,
    CONNACK,
    PUBLISH,
    PUBACK,
    SUBSCRIBE,
    SUBACK,
    UNSUBSCRIBE,
    UNSUBACK,
    PINGREQ,
    PINGRESP,
    DISCONNECT,
}

pub struct MqttMessage {
    message_type: MqttMessageType,
    topic: Option<String>,
    payload: Option<Vec<u8>>,
    qos: u8,
    retain: bool,
}

impl MqttMessage {
    pub fn new_publish(topic: &str, payload: Vec<u8>, qos: u8, retain: bool) -> Self {
        MqttMessage {
            message_type: MqttMessageType::PUBLISH,
            topic: Some(String::from(topic)),
            payload: Some(payload),
            qos,
            retain,
        }
    }
    
    pub fn new_subscribe(topic: &str, qos: u8) -> Self {
        MqttMessage {
            message_type: MqttMessageType::SUBSCRIBE,
            topic: Some(String::from(topic)),
            payload: None,
            qos,
            retain: false,
        }
    }
}