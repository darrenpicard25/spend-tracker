pub mod api;

use crate::error;

pub trait Adapter {
    async fn start(&mut self) -> Result<(), error::ServiceError>;
}
