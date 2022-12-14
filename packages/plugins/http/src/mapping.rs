use crate::wrap::types::{Request, Response};
use polywrap_plugin::error::PluginError;
use std::collections::BTreeMap;

pub enum RequestMethod {
    GET,
    POST,
}

pub fn parse_response(response: ureq::Response) -> Result<Response, PluginError> {
    let headers = response
        .headers_names()
        .iter()
        .map(|header_name| {
            (
                header_name.to_string(),
                response.header(header_name).unwrap().to_string(),
            )
        })
        .collect::<BTreeMap<String, String>>();

    let status = response.status();
    let status_text = response.status_text().to_string();

    let data = response
        .into_string()
        .map_err(|e| PluginError::ModuleError(e.to_string()))?;

    Ok(Response {
        status: status.into(),
        status_text,
        headers: Some(headers),
        body: Some(data),
    })
}

pub fn parse_request(
    url: &str,
    request: Option<Request>,
    method: RequestMethod,
) -> Result<ureq::Request, PluginError> {
    let mut req = match method {
        RequestMethod::GET => ureq::get(url),
        RequestMethod::POST => ureq::post(url),
    };

    if let Some(request) = request {
        if let Some(url_params) = request.url_params {
            for (param, value) in url_params.iter() {
                req = req.query(param, value)
            }
        };

        if let Some(headers) = request.headers {
            for (name, value) in headers.iter() {
                req = req.set(name, value)
            }
        }
    }

    Ok(req)
}
