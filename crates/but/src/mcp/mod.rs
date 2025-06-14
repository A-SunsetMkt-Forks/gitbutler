use std::path::PathBuf;

use anyhow::Result;
use but_action::{ActionHandler, OpenAiProvider};
use but_settings::AppSettings;
use gitbutler_command_context::CommandContext;
use gitbutler_project::Project;
use rmcp::{
    Error as McpError, ServerHandler, ServiceExt,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    schemars, tool,
};

use crate::metrics::{Event, EventKind, Metrics};

pub(crate) async fn start(app_settings: AppSettings) -> Result<()> {
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = Mcp::new(app_settings).serve(transport).await?;
    service.waiting().await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Mcp {
    app_settings: AppSettings,
    metrics: Metrics,
    openai: Option<OpenAiProvider>,
}

#[tool(tool_box)]
impl Mcp {
    pub fn new(app_settings: AppSettings) -> Self {
        let metrics = Metrics::new_with_background_handling(&app_settings);
        let openai = OpenAiProvider::with(None);
        Self {
            app_settings,
            metrics,
            openai,
        }
    }

    #[tool(
        description = "Update commits on the current branch based on the prompt used to modify the codebase and a summary of the changes made."
    )]
    pub fn gitbutler_update_branches(
        &self,
        #[tool(aggr)] request: GitButlerUpdateBranchesRequest,
    ) -> Result<CallToolResult, McpError> {
        self.metrics.capture(Event::new(
            EventKind::Mcp,
            vec![("gitbutler_update_branches".to_string(), None)],
        ));
        if request.changes_summary.is_empty() {
            return Err(McpError::invalid_request(
                "ChangesSummary cannot be empty".to_string(),
                None,
            ));
        }
        if request.full_prompt.is_empty() {
            return Err(McpError::invalid_request(
                "FullPrompt cannot be empty".to_string(),
                None,
            ));
        }
        if request.current_working_directory.is_empty() {
            return Err(McpError::invalid_request(
                "CurrentWorkingDirectory cannot be empty".to_string(),
                None,
            ));
        }

        let repo_path = PathBuf::from(request.current_working_directory.clone());
        let project = Project::from_path(&repo_path).expect("Failed to create project from path");
        let ctx = &mut CommandContext::open(&project, self.app_settings.clone())
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        let response = but_action::handle_changes(
            ctx,
            &self.openai,
            &request.changes_summary,
            Some(request.full_prompt),
            ActionHandler::HandleChangesSimple,
        )
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::json(response)?]))
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct GitButlerUpdateBranchesRequest {
    #[schemars(description = "The exact prompt that the user gave to generate these changes")]
    pub full_prompt: String,
    #[schemars(
        description = "A short bullet list of important things that were changed in the codebase and why"
    )]
    pub changes_summary: String,
    #[schemars(
        description = "The full root path of the Git project the agent is actively working in"
    )]
    pub current_working_directory: String,
}

#[tool(tool_box)]
impl ServerHandler for Mcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("GitButler MCP server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "GitButler MCP Server".into(),
                version: "1.0.0".into(),
            },
            protocol_version: ProtocolVersion::LATEST,
        }
    }
}
