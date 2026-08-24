use actix_web::{App, HttpServer, web,http::header};
use tracing::{info, error};
use crate::appstate::AppState;
use std::env;
use actix_cors::Cors;
use tracing_subscriber::{EnvFilter, Registry, fmt, prelude::*};
use actix_web::{dev::RequestHead, http::header::HeaderValue};


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
    let frontend_origin: String = frontend_origin.clone();
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(origin_filter(frontend_origin.clone()))
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::HeaderName::from_static("secret"),
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
                    .service(endpoints::user::reset_login)
                    .service(endpoints::user::check_reset_id)

                    .service(endpoints::post::get_post)
                    .service(endpoints::post::create_post)
                    .service(endpoints::post::update_post)
                    .service(endpoints::post::query_posts)
                    .service(endpoints::post::delete_post)

                    .service(endpoints::magma::list_magmas)
                    .service(endpoints::magma::new_magma)
            )
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}

#[inline]
fn config_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = tracing_appender::rolling::weekly("/logs", "app.log");

    // let filter = EnvFilter::new(
    //     "actix_http=info,actix_http=warn,actix_server=warn,mio=warn"
    // );

    let filter = EnvFilter::from_default_env();

    let (non_blocking, guard) =
        tracing_appender::non_blocking(file_appender);

    let file_layer: fmt::Layer<tracing_subscriber::layer::Layered<EnvFilter, Registry>, fmt::format::DefaultFields, fmt::format::Format, tracing_appender::non_blocking::NonBlocking> = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking);

    let stdout_layer = fmt::layer().with_ansi(false);

    Registry::default()
        .with(filter)
        .with(file_layer)
        .with(stdout_layer)
        .init();

    guard
}





fn origin_filter(
    frontend_origin: String,
) -> impl Fn(&HeaderValue, &RequestHead) -> bool + Clone {
    move |origin: &HeaderValue, req_head: &RequestHead| {
        let is_magma_post = req_head.uri.path() == "/api/v1/magma"
            && (
                req_head.method == actix_web::http::Method::POST
                || req_head
                    .headers
                    .get("access-control-request-method")
                    .map(|v| v.as_bytes() == b"POST")
                    .unwrap_or(false)
            );

        if is_magma_post {
            return true;
        }

        origin.as_bytes() == frontend_origin.as_bytes()
    }
}