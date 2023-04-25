mod remote;
mod error;
pub mod models;

/// Request data and convert it to fields structure
/// pub struct ServiceField {
///     pub name: String,
///     pub field_type: String,
///     pub is_optional: bool,
/// }
/// Paramters:
/// url: &str - full thirdparty url. Params are included
/// method: &str - http method. GET, POST ans etc.
/// auth_token: Option<String> - just an Authorization header. e.g. "Bearer {Your super secure token}"
/// body: Option<Value> - json body for post requests
/// headers: Option<HashMap<String, String>> - Addional headers
/// target: Option<String> - level of nesting to be skiped. e.g. products.product will try to find object {products: {product: {..object used by api to extract fields} }}
pub use remote::utils::request_data;
