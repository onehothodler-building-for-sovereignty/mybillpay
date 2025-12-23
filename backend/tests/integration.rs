use axum::http::Method;
use hyper::{Body, body::to_bytes};
use axum::http::Request;
use tower::util::ServiceExt;

#[tokio::test]
async fn health_and_items_via_router() {
    let app = mybillpay_backend::app();

    // GET /health
    let req: Request<Body> = Request::builder()
        .method(Method::GET)
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.expect("request failed");
    let bytes = to_bytes(resp.into_body()).await.expect("read body");
    let body = String::from_utf8(bytes.to_vec()).expect("utf8");
    assert_eq!(body, "OK");

    // GET /items
    let req: Request<Body> = Request::builder()
        .method(Method::GET)
        .uri("/items")
        .body(Body::empty())
        .unwrap();
    let resp = app.clone().oneshot(req).await.expect("request failed");
    let bytes = to_bytes(resp.into_body()).await.expect("read body");
    let items: Vec<mybillpay_backend::routes::Item> = serde_json::from_slice(&bytes).expect("invalid json");
    assert!(!items.is_empty());

    // POST /items - send JSON and expect the same item echoed back
    let new = mybillpay_backend::routes::Item { id: 42, name: "Posted".into() };
    let body_bytes = serde_json::to_vec(&new).expect("serialize");
    let req: Request<Body> = Request::builder()
        .method(Method::POST)
        .uri("/items")
        .header("content-type", "application/json")
        .body(Body::from(body_bytes))
        .unwrap();
    let resp = app.clone().oneshot(req).await.expect("request failed");
    let bytes = to_bytes(resp.into_body()).await.expect("read body");
    let echoed: mybillpay_backend::routes::Item = serde_json::from_slice(&bytes).expect("invalid json");
    assert_eq!(echoed.id, new.id);
    assert_eq!(echoed.name, new.name);
}
