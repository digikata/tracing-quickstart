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
    instrumeted_function(s);
    useful_target_variations();
    span_variations();
    others();
}

fn setup_tracing() {
    use tracing_subscriber::EnvFilter;
    use tracing_subscriber::fmt::Subscriber;

    // default or take value from RUST_LOG
    const DEFAULT_RUST_LOG: &str = "trace";
    let default = DEFAULT_RUST_LOG.to_string().parse().unwrap_or_default();
    let env_filter = EnvFilter::builder()
        .with_default_directive(default)
        .from_env_lossy();
    Subscriber::builder()
        .with_env_filter(env_filter)
        .try_init()
        .expect("unable to setup tracing");
}

#[tracing::instrument]
fn instrumeted_function(a: String) {
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

fn others() {
    // use a differnt default target defined in local_macros::info
    info!("in fn others, custom macro log");
}
