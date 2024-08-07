use std::net::SocketAddr;

use chrono::{DateTime, Utc};
use clap::Parser;
use clap_num::number_range;
use error_stack::ResultExt;
use hexafreeze::{Generator, HexaFreezeError};
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

use common::with_bootstrap;
use proto::de::phyrone::kiwi::snowflake::snowflake_service_server::SnowflakeServiceServer;
use proto::de::phyrone::kiwi::snowflake::{Snowflake, SnowflakesRequest, SnowflakesResponse};

with_bootstrap!(main_inner, StartupParams);

#[instrument]
async fn main_inner(startup_params: StartupParams) -> error_stack::Result<(), ApplicationError> {
    info!("NodeID is {}", startup_params.node_id);
    let epoch = DateTime::parse_from_rfc3339(startup_params.epoch.as_str());
    let epoch = match epoch {
        Ok(ok) => ok,
        Err(_) => DateTime::parse_from_rfc2822(startup_params.epoch.as_str())
            .change_context(ApplicationError)
            .attach_printable_lazy(|| {
                format!(
                    "specified epoch start '{}' is not a valid rfc2822 nor a valid rfc3339 format",
                    &startup_params.epoch
                )
            })?,
    };
    let epoch: DateTime<Utc> = epoch.into();

    let snowflake =
        Generator::new(startup_params.node_id as i64, epoch).change_context(ApplicationError)?;

    let snowflake = SnowflakeServiceImpl::new(snowflake);
    let snowflake = SnowflakeServiceServer::new(snowflake);
    info!("Starting server on {}...", startup_params.bind);

    Server::builder()
        .add_service(snowflake)
        .serve(startup_params.bind)
        .await
        .change_context(ApplicationError)?;

    Ok(())
}

fn node_id_parser(s: &str) -> Result<u16, String> {
    number_range(s, 0, 1023)
}

#[derive(Debug, Parser)]
struct StartupParams {
    #[clap(env, value_parser = node_id_parser)]
    node_id: u16,
    #[clap(short, long, env, default_value = "2024-01-01T00:00:00Z")]
    epoch: String,
    #[clap(short, long, env, default_value = "0.0.0.0:50152")]
    bind: SocketAddr,
}

#[derive(Clone)]
struct SnowflakeServiceImpl {
    generator: Generator,
}

impl SnowflakeServiceImpl {
    fn new(generator: Generator) -> Self {
        Self { generator }
    }

    async fn generate_snowflake_batch(&self, count: u32) -> Result<Vec<i64>, HexaFreezeError> {
        let mut snowflakes = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let snowflake = self.generator.generate().await?;
            snowflakes.push(snowflake);
        }
        Ok(snowflakes)
    }
}

#[tonic::async_trait]
impl proto::de::phyrone::kiwi::snowflake::snowflake_service_server::SnowflakeService
    for SnowflakeServiceImpl
{
    async fn get_snowflake(&self, request: Request<()>) -> Result<Response<Snowflake>, Status> {
        let snowflake = self.generator.generate().await.map_err(|e| {
            error!("error when generating snowflake: {:?}", e);
            match e {
                HexaFreezeError::EpochInTheFuture => {
                    Status::failed_precondition("Epoch in the future")
                }
                HexaFreezeError::EpochTooFarInThePast => {
                    Status::failed_precondition("Epoch too far in the past")
                }
                HexaFreezeError::NodeIdTooLarge => Status::failed_precondition("Node ID too large"),
                HexaFreezeError::ClockWentBackInTime => Status::internal("Clock went back in time"),
                HexaFreezeError::Surpassed64BitLimit => {
                    Status::resource_exhausted("no more ids left")
                }
            }
        })?;

        Ok(Response::new(Snowflake { snowflake }))
    }
    async fn get_snowflakes(
        &self,
        request: Request<SnowflakesRequest>,
    ) -> Result<Response<SnowflakesResponse>, Status> {
        self.generate_snowflake_batch(request.get_ref().count)
            .await
            .map(|snowflakes| SnowflakesResponse { snowflakes })
            .map(Response::new)
            .map_err(|e| {
                error!("error when generating snowflake: {:?}", e);
                match e {
                    HexaFreezeError::EpochInTheFuture => {
                        Status::failed_precondition("Epoch in the future")
                    }
                    HexaFreezeError::EpochTooFarInThePast => {
                        Status::failed_precondition("Epoch too far in the past")
                    }
                    HexaFreezeError::NodeIdTooLarge => {
                        Status::failed_precondition("Node ID too large")
                    }
                    HexaFreezeError::ClockWentBackInTime => {
                        Status::internal("Clock went back in time")
                    }
                    HexaFreezeError::Surpassed64BitLimit => {
                        Status::resource_exhausted("no more ids left")
                    }
                }
            })
    }
}

#[derive(Debug, Default)]
pub struct ApplicationError;

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Application error")
    }
}

impl std::error::Error for ApplicationError {}
