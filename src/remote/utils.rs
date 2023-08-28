use std::collections::HashMap;
use std::str::FromStr;

use reqwest::{Client, Method, Response, StatusCode};
use serde_json::Value;
use tracing::{error, info};

use crate::error::{Error, Result};
use crate::error::Error::{CreationRequestErr, TargetNotFoundErr, UnexpectedConversionErr};
use crate::models::ServiceField;

pub async fn load_remote_data(
    url: &str, method: Method, token: Option<String>, body: Option<Value>, headers: Option<HashMap<String, String>>,
) -> Result<Response> {
    let client = Client::new();
    info!("Visiting {url}");
    let mut builder = client.request(method, &*url);
    if let Some(t) = token {
        builder = builder.header("Authorization", t.as_str());
    };
    if let Some(body) = body {
        builder = builder.json(&body);
    }
    if let Some(headers) = headers {
        for h in headers {
            builder = builder
                .header(h.0, h.1);
        }
    }
    let response = builder
        // .header("Content-Type", "application/json")
        .header("User-Agent", "ahead-org")
        .send().await?;
    Ok(response)
}

#[allow(clippy::if_same_then_else)]
pub fn get_type_by_json_value(value: Value) -> String {
    if value.is_i64() {
        "i64".to_string()
    } else if value.is_u64() {
        "u64".to_string()
    } else if value.is_f64() {
        "f64".to_string()
    } else if value.is_boolean() {
        "bool".to_string()
    } else if value.is_array() || value.is_object() {
        "json".to_string()
    } else {
        "String".to_string()
    }
}

pub async fn request_data(
    url: &str,
    method: &str,
    auth_token: Option<String>,
    target: Option<String>,
    body: Option<Value>,
    headers: Option<HashMap<String, String>>,
) -> Result<Vec<ServiceField>> {
    let http_method = Method::from_str(method).unwrap_or(Method::GET);
    let resp = load_remote_data(url, http_method, auth_token, body, headers).await;
    let result = match resp {
        Ok(r) => match r.status() {
            StatusCode::OK => {
                if let Ok(content) = r.text().await {
                    Ok::<String, Error>(content)
                } else {
                    Err(UnexpectedConversionErr)
                }
            }
            _ => {
                let code = r.status();
                let resp = r
                    .text()
                    .await?;
                error!("Request failed status {} {}", code, resp);
                Err(CreationRequestErr(resp))
            }
        },
        Err(e) => Err(e),
    }?;
    let v: Value = serde_json::from_str(result.as_str())?;
    let data = match target {
        None => v,
        Some(t) => {
            let target_array: Vec<&str> = t.split('.').collect();
            let mut res: Value = v;
            for t in target_array {
                if res.is_array() {
                    let mut is_found = false;
                    for val in res.as_array().unwrap() {
                        if let Some(new_res) = val.get(t) {
                            is_found = true;
                            res = new_res.clone();
                            break;
                        }
                    }
                    if !is_found {
                        error!("Can't find target field: {}", t);
                        return Err(TargetNotFoundErr(t.to_string()));
                    }
                } else {
                    res = res[t].to_owned();
                }
                info!("{}", res);
            }
            res
        }
    };
    let res_to_scan = if data.is_array() {
        let res = data;
        res[0].clone()
    } else {
        data
    };
    let mut fields: Vec<ServiceField> = vec![];
    for (key, value) in res_to_scan.as_object().unwrap() {
        fields.push(ServiceField::new(key.to_owned(), get_type_by_json_value(value.clone())));
    }

    Ok(fields)
}
