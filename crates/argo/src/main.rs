use argo_ui::ArgoApp;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "argo=debug,info".into()),
        )
        .init();
    ArgoApp::launch();
}
