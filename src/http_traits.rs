use actix_http::body::Body;
use actix_web::HttpResponse;

pub trait CustomHttpResponse {
    /// A shortcut for returning a HttpResponse like
    ///
    /// ```
    /// return HttpResponse::Ok()
    /// .content_type("application/text")
    /// .header("Access-Control-Allow-Origin", "*")
    /// .body("OK: success!");
    /// ```
    ///
    /// # Arguments
    ///
    /// - `body': a `actix_http::body::Body` with the text content to return
    ///
    /// # Returns
    ///
    /// - `actix_web::HttpResponse`
    fn ok_text_response<B>(body: B) -> actix_web::HttpResponse
    where
        B: Into<Body>, ;
    /// A shortcut for returning a HttpResponse like
    ///
    /// ```
    /// return HttpResponse::Err()
    /// .content_type("application/text")
    /// .header("Access-Control-Allow-Origin", "*")
    /// .body("ERROR: can not get a lock on data!");
    /// ```
    ///
    /// # Arguments
    ///
    /// - `body': a `actix_http::body::Body` with the text content to return
    ///
    /// # Returns
    ///
    /// - `actix_web::HttpResponse`
    fn error_text_response<B>(body: B) -> actix_web::HttpResponse
    where
        B: Into<Body>, ;
    /// A shortcut for returning a HttpResponse like
    ///
    /// ```
    /// return HttpResponse::Ok()
    /// .content_type("application/json")
    /// .header("Access-Control-Allow-Origin", "*")
    /// .body("ERROR: can not get a lock on data!");
    /// ```
    ///
    /// # Arguments
    ///
    /// - `body': a `actix_http::body::Body` with the JSON content to return
    ///
    /// # Returns
    ///
    /// - `actix_web::HttpResponse`
    fn ok_json_response<B>(body: B) -> actix_web::HttpResponse
    where
        B: Into<Body>, ;
}

impl CustomHttpResponse for HttpResponse {
    fn ok_text_response<B>(body: B) -> actix_web::HttpResponse
    where
        B: Into<Body>,
    {
        return HttpResponse::Ok()
            .content_type("application/text")
            .header("Access-Control-Allow-Origin", "*")
            .body(body);
    }

    fn error_text_response<B>(body: B) -> actix_web::HttpResponse
    where
        B: Into<Body>,
    {
        return HttpResponse::BadRequest()
            .content_type("application/text")
            .header("Access-Control-Allow-Origin", "*")
            .body(body);
    }

    fn ok_json_response<B>(body: B) -> actix_web::HttpResponse
    where
        B: Into<Body>,
    {
        return HttpResponse::Ok()
            .content_type("application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(body);
    }
}
