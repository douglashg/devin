use actix_web::{test, App};
use hello_rust;

#[actix_web::test]
async fn test_hello_endpoint() {
    let app = test::init_service(
        App::new()
            .service(hello_rust::hello)
    ).await;

    let req = test::TestRequest::get()
        .uri("/hello")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello from Rust REST");
}