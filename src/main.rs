use std::io;

use actix_files::NamedFile;
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    get,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use env_logger;
use handlebars::Handlebars;
use log;
use serde_json::json;

/// favicon handler
#[get("/favicon.ico")]
async fn favicon() -> Result<impl Responder> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "title": "Home"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/about")]
async fn about(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "title": "About Me"
    });
    let body = hb.render("about", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/contact")]
async fn contact(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "title": "Contact"
    });
    let body = hb.render("contact", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/legals")]
async fn legals(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "title": "Legals"
    });
    let body = hb.render("legals", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/projects")]
async fn projects(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "title": "My Projects"
    });
    let body = hb.render("projects", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://0.0.0.0:80");

    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html.hbs", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .app_data(handlebars_ref.clone())
            .service(favicon)
            .service(index)
            .service(about)
            .service(contact)
            .service(legals)
            .service(projects)
    })
    .bind(("0.0.0.0", 80))?
    .bind("[::1]:80")?
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(
        &res,
        "Page not found",
        "We couldn't find the requested Path: ".to_owned()
            + &HttpRequest::path(res.request()).to_owned(),
    );
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(
    res: &ServiceResponse<B>,
    error: &str,
    description: String,
) -> HttpResponse<BoxBody> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({
                "title": error,
                "describtion": description,
                "error": error,
                "status_code": res.status().as_str(),
            });
            let body = hb.render("error", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type(ContentType::html())
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
