mod memvid_service;
mod memvid_wrapper;
mod types;
use anyhow::Result;
use memvid_service::MemvidService;
use rmcp::{
    ServiceExt,
    transport::{
        StreamableHttpService, stdio, streamable_http_server::session::local::LocalSessionManager,
    },
};
use std::env;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();
    // Get transport mode from environment variable or command line argument
    let transport_mode = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("TRANSPORT_MODE").unwrap_or_else(|_| "stdio".to_string()));

    match transport_mode.as_str() {
        "stdio" => run_stdio().await,
        "http" | "streamhttp" => run_streamable_http().await,
        _ => {
            eprintln!("Usage: {} [stdio|http]", env::args().next().unwrap());
            std::process::exit(1);
        }
    }
}

async fn run_stdio() -> anyhow::Result<()> {
    let server = MemvidService::new();
    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("stdio serving error: {:?}", e);
    })?;

    tracing::info!("Memvid MCP running in STDIO mode. Press Ctrl+C to exit.");
    service.waiting().await?;
    Ok(())
}

async fn run_streamable_http() -> anyhow::Result<()> {
    let http_bind_address: String = if let Some(add) = env::args().nth(2) {
        add
    } else {
        String::from("127.0.0.1:8080")
    };

    let service = StreamableHttpService::new(
        || Ok(MemvidService::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );

    let router = axum::Router::new().nest_service("/mcp", service);
    let tcp_listener = tokio::net::TcpListener::bind(http_bind_address.as_str()).await?;

    tracing::info!(
        "Memvid MCP started at http://{}/mcp",
        http_bind_address.as_str()
    );
    tracing::info!("Press Ctrl+C to shutdown");

    let _ = axum::serve(tcp_listener, router)
        .with_graceful_shutdown(async { tokio::signal::ctrl_c().await.unwrap() })
        .await;

    Ok(())
}
