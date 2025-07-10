pub use opentelemetry;
pub use opentelemetry_sdk;

use opentelemetry_sdk::{propagation::TraceContextPropagator, trace::Sampler, Resource};
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

//
fn otlp_with_resource(trace_resource: Resource, endpoint: String) -> opentelemetry_sdk::trace::Tracer {
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());

    let trace_config = opentelemetry_sdk::trace::config()
    .with_resource(trace_resource)
    .with_sampler(Sampler::AlwaysOn);

    tracing::info!("Tracing initialized");
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint)
        )
        .with_trace_config(trace_config)
        .install_batch(opentelemetry_sdk::runtime::Tokio).unwrap()
}

pub fn init_tracing(tracing_resource: Resource, endpoint: String) {
    let env_filter = EnvFilter::from_default_env();
    let tracing_layer = tracing_opentelemetry::layer().with_tracer(otlp_with_resource(tracing_resource, endpoint));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer()) // required for the `tracing` macros to be logged to stdout
        .with(tracing_layer).init();
}