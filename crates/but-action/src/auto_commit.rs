use anyhow::Context;
use but_tools::workspace::commit_toolset;
use gitbutler_command_context::CommandContext;

use crate::OpenAiProvider;

pub fn auto_commit(
    app_handle: &tauri::AppHandle,
    ctx: &mut CommandContext,
    openai: &OpenAiProvider,
    changes: Vec<but_core::TreeChange>,
) -> anyhow::Result<()> {
    let repo = ctx.gix_repo()?;

    let project_status = crate::get_project_status(ctx, &repo, Some(changes))?;
    let serialized_status = serde_json::to_string_pretty(&project_status)
        .context("Failed to serialize project status")?;

    let mut toolset = commit_toolset(ctx, Some(app_handle))?;

    let system_message ="
        You are an expert in grouping and committing file changes into logical units for version control.
        When given the status of a project, you should be able to identify related changes and suggest how they should be grouped into commits.
        It's also important to suggest a branch for each group of changes.
        The branch can be either an existing branch or a new one.
        In order to determine the branch, you should consider diffs, the assignments and the dependency locks, if any.
        Before committing, you should create the branches that are needed for the changes, if they don't already exist.
        ";

    let prompt = format!("
        Please, figure out how to group the file changes into logical units for version control and commit them.
        Follow these steps:
        1. Take a look at the exisiting branches (stack heads) and the file changes. You can see all this information in the **project status** below.
        2. Determine which are the related changes that should be grouped together. You can do this by looking at the diffs, assignments, and dependency locks, if any.
        3. Determine if any new branches need to be created. If so, create them using the provided tool.
        4. For each group of changes, create a commit (using the provided tool) with a detailed summary of the changes in the group (not the intention, but an overview of the actual changes made and why they are related).
        5. When you're done, only send the message 'done'

        Here is the project status:
        <project_status>
                {}
        </project_status>
    ", serialized_status);

    crate::openai::tool_calling_loop(openai, system_message, &prompt, &mut toolset)?;

    Ok(())
}
