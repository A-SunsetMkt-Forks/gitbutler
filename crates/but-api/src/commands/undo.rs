use anyhow::Context;
use gitbutler_command_context::CommandContext;
use gitbutler_oplog::entry::OperationKind;
use gitbutler_oplog::{OplogExt, entry::Snapshot};
use gitbutler_project::ProjectId;
use serde::Deserialize;

use crate::{IpcContext, error::Error};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSnapshotsParams {
    pub project_id: ProjectId,
    pub limit: usize,
    pub sha: Option<String>,
    pub exclude_kind: Option<Vec<OperationKind>>,
}

pub fn list_snapshots(
    ipc_ctx: &IpcContext,
    params: ListSnapshotsParams,
) -> Result<Vec<Snapshot>, Error> {
    let project = gitbutler_project::get(params.project_id).context("failed to get project")?;
    let ctx = CommandContext::open(&project, ipc_ctx.app_settings.get()?.clone())?;
    let snapshots = ctx.list_snapshots(
        params.limit,
        params
            .sha
            .map(|hex| hex.parse().map_err(anyhow::Error::from))
            .transpose()?,
        params.exclude_kind.unwrap_or_default(),
    )?;
    Ok(snapshots)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreSnapshotParams {
    pub project_id: ProjectId,
    pub sha: String,
}

pub fn restore_snapshot(ipc_ctx: &IpcContext, params: RestoreSnapshotParams) -> Result<(), Error> {
    let project = gitbutler_project::get(params.project_id).context("failed to get project")?;
    let ctx = CommandContext::open(&project, ipc_ctx.app_settings.get()?.clone())?;
    let mut guard = project.exclusive_worktree_access();
    ctx.restore_snapshot(
        params.sha.parse().map_err(anyhow::Error::from)?,
        guard.write_permission(),
    )?;
    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDiffParams {
    pub project_id: ProjectId,
    pub sha: String,
}

pub fn snapshot_diff(
    ipc_ctx: &IpcContext,
    params: SnapshotDiffParams,
) -> Result<Vec<but_core::ui::TreeChange>, Error> {
    let project = gitbutler_project::get(params.project_id).context("failed to get project")?;
    let ctx = CommandContext::open(&project, ipc_ctx.app_settings.get()?.clone())?;
    let diff = ctx.snapshot_diff(params.sha.parse().map_err(anyhow::Error::from)?)?;
    let diff: Vec<but_core::ui::TreeChange> = diff.into_iter().map(Into::into).collect();
    Ok(diff)
}
