use axum::Router;
use axum::routing::{get, post};
use opentelemetry::{global, KeyValue};
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, SdkTracerProvider};
use opentelemetry_semantic_conventions::{
    attribute::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod logic;

#[tokio::main]
async fn main() {

    let _guard = init_tracing_subscriber();

    info!("STARTING Fast Axum - Domains API");

    let router = Router::new() 
        .route("/", get(|| async {
            "Healthy!"
        }))
        .route("/domains", post(logic::domains::get_domains));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap()
}

fn init_tracing() -> SdkTracerProvider {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .build()
        .unwrap();

    SdkTracerProvider::builder()
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(1.0))))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource())
        .with_batch_exporter(exporter)
        .build()
}

fn resource() -> Resource {
    Resource::builder()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .with_schema_url([
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
        ], SCHEMA_URL)
        .build()
}

fn init_metrics() -> SdkMeterProvider {
    let exporter = opentelemetry_otlp::MetricExporter::builder()
        .with_tonic()
        .with_temporality(opentelemetry_sdk::metrics::Temporality::default())
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter)
        .with_interval(std::time::Duration::from_secs(30))
        .build();

    // For debugging in development
    let stdout_reader =
        PeriodicReader::builder(opentelemetry_stdout::MetricExporter::default()).build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource())
        .with_reader(reader)
        .with_reader(stdout_reader)
        .build();

    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

fn init_tracing_subscriber() -> OtelGuard {
    let tracer_provider = init_tracing();
    let meter_provider = init_metrics();

    let tracer = tracer_provider.tracer("tracing-otel-subscriber");

    tracing_subscriber::registry()
        // The global level filter prevents the exporter network stack
        // from reentering the globally installed OpenTelemetryLayer with
        // its own spans while exporting, as the libraries should not use
        // tracing levels below DEBUG. If the OpenTelemetry layer needs to
        // trace spans and events with higher verbosity levels, consider using
        // per-layer filtering to target the telemetry layer specifically,
        // e.g. by target matching.
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(tracing_subscriber::fmt::layer())
        .with(MetricsLayer::new(meter_provider.clone()))
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    OtelGuard {
        tracer_provider,
        meter_provider,
    }
}

struct OtelGuard {
    tracer_provider: SdkTracerProvider,
    meter_provider: SdkMeterProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }
        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}