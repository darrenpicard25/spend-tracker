use std::net::SocketAddr;

use app::App;
use axum::Router;
use leptos::LeptosOptions;
use leptos_axum::{generate_route_list, LeptosRoutes};

use crate::{adapters::api::fileserv::file_and_error_handler, app_state::AppState};

use super::Adapter;

mod error;
mod fileserv;
mod user;

pub struct Controller {
    addr: SocketAddr,
}

impl Controller {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }
}

impl Adapter for Controller {
    async fn start(&mut self) -> Result<(), crate::error::ServiceError> {
        let leptos_options = LeptosOptions::builder()
            .site_addr(self.addr)
            .output_name("server")
            .build();

        let addr = leptos_options.site_addr;
        let routes = generate_route_list(App);

        let app_state = AppState::new(leptos_options).await?;

        // build our application with a route
        let app = Router::new()
            .merge(user::routes())
            .leptos_routes(&app_state, routes, App)
            .fallback(file_and_error_handler)
            .with_state(app_state);

        // run our app with hyper
        // `axum::Server` is a re-export of `hyper::Server`

        log::info!("listening on http://{}", &addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .map_err(|e| {
                log::error!("Error occurred starting up axum server on {addr}: {e}");

                crate::error::ServiceError::StartUp(e.to_string())
            })?;

        Ok(())
    }
}
