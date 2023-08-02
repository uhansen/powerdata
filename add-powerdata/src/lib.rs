mod model;
mod utils;

use anyhow::Result;
use bytes::Bytes;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

use model::POIData;

use utils::{bad_request,internal_server_error,method_not_allowed,not_found, ok, no_content};

enum Api {
    Create(model::POIData),
    ReadByHandle(String),
    Update(model::POIData),
    Delete(model::POIData),
    BadRequest,
    NotFound,
    MethodNotAllowed,
}
/// A simple Spin HTTP component.
#[http_component]
fn handle_add_powerdata(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Powerdata added".into()))?)
}

//https://github.com/fermyon/code-things/commit/f40d033a25729baed8b8351ed2e28f37688dcfe3#diff-764055ca37d0f6c66c755b47c85025b44334699d510c23653f33d802e826a23f
// https://www.fermyon.com/blog/building-a-social-app-with-spin-1
// https://www.thorsten-hans.com/master-configuration-data-in-fermyon-spin/
