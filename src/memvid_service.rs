pub mod base;

use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{ServerHandler, tool};
use rmcp::{handler::server::tool::ToolRouter, tool_handler, tool_router};

use crate::types::{CreateParams, PutBytesParams};

#[derive(Clone, Debug)]
pub struct MemvidService {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl MemvidService {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Create new memory file")]
    pub fn create(
        &self,
        Parameters(CreateParams { filepath }): Parameters<CreateParams>,
    ) -> String {
        tracing::info!("Creating memory file at: {}", filepath);
        base::create(&filepath)
    }

    #[tool(description = "Add data as string to memory file")]
    pub fn add(
        &self,
        Parameters(PutBytesParams { data, options }): Parameters<PutBytesParams>,
    ) -> String {
        tracing::info!("Adding data to memory file, size: {} bytes", data.len());
        match base::put_bytes(data.as_bytes(), options) {
            Ok(frame_id) => format!("Data added successfully with Frame ID: {}", frame_id),
            Err(e) => format!("Failed to add data: {:?}", e),
        }
    }
}

#[tool_handler]
impl ServerHandler for MemvidService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Tools to use Memvid for storing knowledge or data".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
