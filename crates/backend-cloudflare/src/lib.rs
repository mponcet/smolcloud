use backend::config;
use backend::router;
use bindings_cloudflare::{CloudflareBindings, KV, R2, SecretError, SecretFn};

use axum::body::Body;
use axum::http::Response;
use tower_service::Service;
use tracing_subscriber::fmt::format::Pretty;
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_web::{MakeConsoleWriter, performance_layer};
use worker::*;

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
    let bucket: R2 = env.bucket(config::BUCKET_NAME)?.into();
    let kv: KV = env.kv(config::KV_NAME)?.into();
    let secret: Box<SecretFn> = Box::new(move |name| {
        env.secret(name)
            .map(|s| s.to_string().into())
            .map_err(|e| SecretError {
                message: format!("failed to get secret '{name}'"),
                source: e.into(),
            })
    });
    let bindings = CloudflareBindings::new(bucket, kv, secret);
    Ok(router(bindings).call(req).await?)
}
