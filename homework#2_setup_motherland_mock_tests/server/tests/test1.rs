#[cfg(test)]
#[allow(unused_imports)]
mod test_connection_handling {
    //     use crate::*;
    //     use super::connection_processor::server_connection_processing::Implement::*;
    //     use anyhow::*;
    //     use lib_common_server::connection_processor::server_connection_processing::Implement::TransportInterface;
    use lib_common_server::connection_processor::server_connection_processing::Implement::*;
    use mockall::predicate::*;
    use mockall::*;
    use rstest::*;
    use tonic::{Request, Response, Status};
    use transport::transport_interface_server::TransportInterface;
    use transport::{ClientCommand, ClientRequest, Connection, ServerResponse};
    //---------------------------------
    // trait reimported here
    //testing connection
    async fn test_accept_client(
        connection: Box<dyn TransportInterface>,
        request: Request<ClientRequest>,
    ) -> Result<Response<ServerResponse>, Status> {
        connection.establish_connection(request).await
    }

    #[tokio::test]
    async fn client_connection_establishing() {
        let request = tonic::Request::new(ClientRequest {
            type_c: Connection::Client.into(),
            command: ClientCommand::InitName.into(),
            timestamp: Some(std::time::SystemTime::now().into()),
            payload: None,
        });
        /*let response = Box::pin(std::future::ready(Ok(tonic::Response::new(
            transport::ServerResponse {
                server_answer: { format!("you asked for a person with a name {} ", "???") },
            },
        ))));*/
        let mut mock_server = Box::<MockRpcServiceServer>::default();
        mock_server
            .expect_establish_connection()
            .times(1)
            .returning(|_x| {
                Box::pin(std::future::ready(Ok(tonic::Response::new(
                    transport::ServerResponse {
                        server_answer: { format!("you asked for a person with a name {} ", "???") },
                    },
                ))))
            });
        assert!(test_accept_client(mock_server, request).await.is_ok());
    }
    #[tokio::test]
    async fn test_no_client() {
        let request = tonic::Request::new(ClientRequest {
            type_c: Connection::Agent.into(),
            command: ClientCommand::InitName.into(),
            timestamp: Some(std::time::SystemTime::now().into()),
            payload: None,
        });
        let mut mock_server = Box::<MockRpcServiceServer>::default();
        mock_server
            .expect_establish_connection()
            .times(1)
            .returning(|_x| {
                Box::pin(std::future::ready(Ok(tonic::Response::new(
                    transport::ServerResponse {
                        server_answer: { "connecting agent".to_string() },
                    },
                ))))
            });
        assert_eq!(
            test_accept_client(mock_server, request)
                .await
                .unwrap()
                .into_inner()
                .server_answer,
            "connecting agent"
        );
    }
}
