use tonic::{transport::Server, Request, Response, Status};

use bdt::bdt_server::{Bdt, BdtServer};
use bdt::{BdtRequest, BdtResponse, Row};

pub mod bdt {
    tonic::include_proto!("bdt");
}

#[derive(Debug, Default)]
pub struct BdtService {}

#[tonic::async_trait]
impl Bdt for BdtService {
    async fn get_data(
        &self,
        request: Request<BdtRequest>,
    ) -> Result<Response<BdtResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let row1 = Row {
            values: vec![]
        };

        let reply = BdtResponse {
            rows: vec![row1]
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let bdt_service = BdtService::default();

    Server::builder()
        .add_service(BdtServer::new(bdt_service))
        .serve(addr)
        .await?;

    Ok(())
}
