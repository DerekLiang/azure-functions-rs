use azure_functions::bindings::{HttpRequest, HttpResponse};
use azure_functions::func;

#[func(disabled = true)]
#[binding(name = "req", auth_level = "anonymous")]
pub fn greet(req: &HttpRequest) -> HttpResponse {
    debug!("Request: {:?}", req);

    format!(
        "Hello from Rust, {}!\n",
        req.query_params().get("name").map_or("stranger", |x| x)
    ).into()
}
