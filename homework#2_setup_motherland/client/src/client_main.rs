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
    loop {
        println!("\n-----Enter a name------");
        let mut some_input = String::new();
        stdin().read_line(&mut some_input).unwrap();
        let some_input = some_input.trim();
        let request = tonic::Request::new(ClientRequest {
            type_c: Connection::Client.into(),
            command: ClientCommand::InitName.into(),
            timestamp: Some(std::time::SystemTime::now().into()),
        });
        let response = client.establish_connection(request).await?;
        //         let resp: WrapPerson = WrapPerson::convert(response.into_inner());
        println!("---> Server answered {}", "OK");
    }
    Ok(())
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
