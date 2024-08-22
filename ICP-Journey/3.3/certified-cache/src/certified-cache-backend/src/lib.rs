
use ic_http_certification::cel::{CelExpression, DefaultCelExpression, create_cel_expr};
use ic_http_certification::{HttpCertification, HttpResponse, DefaultCelBuilder, DefaultResponseCertification};
use ic_http_certification::{DefaultCelBuilder, DefaultResponseCertification};

let certification = CelExpression::Default(DefaultCelExpression::Skip);
let cel_expr = create_cel_expr(&certification);

let cel_expr = DefaultCelBuilder::full_certification()
    .with_request_headers(vec!["Accept", "Accept-Encoding", "If-None-Match"])
    .with_request_query_parameters(vec!["foo", "bar", "baz"])
    .with_response_certification(DefaultResponseCertification::certified_response_headers(vec![
        "Cache-Control",
        "ETag",
    ]))
    .build();


let cel_expr = DefaultCelBuilder::response_only_certification()
    .with_response_certification(DefaultResponseCertification::certified_response_headers(vec![
        "Cache-Control",
        "ETag",
    ]))
    .build();

let response = HttpResponse {
    status_code: 200,
    headers: vec![
        ("Cache-Control".to_string(), "no-cache".to_string()),
        ("ETag".to_string(), "123456789".to_string()),
        ("IC-CertificateExpression".to_string(), cel_expr.to_string()),
    ],
    body: vec![1, 2, 3, 4, 5, 6],
    upgrade: None,
};

let certification = HttpCertification::response_only(&cel_expr, &response, None).unwrap();