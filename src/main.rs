use anyhow::Error;
use bytes::BytesMut;
use codecrafters_kafka::kafka::{BaseRequest, BaseResponse};
use codecrafters_kafka::{Decode, Encode};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

static SERVER_ADDRESS: &str = "127.0.0.1:9092";

async fn respond(socket: &mut TcpStream, buf: &[u8]) {
    if let Err(e) = socket.write_all(buf).await {
        eprintln!("failed to write to socket; err = {e:?}");
        return;
    }
    let _ = socket.flush().await;
}

async fn handle_client(buf: &[u8], socket: &mut TcpStream) -> Result<(), Error> {
    println!("Handle client");

    let mut offset = 0 as usize;
    let request = BaseRequest::decode(buf, &mut offset);

    println! {"{request:?}"}

    let response = BaseResponse {
        size: request.size,
        correlation_id: request.correlation_id,
    };

    respond(socket, &response.encode()[..]).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(SERVER_ADDRESS).await?;
    println!("Starting server at {SERVER_ADDRESS}");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = BytesMut::with_capacity(1024);

            loop {
                buf.resize(buf.capacity(), 0);
                let _n = match socket.read(&mut buf).await {
                    Ok(0) => {
                        println!("Connection closed by client.");
                        return;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {e:?}");
                        return;
                    }
                };
                let result = handle_client(&buf[..], &mut socket).await;

                if let Err(result) = result {
                    eprintln!("{:?}", result)
                }
            }
        });
    }
}
