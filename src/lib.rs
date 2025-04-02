use anyhow::{anyhow, Error};
use encode_derive::Encode;
use kafka::apiversions::ApiVersionsRequest;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Debug, Encode)]
pub struct ErrorResponse {
    pub code: i16,
}

pub async fn respond(socket: &mut TcpStream, buf: &[u8]) {
    if let Err(e) = socket.write_all(buf).await {
        eprintln!("failed to write to socket; err = {e:?}");
        return;
    }
    let _ = socket.flush().await;
}

pub async fn handle_client(buf: &[u8], socket: &mut TcpStream) -> Result<(), Error> {
    let key: i16 = i16::decode(&buf[4..6], &mut 0);

    let handler = get_handler(key, buf);

    match handler {
        Some(h) => {
            handle_request(h, socket).await;
            Ok(())
        }
        None => Err(anyhow!("Error while getting handler")),
    }
}

pub mod kafka;
pub mod types;

pub trait Encode {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decode: Sized {
    fn decode(bytes: &[u8], offset: &mut usize) -> Self;
}

pub trait Offset {
    fn size(&self) -> usize;
}

pub trait Size {
    fn size_in_bytes(&self) -> usize;
}

pub enum Handler {
    ApiVersions(ApiVersionsRequest),
}

pub fn get_handler(key: i16, request: &[u8]) -> Option<Handler> {
    let mut offset = 0;

    match key {
        18 => Some(Handler::ApiVersions(ApiVersionsRequest::decode(
            request,
            &mut offset,
        ))),
        _ => None,
    }
}

pub async fn handle_request(handler: Handler, socket: &mut TcpStream) {
    match handler {
        Handler::ApiVersions(request) => {
            if let Ok(value) = request.handle_request().await {
                respond(socket, &value.encode()[..]).await
            } else {
                let err = ErrorResponse { code: -1 };
                respond(socket, &err.encode()[..]).await
            };
        }
    }
}
