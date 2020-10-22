extern crate serde_json;
extern crate serde_yaml;

use std::{env, io};
use tiny_http::{Method, Response, Server, StatusCode};

#[derive(Default)]
struct Endpoint {
    path: String,
    response_body: String,
    allowed_methods: Vec<Method>,
}

impl Endpoint {
    pub fn new(path: String, response_body: String) -> Self {
        Self {
            path,
            response_body,
            allowed_methods: Vec::new(),
        }
    }
    pub fn add_method(&mut self, method: &String) {
        match method.trim_matches(' ').to_uppercase().as_str() {
            "GET" => self.allowed_methods.push(Method::Get),
            "POST" => self.allowed_methods.push(Method::Post),
            "PUT" => self.allowed_methods.push(Method::Put),
            "PATCH" => self.allowed_methods.push(Method::Patch),
            "DELETE" => self.allowed_methods.push(Method::Delete),
            _ => println!("Method {} is not known", method),
        }
    }
    pub fn check_route(&self, path: &String, method: Method) -> bool {
        return self.allowed_methods.iter().any(|m| *m == method) && self.path == *path;
    }
}

fn main() {
    let mut config_path = env::current_dir().unwrap();

    let entries = std::fs::read_dir(config_path.clone())
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    for entry in entries {
        if entry.is_file() {
            let filename = entry.file_name().unwrap();
            if filename.to_str().unwrap().get(..10).unwrap() == "Mockserver" {
                config_path.push(filename);
                break;
            }
        }
    }

    if config_path.extension() == None {
        panic!(
            "Your current directory should contain a yaml config file containing Mockserver prefix"
        );
    }

    let config_reader = std::fs::File::open(config_path).unwrap();
    let config_yaml: serde_yaml::Value = serde_yaml::from_reader(config_reader).unwrap();

    let host: &str = config_yaml.get("host").unwrap().as_str().unwrap();

    let endpoints_list_mapping = config_yaml.get("endpoints").unwrap().as_mapping().unwrap();

    let mut endpoints: Vec<Endpoint> = Vec::new();

    for endpoint_mapping_tuple in endpoints_list_mapping.iter() {
        let endpoint_mapping = endpoint_mapping_tuple.1.as_mapping().unwrap();
        let path: String = String::from(
            endpoint_mapping[&serde_yaml::Value::String("path".into())]
                .as_str()
                .unwrap(),
        );

        let allowed_methods: Vec<String> = endpoint_mapping
            [&serde_yaml::Value::String("allowed_methods".into())]
            .clone()
            .as_sequence_mut()
            .unwrap()
            .iter_mut()
            .map(|value| String::from(value.as_str().unwrap()))
            .collect();

        let response_body =
            endpoint_mapping[&serde_yaml::Value::String("response_body".into())].to_owned();

        let response_body_json = serde_json::to_value(response_body).unwrap().to_string();

        let mut endpoint = Endpoint::new(path, response_body_json);

        for method in allowed_methods.iter() {
            endpoint.add_method(method);
        }
        endpoints.push(endpoint);
    }

    let server = Server::http(host).unwrap();

    for request in server.incoming_requests() {
        let path: String = String::from(request.url());
        let method: &Method = request.method();

        let mut response =
            Response::from_string("No endpoint matched").with_status_code(StatusCode::from(404));

        for endpoint in &endpoints {
            if endpoint.check_route(&path, method.clone()) {
                response = Response::from_string(endpoint.response_body.clone())
                    .with_status_code(StatusCode::from(200));
                break;
            }
        }
        match request.respond(response) {
            Err(e) => println!("Error occured while sending response {}", e),
            _ => (),
        }
    }
}
