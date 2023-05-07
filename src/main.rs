use sqlx::sqlite::SqliteRow;
use sqlx::{Pool, Sqlite, SqlitePool, Row, Column, query};
use std::collections::HashMap;
use std::env;
use tonic::{transport::Server, Request, Response, Status};

use bdt::bdt_server::{Bdt, BdtServer};
use bdt::{BdtRequest, BdtResponse, BdtRow};

pub mod bdt {
    tonic::include_proto!("bdt");
}

#[derive(Debug)]
pub struct BdtService {
    pool: Pool<Sqlite>,
}

impl BdtService {
    pub fn new(pool: Pool<Sqlite>) -> BdtService {
        BdtService { pool }
    }
}

#[tonic::async_trait]
impl Bdt for BdtService {
    async fn get_data(
        &self,
        request: Request<BdtRequest>,
    ) -> Result<Response<BdtResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let query_str = {
            let select_str = req.columns.iter().map(|x| x.name.to_string()).collect::<Vec<_>>().join(",");
            let mut select = sql_query_builder::Select::new()
                .select(&select_str)
                .from(req.table.as_str());

            for (i, filter) in req.filters.iter().enumerate() {
                let constraint = format!("{} {} ${}", filter.column.to_string().clone(), filter.operator.to_string().clone(), i + 1);
                select = select.where_clause(&constraint).clone();
            };
            select.as_string()
        };

        let recs: Vec<SqliteRow> = query(&query_str).fetch_all(&self.pool).await.unwrap();

        let mut rows:Vec<BdtRow> = vec![];

        for rec in recs {
            let mut values: HashMap<String, String> = HashMap::new();
            for col in rec.columns() {
                let name = col.name();
                values.insert(name.to_string(), rec.get(name));        
            }
            let row = BdtRow { values };
            rows.push(row);
        }

        let reply = BdtResponse { rows };

        Ok(Response::new(reply))
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let _ = &env::set_var("DATABASE_URL", "sqlite:./data/db.db");
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    let bdt_service = BdtService::new(pool);

    Server::builder()
        .add_service(BdtServer::new(bdt_service))
        .serve(addr)
        .await?;

    Ok(())
}
