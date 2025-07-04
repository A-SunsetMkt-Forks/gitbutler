<script lang="ts">
	import ReduxResult from '$components/ReduxResult.svelte';
	import FileListItemWrapper from '$components/v3/FileListItemWrapper.svelte';
	import UnifiedDiffView from '$components/v3/UnifiedDiffView.svelte';
	import { OplogService } from '$lib/history/oplogService.svelte';
	import { DiffService } from '$lib/hunks/diffService.svelte';
	import { UncommittedService } from '$lib/selection/uncommittedService.svelte';
	import { StackService } from '$lib/stacks/stackService.svelte';
	import { combineResults } from '$lib/state/helpers';
	import { WorktreeService } from '$lib/worktree/worktreeService.svelte';
	import { inject } from '@gitbutler/shared/context';
	import type { Modification } from '$lib/hunks/change';
	import type { SelectedFile } from '$lib/selection/key';

	type Props = {
		selectedFile: SelectedFile;
		projectId: string;
		draggable?: boolean;
		onCloseClick: () => void;
	};

	const { selectedFile, projectId, onCloseClick, draggable }: Props = $props();

	const [diffService, stackService, worktreeService, uncommittedService, oplogService] = inject(
		DiffService,
		StackService,
		WorktreeService,
		UncommittedService,
		OplogService
	);

	const changeResult = $derived.by(() => {
		switch (selectedFile.type) {
			case 'commit':
				return stackService.commitChange(projectId, selectedFile.commitId, selectedFile.path);
			case 'branch':
				return stackService.branchChange({
					projectId,
					stackId: selectedFile.stackId,
					branchName: selectedFile.branchName,
					path: selectedFile.path
				});
			case 'worktree':
				uncommittedService.assignmentsByPath(selectedFile.stackId || null, selectedFile.path);
				return worktreeService.treeChangeByPath(projectId, selectedFile.path);
			case 'snapshot':
				return oplogService.diffWorktreeByPath({
					projectId,
					before: selectedFile.before,
					after: selectedFile.after,
					path: selectedFile.path
				});
		}
	});

	const change = $derived(changeResult.current.data);
	const diffResult = $derived(change ? diffService.getDiff(projectId, change) : undefined);
</script>

{#if diffResult?.current}
	<ReduxResult {projectId} result={combineResults(changeResult.current, diffResult.current)}>
		{#snippet children([change, diff], env)}
			{#if change}
				{@const isExecutable = (change.status.subject as Modification).flags}
				<div class="selected-change-item" data-remove-from-panning>
					<FileListItemWrapper
						selectionId={selectedFile}
						projectId={env.projectId}
						{change}
						{diff}
						{draggable}
						isHeader
						executable={!!isExecutable}
						listMode="list"
						{onCloseClick}
					/>
					<UnifiedDiffView
						projectId={env.projectId}
						stackId={'stackId' in selectedFile ? selectedFile.stackId : undefined}
						commitId={selectedFile.type === 'commit' ? selectedFile.commitId : undefined}
						{draggable}
						{change}
						{diff}
						selectable
						selectionId={selectedFile}
					/>
				</div>
			{/if}
		{/snippet}
	</ReduxResult>
{/if}

<style>
	.selected-change-item {
		display: flex;
		flex-direction: column;
		border-bottom: 1px solid var(--clr-border-2);
		background-color: var(--clr-bg-1);
	}
</style>
