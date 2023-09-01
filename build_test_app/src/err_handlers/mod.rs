use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, HttpResponse, Result,
};
use std::str;
use tinytemplate::TinyTemplate;

// Custom error handlers, to return HTML responses when an error occurs.
pub fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |err: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(err.to_string())
    };

    let tt = request.app_data::<web::Data<TinyTemplate<'_>>>().map(|t| t.get_ref());
    match tt {
        Some(tt) => {
            let mut context = std::collections::HashMap::new();
            context.insert("error", String::from(error));
            context.insert("status_code", String::from(res.status().as_str()));
            let body = tt.render("error.html", &context);

            match body {
                Ok(body) => HttpResponse::build(res.status()).content_type(ContentType::html()).body(body),
                Err(_) => fallback(error),
            }
        },
        None => fallback(error),
    }
}
