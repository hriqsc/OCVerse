use actix_web::{App, HttpServer, web,http::header};
use tracing::{info, error};
use crate::appstate::AppState;
use std::env;
use actix_cors::Cors;
use tracing_subscriber::{EnvFilter, Registry, fmt, prelude::*};

mod endpoints;
mod appstate;
mod error;
mod api_error;
mod schemas;
mod services;
mod middleware;
mod shared;
mod definitions;
mod test;
mod validator;

#[tokio::main]
async fn main() {

    let _guard = config_tracing();

    let port = env::var("BACKEND_PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{}", port);
    let app_state = AppState::new().await.unwrap();

    let frontend_origin = env::var("FRONTEND_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    loop{
        match run(&address,&app_state,&frontend_origin).await{
            Ok(_) => (),
            Err(e) => error!("Error on Server: {}", e)
        };
    }
}


async fn run(
    address: &String,
    app_state: &AppState,
    frontend_origin: &String
) -> Result<(), error::Error> {
    info!("Starting server on {}", address);
    let app_state = web::Data::new(app_state.clone());
    let frontend_origin = frontend_origin.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .service(
                web::scope("")
                    .wrap(cors)
                    .service(endpoints::user::create_user)
                    .service(endpoints::user::login_user)
                    .service(endpoints::user::logout_user)
                    .service(endpoints::user::refresh_token)

                    .service(endpoints::post::get_post)
                    .service(endpoints::post::create_post)
                    .service(endpoints::post::update_post)
                    .service(endpoints::post::query_posts)
                    .service(endpoints::post::delete_post)

                    .service(endpoints::magma::list_magmas)
            )
            .service(endpoints::magma::new_magma)
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}

#[inline]
fn config_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = tracing_appender::rolling::weekly("/logs", "app.log");

    let filter = EnvFilter::new(
        "actix_http=info,actix_http=warn,actix_server=warn,mio=warn"
    );

    let (non_blocking, guard) =
        tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking);

    Registry::default()
        .with(filter)
        .with(file_layer)
        .init();

    guard
}