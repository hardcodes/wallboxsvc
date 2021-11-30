//#[macro_use]
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Holds the deserialized entries of the json file
/// that is passed to the program
#[derive(Clone, Deserialize, Debug)]
pub struct ConfigurationFile {
    pub web_bind_address: String,
    pub ssl_private_key_file: String,
    pub ssl_certificate_chain_file: String,
    pub get_relay_state_command: String,
    pub set_relay_state_command: String,
    pub user_id: String,
    pub password: String,
}

/// Loads a json file and deserializes it into an
/// instance of ConfigurationFile
impl ConfigurationFile {
    /// Read the web service configuration from a json file.
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        // Open the file in read-only mode with buffer.
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        // Read the JSON contents of the file as an instance of `ConfigurationFile`.
        let parsed_config: ConfigurationFile = serde_json::from_reader(reader)?;
        Ok(parsed_config)
    }
}

/// Holds the complete configuration information
/// that is passed to the HttpServer
#[derive(Clone)]
pub struct ApplicationConfiguration {
    pub configuration_file: ConfigurationFile,
}

/// Build a new instance of ApplicationConfiguration
impl ApplicationConfiguration {
    pub fn new<P: AsRef<Path>>(configuration_file_path: P) -> ApplicationConfiguration {
        ApplicationConfiguration {
            configuration_file: ConfigurationFile::read_from_file(configuration_file_path)
                .expect("can not load the json configuration file!"),
        }
    }
    pub fn get_ssl_acceptor_builder(&self) -> SslAcceptorBuilder {
        let mut ssl_acceptor_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        ssl_acceptor_builder
            .set_private_key_file(
                &self.configuration_file.ssl_private_key_file,
                SslFiletype::PEM,
            )
            .unwrap();
        ssl_acceptor_builder
            .set_certificate_chain_file(&self.configuration_file.ssl_certificate_chain_file)
            .unwrap();
        ssl_acceptor_builder
    }
}
