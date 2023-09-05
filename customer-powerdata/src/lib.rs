use anyhow::Result;
//use serde::Serialize;
use spin_sdk::{
    http::{Request, Response, Router},
    http_component,
};


/// A simple Spin HTTP component.
#[http_component]
fn handle_customer_powerdata(req: Request) -> Result<Response> {
    let mut router = Router::default();
    /*router.get("/", handle_get_all);
    router.get("/:id", handle_get_by_id);
    router.post("/", handle_create);
    router.put("/:id", handle_update_by_id);
    router.delete("/:id", handle_delete_by_id);
    //router.add("/healthz/readiness", http::Method::GET, handle_healthz);
    
    router.handle(req)
    */
    println!("{:?}", req.headers());
    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon".into()))?)
        
}
