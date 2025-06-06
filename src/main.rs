#[macro_use]
mod local_macros {
    pub const DEFAULT_TGT: &str = "my_app";

    /// you may want to define standard macros for your app with a different
    /// default target name than the package name
    macro_rules! info {
        ($($arg:tt)*) => {
            tracing::info!(target: local_macros::DEFAULT_TGT, $($arg)*)
        }
    }
}

#[derive(Debug)]
struct DebugVal {
    value: i32,
}

impl std::fmt::Display for DebugVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Val: {}", self.value)
    }
}

fn main() {
    setup_tracing();

    tracing::info!("plain info log");
    tracing::warn!("plain warn log");
    tracing::debug!("plain debug log");
    tracing::trace!("plain trace log");
    tracing::error!("plain error log");

    println!("Hello, World!");

    let s = "Earth".to_string();
    useful_parameter_variations(&s);
    instrumented_function(s);
    useful_target_variations();
    span_variations();
    noisy_logs();
    others();
}

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt::Subscriber;
    use tracing_subscriber::fmt;

    // default or take value from RUST_LOG
    const DEFAULT_RUST_LOG: &str = "trace,noisy=error";
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(DEFAULT_RUST_LOG))
        .unwrap_or_else(|err| {
            eprintln!("Failed to parse RUST_LOG: {err}, defaulting to an empty EnvFilter");
            EnvFilter::new("")
        });

    Subscriber::builder()
        .with_env_filter(env_filter)
        .with_timer(fmt::time::uptime())
        .with_span_events(fmt::format::FmtSpan::CLOSE)
        .try_init()
        .expect("unable to setup tracing");
}

pub fn setup_tracing_alt() {
    use tracing_appender::non_blocking;
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt;
    use tracing_subscriber::Layer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    // Create a file appender specifically for "my_span" logs
    let my_span_file = std::fs::File::create("my_span.log")
        .expect("Failed to create my_span.log");
    let (my_span_writer, _guard) = non_blocking(my_span_file);

    // Create a layer that only logs events from "my_span"
    let my_span_layer = fmt::layer()
        .with_writer(my_span_writer)
        .with_filter(tracing_subscriber::filter::filter_fn(|metadata| {
            // Only include events where the span name is "my_span"
            metadata.is_span() && metadata.name() == "my_span"
        }));

    // Create a stdout layer for all logs
    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::from_default_env());

    // Combine the layers and initialize the subscriber
    tracing_subscriber::registry()
        .with(my_span_layer)
        .with(stdout_layer)
        .init();
}

#[tracing::instrument]
fn instrumented_function(a: String) {
    tracing::info!("instrumeted_function called with {}", a);
    tracing::info!(%a, "instrumeted_function called");
    println!("Hello, {a} (instrumented)");
}

/// Demonstrates various ways to use tracing parameters
fn useful_parameter_variations(a: &str) {
    tracing::info!("regular println usage a={}", a);
    tracing::info!(name = a, "named param");
    tracing::info!(a, "quick named param");

    let dbg_val = DebugVal { value: 42 };
    tracing::info!(%dbg_val, "display print param"); // use display trait print
    tracing::info!(?dbg_val, "debug print param"); // use debug trait print
    tracing::info!(val = dbg_val.value, "named debug print param"); // named param w/ expression
}

fn useful_target_variations() {
    tracing::info!("regular println usage");

    // targets can be individually controlled via RUST_LOG
    // RUST_LOG
    tracing::info!(target: "my_target", "targeted log");
    tracing::warn!(target: "my_target", "targeted log with parameter {}", "parameter");

    // RUST_LOG=my_app=off  will shut off this log, but
    // RUST_LOG=my_app::level2=info,my_app=off will show the my_app::level2 targetd log
    tracing::info!(target: "my_app::level2", "multi level targetd log");
    tracing::info!(target: "my_app::sub2", "multi level targetd log");
}

fn span_variations() {
    let span = tracing::span!(tracing::Level::INFO, "my_span");
    let _guard = span.enter();
    tracing::info!("inside span");

    info!("custom, inside span");
}

fn noisy_logs() {
    tracing::info!("noisy log 1");
    tracing::info!("noisy log 2");
    tracing::error!("noisy log err example");
    tracing::info!("noisy log 3");
}

fn others() {
    // use a differnt default target defined in local_macros::info
    info!("in fn others, custom macro log");
}