// tell the rust compiler which modules we have in extra files
mod basic_auth_middleware;
mod cli_parser;
mod configuration;
mod handler_functions;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
// use actix_web_httpauth::middleware::HttpAuthentication;
//use basic_auth_middleware::basic_auth_validator;
use cli_parser::parse_cli_parameters;
use configuration::ApplicationConfiguration;
use handler_functions::{get_relay_state, set_relay_state};
mod http_traits;
use std::io::Write;
use std::path::Path;

const PROGRAM_NAME: &str = "wallboxsvc";
const PROGRAM_VERSION: &str = "0.1.0";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // crate env_logger is configured via the RUST_LOG environment variable
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "debug, actix_web=trace");
    #[cfg(not(debug_assertions))]
    std::env::set_var("RUST_LOG", "info, actix_web=trace");
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    // parse cli parameters and load the configuration
    let clap_arg_matches = parse_cli_parameters(PROGRAM_NAME, PROGRAM_VERSION);
    let web_data =
        ApplicationConfiguration::new(Path::new(clap_arg_matches.value_of("configfile").unwrap()));
    // make a clone of the web_bind_address since it used after
    // moving application_configuration into the webservice
    let web_bind_address = web_data.configuration_file.web_bind_address.clone();
    // load ssl keys
    let ssl_acceptor_builder = web_data.get_ssl_acceptor_builder();

    HttpServer::new(move || {
        //let auth = HttpAuthentication::basic(basic_auth_validator);
        App::new()
            // Enable the logger.
            .wrap(middleware::Logger::default())
            //.wrap(auth)
            // clone of the application configuration
            .data(web_data.clone())
            // get the current state of the relay and return json string
            .route("/get/relay_state", web::get().to(get_relay_state))
            // set a new relay state and return the current state after changing it as json string
            .route(
                "/set/relay_state/{new_state}",
                web::get().to(set_relay_state),
            )
            // Serve a tree of static files at the web root and specify the index file.
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    // use only one worker, there shouldn't be too many clients at the same time
    .workers(1)
    .bind_openssl(web_bind_address, ssl_acceptor_builder)?
    .run()
    .await
}
