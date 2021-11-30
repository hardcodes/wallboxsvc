use crate::configuration::ApplicationConfiguration;
use crate::http_traits::CustomHttpResponse;
use actix_web::{web, HttpResponse};
use log::{debug, info, warn};
use regex::Regex;
use std::process::Command;

const VALID_ON_OFF_PARAM_REGEX: &str = r##"^(on|off)$"##;

/// Gets the current relay state and returns it as JSON value.
///
/// # Arguments
///
/// - `application_configuration`: application configuration, contains the command to execute
///
/// # Returns
///
/// - `HttpResponse`
/// 
/// Example
/// 
/// ```
/// {"state":on}
/// ```
pub async fn get_relay_state(
    application_configuration: web::Data<ApplicationConfiguration>,
) -> HttpResponse {
    debug!("fn get_relay_state");
    info!(
        "executing command: {}",
        &application_configuration
            .configuration_file
            .get_relay_state_command
    );
    if let Ok(output) = Command::new(
        &application_configuration
            .configuration_file
            .get_relay_state_command,
    )
    .output()
    {
        let stdout = String::from_utf8(output.stdout).expect("Found invalid UTF-8");
        let stderr = String::from_utf8(output.stderr).expect("Found invalid UTF-8");
        let result = format!("{}{}", stdout, stderr);
        debug!("result = {}", &result);
        return HttpResponse::ok_json_response(result);
    }

    HttpResponse::error_text_response("ERROR: can not execute command!")
}

/// Sets the current relay state and returns it as JSON value.
///
/// # Arguments
///
/// - `path`: tail of the url = on or off
/// - `application_configuration`: application configuration, contains the command to execute
///
/// # Returns
///
/// - `HttpResponse`
/// 
/// Example
/// 
/// ```
/// {"state":off}
/// ```
pub async fn set_relay_state(
    web::Path(new_state): web::Path<String>,
    application_configuration: web::Data<ApplicationConfiguration>,
) -> HttpResponse {
    debug!("fn set_relay_state: {}", &new_state);
    if let Ok(valid_param_regex) = Regex::new(VALID_ON_OFF_PARAM_REGEX) {
        if valid_param_regex.is_match(&new_state) {
            let on_off_param = new_state.as_str().clone();
            info!(
                "executing command: {} {}",
                &application_configuration
                    .configuration_file
                    .set_relay_state_command,
                    &on_off_param
            );
            if let Ok(output) = Command::new(
                &application_configuration
                    .configuration_file
                    .set_relay_state_command,
            )
            .arg(on_off_param)
            .output()
            {
                let stdout = String::from_utf8(output.stdout).expect("Found invalid UTF-8");
                let stderr = String::from_utf8(output.stderr).expect("Found invalid UTF-8");
                let result = format!("{}{}", stdout, stderr);
                debug!("result = {}", &result);
                return HttpResponse::ok_json_response(result);
            }
            return HttpResponse::error_text_response("ERROR: can not execute command!");
        }
        warn!("invalid parameter!");
        return HttpResponse::error_text_response("ERROR: invalid parameter!");
    }
    HttpResponse::error_text_response("ERROR: cannot build regex!")
}
