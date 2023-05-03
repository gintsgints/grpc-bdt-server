use tonic::{transport::Server, Request, Response, Status};

use tt_config::tt_config_server::{TtConfig, TtConfigServer};
use tt_config::{BdtRequest, TtConfigObject};

pub mod tt_config {
    tonic::include_proto!("tt.config");
}

#[derive(Debug, Default)]
pub struct TtConfigService {}

#[tonic::async_trait]
impl TtConfig for TtConfigService {
    async fn get_data(
        &self,
        request: Request<BdtRequest>,
    ) -> Result<Response<TtConfigObject>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let reply = TtConfigObject {
            config_type: "type".to_string(),
            config_value: format!("Select: {:?}", req),
            config_num_value: 0.3,
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let tt_config_service = TtConfigService::default();

    Server::builder()
        .add_service(TtConfigServer::new(tt_config_service))
        .serve(addr)
        .await?;

    Ok(())
}
