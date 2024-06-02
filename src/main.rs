use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use serde::Serialize;
use std::convert::Infallible;
use tokio::runtime::Runtime;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = HelloResponse {
        message: "Hello, world!".to_string(),
    };
    let json = serde_json::to_string(&response).unwrap();

    Ok(Response::new(Body::from(json)))
}

#[tokio::main]
async fn main() {
    // Endereço e porta onde o servidor será executado
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Servidor rodando em http://{}", addr);

    // Executa o servidor
    if let Err(e) = server.await {
        eprintln!("erro do servidor: {}", e);
    }
}
