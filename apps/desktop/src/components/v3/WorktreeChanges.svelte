<script lang="ts">
	import CardOverlay from '$components/CardOverlay.svelte';
	import ScrollableContainer from '$components/ConfigurableScrollableContainer.svelte';
	import Dropzone from '$components/Dropzone.svelte';
	import ReduxResult from '$components/ReduxResult.svelte';
	import FileList from '$components/v3/FileList.svelte';
	import FileListMode from '$components/v3/FileListMode.svelte';
	import WorktreeTipsFooter from '$components/v3/WorktreeTipsFooter.svelte';
	import noChanges from '$lib/assets/illustrations/no-changes.svg?raw';
	import { createCommitStore } from '$lib/commits/contexts';
	import { UncommitDzHandler } from '$lib/commits/dropHandler';
	import { Focusable } from '$lib/focus/focusManager.svelte';
	import { focusable } from '$lib/focus/focusable.svelte';
	import { previousPathBytesFromTreeChange } from '$lib/hunks/change';
	import { ChangeSelectionService, type SelectedFile } from '$lib/selection/changeSelection.svelte';
	import { IdSelection } from '$lib/selection/idSelection.svelte';
	import { StackService } from '$lib/stacks/stackService.svelte';
	import { UiState } from '$lib/state/uiState.svelte';
	import { TestId } from '$lib/testing/testIds';
	import { WorktreeService } from '$lib/worktree/worktreeService.svelte';
	import { inject } from '@gitbutler/shared/context';
	import Badge from '@gitbutler/ui/Badge.svelte';
	import Button from '@gitbutler/ui/Button.svelte';
	import Checkbox from '@gitbutler/ui/Checkbox.svelte';
	import { untrack } from 'svelte';

	type Props = {
		projectId: string;
		stackId?: string;
		active: boolean;
	};

	let { projectId, stackId, active }: Props = $props();

	const [changeSelection, worktreeService, uiState, stackService, idSelection] = inject(
		ChangeSelectionService,
		WorktreeService,
		UiState,
		StackService,
		IdSelection
	);

	const uncommitDzHandler = $derived(new UncommitDzHandler(projectId, stackService, uiState));

	const projectState = $derived(uiState.project(projectId));
	const drawerPage = $derived(projectState.drawerPage.get());
	const isCommitting = $derived(drawerPage.current === 'new-commit');
	const stackState = $derived(stackId ? uiState.stack(stackId) : undefined);

	const defaultBranchResult = $derived(
		stackId !== undefined ? stackService.defaultBranch(projectId, stackId) : undefined
	);
	const defaultBranchName = $derived(defaultBranchResult?.current.data);

	const selectedChanges = changeSelection.list();
	const noChangesSelected = $derived(selectedChanges.current.length === 0);
	const changesResult = $derived(worktreeService.getChanges(projectId));
	const affectedPaths = $derived(changesResult.current.data?.map((c) => c.path));

	const filesFullySelected = $derived(
		changeSelection.every(affectedPaths ?? [], (f) => f.type === 'full')
	);

	const filesPartiallySelected = $derived(!noChangesSelected && !filesFullySelected);

	// TODO: Make this go away.
	createCommitStore(undefined);

	/** Clear any selected changes that no longer exist. */
	$effect(() => {
		if (affectedPaths) {
			untrack(() => {
				changeSelection.retain(affectedPaths);
				idSelection.retain(affectedPaths);
			});
		}
	});

	let listMode: 'list' | 'tree' = $state('list');

	function selectEverything() {
		const affectedPaths =
			changesResult.current.data?.map(
				(c) => [c.path, c.pathBytes, previousPathBytesFromTreeChange(c)] as const
			) ?? [];
		const files: SelectedFile[] = affectedPaths.map(([path, pathBytes, previousPathBytes]) => ({
			path,
			pathBytes,
			previousPathBytes,
			type: 'full'
		}));
		changeSelection.addMany(files);
	}

	function updateCommitSelection() {
		if (!noChangesSelected) return;
		// If no changes are selected, select everything.
		selectEverything();
	}

	function startCommit() {
		updateCommitSelection();
		projectState.drawerPage.set('new-commit');
		if (defaultBranchName) {
			stackState?.selection.set({ branchName: defaultBranchName });
		}
	}

	function toggleGlobalCheckbox() {
		if (noChangesSelected) {
			selectEverything();
			return;
		}
		changeSelection.clear();
	}

	let scrollTopIsVisible = $state(true);
	let scrollBottomIsVisible = $state(true);
</script>

<Dropzone handlers={[uncommitDzHandler]} maxHeight>
	{#snippet overlay({ hovered, activated })}
		<CardOverlay {hovered} {activated} label="Uncommit changes" />
	{/snippet}

	<div
		class="uncommitted-changes-wrap"
		use:focusable={{ id: Focusable.UncommittedChanges, parentId: Focusable.ViewportLeft }}
	>
		<ReduxResult {stackId} {projectId} result={changesResult.current}>
			{#snippet children(changes, { stackId, projectId })}
				{#if changes.length > 0}
					<div
						data-testid={TestId.UncommittedChanges_Header}
						class="worktree-header"
						class:sticked-top={!scrollTopIsVisible}
					>
						<div class="worktree-header__general">
							{#if isCommitting}
								<Checkbox
									checked={filesPartiallySelected || filesFullySelected}
									indeterminate={filesPartiallySelected}
									small
									onchange={toggleGlobalCheckbox}
								/>
							{/if}
							<div class="worktree-header__title truncate">
								<h3 class="text-14 text-semibold truncate">Uncommitted</h3>
								{#if changes.length > 0}
									<Badge>{changes.length}</Badge>
								{/if}
							</div>
						</div>
						<FileListMode bind:mode={listMode} persist="uncommitted" />
					</div>

					<ScrollableContainer
						autoScroll={false}
						onscrollTop={(visible) => {
							scrollTopIsVisible = visible;
						}}
						onscrollEnd={(visible) => {
							scrollBottomIsVisible = visible;
						}}
					>
						<div data-testid={TestId.UncommittedChanges_FileList} class="uncommitted-changes">
							<FileList
								selectionId={{ type: 'worktree' }}
								showCheckboxes={isCommitting}
								draggableFiles
								{projectId}
								{stackId}
								{changes}
								{listMode}
								{active}
							/>
						</div>
					</ScrollableContainer>
					<div class="start-commit" class:sticked-bottom={!scrollBottomIsVisible}>
						<Button
							testId={TestId.StartCommitButton}
							kind={isCommitting ? 'outline' : 'solid'}
							type="button"
							size="cta"
							wide
							disabled={isCommitting || defaultBranchResult?.current.isLoading}
							onclick={startCommit}
						>
							Start a commit…
						</Button>
					</div>
				{:else}
					<div
						data-testid={TestId.UncommittedChanges_Header}
						class="worktree-header"
						class:sticked-top={!scrollTopIsVisible}
					>
						<div class="worktree-header__general">
							<div class="worktree-header__title truncate">
								<h3 class="text-14 text-semibold truncate">Uncommitted</h3>

								<Badge>0</Badge>
							</div>
						</div>
						<FileListMode bind:mode={listMode} persist="uncommitted" />
					</div>

					<div class="uncommitted-changes__empty">
						<div class="uncommitted-changes__empty__placeholder">
							{@html noChanges}
							<p class="text-13 text-body uncommitted-changes__empty__placeholder-text">
								You're all caught up!<br />
								No files need committing
							</p>
						</div>
						<WorktreeTipsFooter />
					</div>
				{/if}
			{/snippet}
		</ReduxResult>
	</div>
</Dropzone>

<style>
	.uncommitted-changes-wrap {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
		border: 1px solid var(--clr-border-2);
		border-radius: var(--radius-ml);
		background-color: var(--clr-bg-1);
	}

	.worktree-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		width: 100%;
		padding: 10px 10px 10px 14px;
		gap: 8px;
		background-color: var(--clr-bg-1);
		text-wrap: nowrap;
		white-space: nowrap;
	}

	.worktree-header__general {
		display: flex;
		align-items: center;
		overflow: hidden;
		gap: 10px;
	}

	.worktree-header__title {
		display: flex;
		align-items: center;
		gap: 6px;
	}

	.uncommitted-changes {
		display: flex;
		flex: 1;
		flex-direction: column;
		width: 100%;
	}

	.start-commit {
		position: sticky;
		bottom: -1px;
		padding: 14px;
		background-color: var(--clr-bg-1);
	}

	.uncommitted-changes__empty {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.uncommitted-changes__empty__placeholder {
		display: flex;
		flex: 1;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 0 20px 40px;
		gap: 20px;
	}

	.uncommitted-changes__empty__placeholder-text {
		color: var(--clr-text-3);
		text-align: center;
	}

	/* MODIFIERS */
	.sticked-top {
		border-bottom: 1px solid var(--clr-border-2);
	}

	.sticked-bottom {
		border-top: 1px solid var(--clr-border-2);
	}
</style>
