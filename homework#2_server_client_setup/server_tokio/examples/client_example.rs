#![allow(dead_code)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
mod args;
use args::ClientArgs;
use clap::Parser;
#[path = "../src/ipc_message.rs"]
mod ipc_message;
use bytes::Bytes;
use futures::sink::SinkExt;
use futures_util::*;
use ipc_message::*;
use std::time::Duration;
use tokio::time::timeout;
//use socket2::{Domain, Protocol, SockRef, Socket, Type};
use tokio::{io::BufStream, net::TcpStream};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

static SERVER_HELLO: &str = "hello from server, what do you want?";

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = ClientArgs::parse();
    println!("starting client application");
    let mut message_to_server = Message::new(NetMsgType::SendClient);
    match args.enable {
        Some(state) => {
            if state {
                message_to_server.assign_command(Command::TurnOn);
            } else {
                message_to_server.assign_command(Command::TurnOff);
            }
        }
        None => message_to_server.assign_command(Command::GetProperty), // if not en\dis then get info
    }
    message_to_server.assign_devname(args.devname);
    println!("devname: {}", message_to_server.devname);
    let msg = message_to_server.serialize_message();
    if let Ok(stream) = TcpStream::connect("127.0.0.1:12345").await {
        tokio::spawn(async move { client_process(stream, msg).await })
            .await
            .unwrap();
    } else {
        println!("cant connect to server, exiting!");
        std::process::exit(1);
    }
}

async fn client_process(stream: TcpStream, msg: Vec<u8>) {
    println!(
        "connected to ip: {}, port: {}",
        stream.local_addr().unwrap().ip(),
        stream.local_addr().unwrap().port()
    );
    let stream_buf = BufStream::new(stream);
    let codec = LengthDelimitedCodec::builder()
        .length_field_offset(0) // default value
        .length_field_type::<u16>()
        .length_adjustment(0) // default value
        .new_codec();
    let mut framed_stream = Framed::new(stream_buf, codec);
    let mut loop_n = 0_usize;
    '_accept_server_request: while let Some(frame) = framed_stream.next().await {
        match frame {
            Ok(f) => {
                if loop_n == 0 {
                    if String::from_utf8_lossy(&f) != SERVER_HELLO {
                        return; // not valid greet
                    }
                    let frame = Bytes::from("client magic words!");
                    framed_stream.send(frame).await.unwrap();
                    loop_n += 1;
                } else {
                    if String::from_utf8_lossy(&f) == "ASK" {
                        let frame: Bytes = Bytes::from(msg.to_owned()); //ok..
                                                                        //println!("send serialized");
                        framed_stream.send(frame).await.unwrap();
                    } else {
                        println!("INFO FROM SERVER: {}", String::from_utf8_lossy(&f));
                        return;
                    }
                    loop_n += 1;
                }
            }
            Err(_) => println!("some error!"),
        }
    }
}
