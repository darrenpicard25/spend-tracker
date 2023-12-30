use std::net::{Ipv4Addr, SocketAddr};

use adapters::Adapter;

use crate::adapters::api::Controller;

mod adapters;
mod app_state;
mod application;
mod domain;
mod error;
mod infrastructure;

#[tokio::main]
async fn main() -> Result<(), error::ServiceError> {
    simple_logger::init_with_level(log::Level::Info).map_err(|e| {
        log::error!("Error occurred instantiating logger: {e}");

        error::ServiceError::StartUp(e.to_string())
    })?;

    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, 8000));

    let mut controller = Controller::new(addr);

    controller.start().await?;

    Ok(())
}
