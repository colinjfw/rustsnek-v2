mod alloc;
mod api;
mod snake;
mod stats;

use hyper::service::{make_service_fn, service_fn};
use hyper::{body::to_bytes, Body, Error, Method, Request, Response, Server, StatusCode};
use serde::Serialize;
use serde_json;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Error> {
    let res = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new(to_json(&api::Hello {
            api_version: "1",
            author: "colinjfw",
            color: "red",
            head: "default",
            tail: "default",
            version: "0.0.1",
        }))),

        (&Method::POST, "/start") => Ok(Response::new(Body::default())),

        (&Method::POST, "/end") => Ok(Response::new(Body::default())),

        (&Method::POST, "/move") => {
            let body = to_bytes(req.into_body()).await?;
            match serde_json::from_slice::<api::MoveRequest>(&body.to_vec()) {
                Ok(move_req) => {
                    let move_res = api::MoveResponse {
                        direction: snake::run(&move_req),
                    };
                    Ok(Response::new(to_json(&move_res)))
                }
                Err(_) => Ok(Response::builder()
                    .status(StatusCode::UNPROCESSABLE_ENTITY)
                    .body(Body::default())
                    .unwrap()),
            }
        }

        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::default())
            .unwrap()),
    };
    println!("stats: {}", stats::STATS);
    res
}

fn to_json<T>(value: &T) -> Body
where
    T: ?Sized + Serialize,
{
    Body::from(serde_json::to_vec(value).unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();
    let make_service = make_service_fn(move |_| async move { Ok::<_, Error>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_service);
    println!("Listening on http://{}", addr);
    server.await?;
    Ok(())
}
