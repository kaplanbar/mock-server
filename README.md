# Mock-server
Mock-server is a lightweight Rust executable to quickly run mock API server where the response body of the API endpoints is defined in the yaml config. 

### Tech

To run Mock-server locally, you should have
* [Rust](https://www.rust-lang.org/tools/install)
* [Cargo](https://crates.io/)

installed on your environment.

### Installation

After you have installed Rust and Cargo, clone the repository.

```sh
$ git clone 
$ cargo run
```

### Mock API Config

The directory you run the Mock-server should contain a YAML configuration file with a Mockserver prefix in its name.

(Mockserver.example_config.yaml)
```yaml
---
  host: "0.0.0.0:8080"
  endpoints:
    endpoint1:
      path: "/"
      allowed_methods:
        - GET
        - POST
      response_body:
        quick: "hello"
        brown: 1
    endpoint2:
      path: "/hello"
      allowed_methods:
        - POST
      response_body:
        fox: "post"
```

### Todos

 - Add comments
 - Add more configuration options for endpoints
 - Add details to README

License
----

MIT
