use actix_web::{App, HttpServer, web};
use tracing::{info, error};
use crate::appstate::AppState;
use std::env;

mod endpoints;
mod appstate;
mod error;
mod api_error;
mod schemas;
mod services;
mod middleware;

#[tokio::main]
async fn main() {
    let port = env::var("BACKEND_PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("127.0.0.1:{}", port);

    let app_state = AppState::new(
        &env::var("DB_URL").unwrap(),
                    &env::var("BACKEND_PORT").unwrap()
        ).await.unwrap();

    loop{
        match run(&address,&app_state).await{
            Ok(_) => (),
            Err(e) => error!("Error on Server: {}", e)
        };
    }
}


async fn run(address: &String, app_state : &AppState) -> Result<(), error::Error> {
    info!("Starting server on {}", address);
    let app_state = web::Data::new(app_state.clone());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(endpoints::user::create_user)
            .service(endpoints::user::login_user)
            .service(endpoints::user::refresh_token)
            .service(endpoints::post::create_post)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}

