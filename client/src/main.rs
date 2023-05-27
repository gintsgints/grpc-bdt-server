pub mod bdt {
    tonic::include_proto!("bdt");
}

use crate::bdt::{BdtRequest, Column, Filter, bdt_client::BdtClient};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = BdtClient::connect("http://localhost:50051").await?;

    let col1 = Column {
        name: "config_type".to_string(),
    };
    let col2 = Column {
        name: "config_value".to_string(),
    };

    let filter = Filter {
        column: "CONFIG_VALUE".to_string(),
        operator: "=".to_string(),
        value: "AVA".to_string(),
    };

    let msg = BdtRequest {
        table: "TT_CONFIG".to_string(),
        columns: vec![col1, col2],
        filters: vec![filter]
    };

    let request = tonic::Request::new(msg);

    let _response = client.get_data(request).await?;

    println!("Result: ");

    Ok(())
}
