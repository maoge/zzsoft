use ntex::web::{get, ServiceConfig};

#[allow(dead_code)]
fn test_handler_regist(cfg: &mut ServiceConfig) {
    cfg.service(test);
}

#[get("/test")]
async fn test() -> &'static str {
    "Hello world!"
}
