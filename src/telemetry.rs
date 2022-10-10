use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to spell out the
/// actual type of the returned subscriber, which is indeed quite complex.
/// We need to explicitly call out that the returned subscriber is `Send` and `Sync`
/// to make it possible to pass it to `init_subscriber` later on.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    // This "weird" syntax is a higher-ranked trait bound (HRTB)
    // It basically means that Sink implements the `MakeWriter` trait for all choices
    // of the lifetime parameter `'a`.
    // We are introducing the "Sink" so that for each `cargo test` we don't see a ton
    // of messages giving the info-level output. This is to bring out a functionality
    // similar to the standard `cargo test -- --nocapture` (to suppress println/print statements.
    // With sink, the formatting_layer setting below is changed from std::io::stdout
    //
    // In our test suite we will choose the sink dynamically according to an env variable, TEST_LOG.
    // If TEST_LOG is set, we use std::io::stdout and if not set, we will send all logs into the
    // void using std::io::sink. This is our own home-made version of the --nocapture flag.
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    // let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
/// It should be called only once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
