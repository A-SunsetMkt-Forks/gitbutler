use std::path::Path;

use but_settings::AppSettings;
use gitbutler_command_context::CommandContext;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectStatus {
    /// List of stacks applied to the project's workspace
    stacks: Vec<but_workspace::ui::StackEntry>,
    /// Unified diff changes that could be committed.
    changes: Vec<but_core::ui::FlatChangeUnifiedDiff>,
}

pub fn project_status(project_dir: &Path) -> anyhow::Result<ProjectStatus> {
    let repo = crate::mcp_internal::project::project_repo(project_dir)?;

    let worktree = but_core::diff::worktree_changes(&repo)?;
    let diff = unified_diff_for_changes(
        &repo,
        worktree.changes.clone(),
        crate::mcp_internal::UI_CONTEXT_LINES,
    )?;

    let stacks = list_applied_stacks(project_dir)?;
    let flat_changes: but_core::ui::FlatUnifiedWorktreeChanges = (&diff).into();

    let serializable = ProjectStatus {
        stacks,
        changes: flat_changes.changes,
    };
    Ok(serializable)
}

fn list_applied_stacks(current_dir: &Path) -> anyhow::Result<Vec<but_workspace::ui::StackEntry>> {
    let project = crate::mcp_internal::project::project_from_path(current_dir)?;
    let ctx = CommandContext::open(&project, AppSettings::default())?;

    let repo = ctx.gix_repo_for_merging_non_persisting()?;
    let meta = crate::mcp_internal::project::ref_metadata_toml(ctx.project())?;
    but_workspace::stacks_v3(&repo, &meta, but_workspace::StacksFilter::InWorkspace)
}

fn unified_diff_for_changes(
    repo: &gix::Repository,
    changes: Vec<but_core::TreeChange>,
    context_lines: u32,
) -> anyhow::Result<Vec<(but_core::TreeChange, but_core::UnifiedDiff)>> {
    changes
        .into_iter()
        .map(|tree_change| {
            tree_change
                .unified_diff(repo, context_lines)
                .map(|diff| (tree_change, diff))
        })
        .collect::<Result<Vec<_>, _>>()
}
