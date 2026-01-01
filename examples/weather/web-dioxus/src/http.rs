use shared::http::{
    protocol::{HttpRequest, HttpResponse},
    HttpError, Result,
};

pub async fn request(
    HttpRequest {
        method,
        url,
        headers,
        ..
    }: &HttpRequest,
) -> Result<HttpResponse> {
    let client = reqwest::Client::new();

    let mut request = match method.as_str() {
        "GET" => client.get(url),
        "POST" => client.post(url),
        _ => panic!("not yet handling this method"),
    };

    for header in headers {
        request = request.header(&header.name, &header.value);
    }

    let response = request
        .send()
        .await
        .map_err(|error| HttpError::Io(error.to_string()))?;
    let status = response.status().as_u16();
    let body = response
        .bytes()
        .await
        .map_err(|error| HttpError::Io(error.to_string()))?;

    Ok(HttpResponse::status(status).body(body.to_vec()).build())
}
