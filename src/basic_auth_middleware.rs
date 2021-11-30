// use crate::configuration::ApplicationConfiguration;
// use actix_web::{dev::ServiceRequest, web, Error, HttpResponse};
// use actix_web_httpauth::extractors::basic::BasicAuth;
// use log::debug;

// pub async fn basic_auth_validator(
//     req: ServiceRequest,
//     credentials: BasicAuth,
// ) -> Result<ServiceRequest, Error> {
//     let web_data = req
//         .app_data::<web::Data<ApplicationConfiguration>>()
//         .unwrap();
//     if (credentials
//         .user_id()
//         .eq(&web_data.configuration_file.user_id))
//         && (credentials
//             .password()
//             .unwrap()
//             .trim()
//             .eq(&web_data.configuration_file.password))
//     {
//         debug!("user '{}' is authenticated", &credentials.user_id());
//         return Ok(req);
//     }

//     Err(actix_web::Error::from(
//         HttpResponse::BadRequest().body("nope!"),
//     ))
// }
