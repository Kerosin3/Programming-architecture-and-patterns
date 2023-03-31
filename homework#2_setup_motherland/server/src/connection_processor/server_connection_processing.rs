pub mod Implement {

    use transport::transport_interface_server::{TransportInterface, TransportInterfaceServer};
    use transport::{ClientCommand, ClientRequest, Connection, ServerResponse};
    pub mod transport {
        // import proto
        tonic::include_proto!("transport_interface");
    }
    use tonic::{transport::Server, Request, Response, Status};
    //  use crate::RpcServiceServer;
    pub fn printsome() {
        println!("aaaaaaaaaa");
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
}
