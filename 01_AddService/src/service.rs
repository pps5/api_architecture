pub trait IHelloService {
    fn get_hello_string(&self, name: Option<String>) -> String;
}

pub struct HelloService;

impl IHelloService for HelloService {
    fn get_hello_string(&self, args: Option<String>) -> String {
        match args {
            None => "Please give me your name...".to_string(),
            Some(name) => format!("Hello {}", &name),
        }
    }
}

pub fn get_hello_service() -> &'static impl IHelloService {
    &HelloService
}
