use axum::body::Body;
use axum::http::Response;
use tower_service::Service;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::{MakeConsoleWriter, performance_layer};
use worker::*;

use backend::router;
use bindings_cloudflare::{CloudflareBindings, R2};

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false)
        .with_timer(UtcTime::rfc_3339())
        .with_writer(MakeConsoleWriter);
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<Response<Body>> {
    let bucket: R2 = env.bucket("mybucket")?.into();
    let bindings = CloudflareBindings::new(bucket);
    Ok(router(bindings).call(req).await?)
}
