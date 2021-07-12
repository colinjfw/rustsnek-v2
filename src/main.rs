mod alloc;
mod api;
mod snake;
mod stats;

use httparse::{Request, EMPTY_HEADER};
use log::*;
use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::time::Instant;

const HTTP_OK: &[u8] = "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\r\n".as_bytes();

struct Handler {
    response_buf: Vec<u8>,
    request_buf: Vec<u8>,
}

impl Handler {
    fn handle(&mut self, stream: &mut TcpStream) -> Result<()> {
        let mut request_headers = [EMPTY_HEADER; 16];
        let mut request = Request::new(&mut request_headers);

        self.request_buf.resize(12288, 0);
        stream.read(&mut self.request_buf)?;
        let status = match request.parse(&self.request_buf) {
            Ok(s) => s,
            Err(e) => return Result::Err(Error::new(ErrorKind::Other, e)),
        };
        if status.is_partial() {
            return Result::Err(Error::from(ErrorKind::InvalidInput));
        }
        let content_start = status.unwrap();
        let content_len = match request
            .headers
            .iter()
            .find(|header| header.name == "Content-Length")
            .and_then(|header| str::from_utf8(header.value).ok())
            .and_then(|value| value.parse::<usize>().ok())
        {
            Some(len) => len,
            None => 0,
        };
        let content = &self.request_buf[content_start..(content_start+content_len)];

        self.response_buf.truncate(0);
        self.response_buf.write(HTTP_OK)?;

        if let Some(path) = request.path {
            match path {
                "/move" => {
                    let req: api::MoveRequest = serde_json::from_slice(content)?;
                    debug!("/move request: {}", str::from_utf8(content).unwrap());
                    let direction = snake::run(&req);
                    serde_json::to_writer(
                        &mut self.response_buf,
                        &api::MoveResponse {
                            direction,
                        },
                    )?;
                }
                _ => {
                    serde_json::to_writer(
                        &mut self.response_buf,
                        &api::RootResponse {
                            api_version: "1",
                            author: "colinjfw",
                            color: "red",
                            head: "default",
                            tail: "default",
                            version: "0.0.1",
                        },
                    )?;
                }
            };
        };

        stream.write(&self.response_buf)?;
        stream.flush()?;
        Ok(())
    }
}

fn server(s: &'static str) {
    let mut handler = Handler {
        request_buf: Vec::with_capacity(12288),
        response_buf: Vec::with_capacity(12288),
    };
    let listener = TcpListener::bind(s).unwrap();
    info!("listener started on {}", s);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let now = Instant::now();
                match handler.handle(&mut stream) {
                    Ok(_) => info!(
                        "request ok: in {:?}, stats: {}",
                        now.elapsed(),
                        stats::STATS
                    ),
                    Err(err) => error!("request failed: {}", err),
                }
            }
            Err(e) => error!("http: connection failed {}", e),
        }
    }
}

fn main() {
    env_logger::init();
    server("0.0.0.0:3000");
}
