#[utoipa::path(get,
responses(
(status = 200, description = "Successfully retrieved prometheus metrics.", content_type = "text/plain; version=0.0.4"),
),
path = "/metrics",
tag = "Metrics",
)]
#[allow(dead_code)]
pub async fn metrics() {
    unreachable!("This function is used for documentation only and should never be called!")
}
