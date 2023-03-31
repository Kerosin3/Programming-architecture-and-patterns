#![allow(unused_imports)]
use prost_types::Timestamp;
//use std::time::SystemTime;
use blake2::{Blake2b512, Blake2s256, Digest};
use lib_game_mechanics::run_me;
use tonic::{transport::Server, Request, Response, Status};
use tracing::Level;
use tracing_subscriber::fmt;
use transport::transport_interface_server::{TransportInterface, TransportInterfaceServer};
use transport::{ClientCommand, ClientRequest, Connection, ServerResponse};
pub mod transport {
    // import proto
    tonic::include_proto!("transport_interface");
}
//main function
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_me();
    let address = "[::1]:8080".parse().unwrap();
    let server_main_service = RpcServiceServer::default();

    let subscriber = fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("start main server loop");
    Server::builder()
        .add_service(TransportInterfaceServer::new(server_main_service))
        .serve(address)
        .await?;
    Ok(())
}

#[derive(Debug, Default)]
pub struct RpcServiceServer {}

#[tonic::async_trait]
impl TransportInterface for RpcServiceServer {
    async fn establish_connection(
        &self,
        request: Request<ClientRequest>,
    ) -> Result<Response<ServerResponse>, Status> {
        let recv_from_client = request.into_inner();
        tracing::info!("got name >>{:?}<<", recv_from_client);
        Ok(Response::new(transport::ServerResponse {
            server_answer: { format!("you asked for a person with a name {} ", "???") },
        }))
    }
}
