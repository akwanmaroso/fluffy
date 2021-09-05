mod config;
mod handlers;

use crate::config::Config;
use actix_web::{App, HttpServer, middleware::Logger};
use color_eyre::Result;
use handlers::app_config;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server Configuration");

    info!("Starting server at http://{}:{}/", config.host, config.port);

    HttpServer::new(move || { 
        App::new()
            .wrap(Logger::default())
            .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
