use prost_types::Timestamp;
use std::io::stdin;
extern crate hex_slice;
use hex_slice::AsHex;
use transport::transport_interface_client::TransportInterfaceClient;
use transport::{ClientCommand, ClientRequest, Connection, ServerResponse};
pub mod transport {
    tonic::include_proto!("transport_interface");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TransportInterfaceClient::connect("http://[::1]:8080").await?;
    let response = client
        .establish_connection(BaseRequest::construct(Cmd1::Name))
        .await?;
    let player_name = response.into_inner().server_answer;
    println!("MY NAME IS {}", player_name);
    //------------------------------------------------------
    let response = client
        .assign_arena(BaseRequest::construct(Cmd1::Battle))
        .await?;
    let battle_name = response.into_inner().server_answer;
    println!("BATTLENAME IS {}", battle_name);
    //-------------------------------------
    'process_input: loop {
        let cmd_text = take_input();
        let response = client
            .client_command(BaseRequest::construct(Cmd1::Control(cmd_text)))
            .await?;
    }
    Ok(())
}

fn take_input() -> String {
    println!("\n-----type a command------");
    let mut some_input = String::new();
    stdin().read_line(&mut some_input).unwrap();
    let some_input = some_input.trim();
    let out = format!("#{}", some_input);
    println!("your command: {out}");
    out
}

struct BaseRequest {}
enum Cmd1 {
    Name,
    Battle,
    Control(String),
}
impl BaseRequest {
    fn construct(pattern: Cmd1) -> tonic::Request<ClientRequest> {
        match pattern {
            Cmd1::Name => tonic::Request::new(ClientRequest {
                type_c: Connection::Client.into(),
                command: ClientCommand::InitName.into(),
                timestamp: Some(std::time::SystemTime::now().into()),
                payload: None,
            }),
            Cmd1::Battle => tonic::Request::new(ClientRequest {
                type_c: Connection::Client.into(),
                command: ClientCommand::AssignBattle.into(),
                timestamp: Some(std::time::SystemTime::now().into()),
                payload: None,
            }),
            Cmd1::Control(s) => tonic::Request::new(ClientRequest {
                type_c: Connection::Client.into(),
                command: ClientCommand::Control.into(),
                timestamp: Some(std::time::SystemTime::now().into()),
                payload: Some(s),
            }),
        }
    }
}
/*
struct WrapPerson {
    timestamp: Timestamp,
    name: String,
    hash: Vec<u8>,
}
impl WrapPerson {
    fn convert(pr: PersonResponse) -> Self {
        Self {
            name: pr.confirmation,
            timestamp: pr.timestamp.unwrap(),
            hash: pr.hash,
        }
    }
}

impl std::fmt::Debug for WrapPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PersonResponse")
            .field("name", &self.name)
            .field("timestamp", &self.timestamp)
            .field("hash", &format!("{:x?}", &self.hash))
            .finish()
    }
}
impl std::fmt::Display for WrapPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nrequested name:{}\nhash:{:02x}\ntimestamp:{}",
            self.name,
            self.hash.as_hex(),
            self.timestamp
        )
    }
} */
