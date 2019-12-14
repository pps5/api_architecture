use iron::prelude::*;
use iron::status;

mod service;
use service::IHelloService;
fn main() {
    let hello_service = service::get_hello_service();

    Iron::new(move |req: &mut Request| {
        use params::{Params, Value};

        let map = req.get_ref::<Params>().unwrap();

        match map.find(&["userName"]) {
            Some(&Value::String(ref name)) => Ok(Response::with((
                status::Ok,
                hello_service.get_hello_string(Some(name.to_string())),
            ))),
            _ => Ok(Response::with((
                status::BadRequest,
                hello_service.get_hello_string(None),
            ))),
        }
    })
    .http("localhost:3000")
    .expect("Cannot start server");
}
