use memvid_core::Memvid;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::{ServerHandler, tool};
use rmcp::{handler::server::tool::ToolRouter, tool_handler, tool_router};

use crate::types::CreateParams;

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

    #[tool(description = "A test method for MemvidService")]
    pub fn test_method(&self) -> String {
        "This is a test method".into()
    }

    #[tool(description = "Create new memory file")]
    pub fn create(
        &self,
        Parameters(CreateParams { filepath }): Parameters<CreateParams>,
    ) -> String {
        tracing::info!("Creating memory file at: {}", filepath);
        match Memvid::create(&filepath) {
            Ok(_) => {
                let msg = format!("Memory file created successfully at: {}", filepath);
                tracing::info!("{}", msg);
                msg
            }
            Err(e) => {
                let msg = format!("Failed to create memory file at {}: {:?}", filepath, e);
                tracing::error!("{}", msg);
                msg
            }
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
