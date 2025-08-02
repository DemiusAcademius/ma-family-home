use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn test_root_endpoint() {
    // Create the router using the function from the app module
    let router = my_family_home::rest::create_router();

    // Create a request to the root endpoint (now at /api)
    let request = Request::builder()
        .uri("/api")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    // Send the request to the router and get the response
    let response = router.oneshot(request).await.unwrap();

    // Check that the response has a 200 OK status
    assert_eq!(response.status(), StatusCode::OK);

    // Get the response body as bytes (with a 1MB limit)
    let body = to_bytes(response.into_body(), 1048576).await.unwrap();

    // Convert the bytes to a string and check the content
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(body_str, "Hello, world!");
}

#[tokio::test]
async fn test_readiness_endpoint() {
    // Create the router
    let router = my_family_home::rest::create_router();

    // Create a request to the readiness endpoint
    let request = Request::builder()
        .uri("/readiness")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    // Send the request to the router and get the response
    let response = router.oneshot(request).await.unwrap();

    // Check that the response has a 200 OK status
    assert_eq!(response.status(), StatusCode::OK);

    // Get the response body
    let body = to_bytes(response.into_body(), 1048576).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Check that the response body is "OK"
    assert_eq!(body_str, "OK");
}

#[tokio::test]
async fn test_liveness_endpoint() {
    // Create the router
    let router = my_family_home::rest::create_router();

    // Create a request to the liveness endpoint
    let request = Request::builder()
        .uri("/liveness")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    // Send the request to the router and get the response
    let response = router.oneshot(request).await.unwrap();

    // Check that the response has a 200 OK status
    assert_eq!(response.status(), StatusCode::OK);

    // Get the response body
    let body = to_bytes(response.into_body(), 1048576).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Check that the response body is "OK"
    assert_eq!(body_str, "OK");
}