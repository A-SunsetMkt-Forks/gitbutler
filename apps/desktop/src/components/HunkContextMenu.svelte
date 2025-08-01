<script lang="ts" module>
	export interface HunkContextItem {
		hunk: DiffHunk;
		selectedLines: LineId[] | undefined;
		beforeLineNumber: number | undefined;
		afterLineNumber: number | undefined;
	}

	export function isHunkContextItem(item: unknown): item is HunkContextItem {
		return typeof item === 'object' && item !== null && 'hunk' in item && isDiffHunk(item.hunk);
	}
</script>

<script lang="ts">
	import { ircEnabled } from '$lib/config/uiFeatureFlags';
	import { isDiffHunk, lineIdsToHunkHeaders, type DiffHunk } from '$lib/hunks/hunk';
	import { IRC_SERVICE } from '$lib/irc/ircService.svelte';
	import { vscodePath } from '$lib/project/project';
	import { PROJECTS_SERVICE } from '$lib/project/projectsService';
	import { SETTINGS } from '$lib/settings/userSettings';
	import { STACK_SERVICE } from '$lib/stacks/stackService.svelte';
	import { TestId } from '$lib/testing/testIds';
	import { getEditorUri, openExternalUrl } from '$lib/utils/url';
	import { inject } from '@gitbutler/shared/context';
	import { ContextMenu, ContextMenuItem, ContextMenuSection } from '@gitbutler/ui';
	import type { TreeChange } from '$lib/hunks/change';
	import type { LineId } from '@gitbutler/ui/utils/diffParsing';

	interface Props {
		trigger: HTMLElement | undefined;
		projectId: string;
		change: TreeChange;
		discardable: boolean;
		selectable: boolean;
		selectAllHunkLines: (hunk: DiffHunk) => void;
		unselectAllHunkLines: (hunk: DiffHunk) => void;
		invertHunkSelection: (hunk: DiffHunk) => void;
	}

	const {
		trigger,
		projectId,
		change,
		discardable,
		selectable,
		selectAllHunkLines,
		unselectAllHunkLines,
		invertHunkSelection
	}: Props = $props();

	const stackService = inject(STACK_SERVICE);
	const ircService = inject(IRC_SERVICE);
	const projectService = inject(PROJECTS_SERVICE);

	const userSettings = inject(SETTINGS);
	const ircChats = $derived(ircService.getChats());
	const ircUsers = $derived(Object.keys(ircChats));
	const ircChannels = $derived(Object.keys(ircService.getChannels()));

	const filePath = $derived(change.path);
	let contextMenu: ReturnType<typeof ContextMenu> | undefined;

	function getDiscardLineLabel(item: HunkContextItem) {
		const { selectedLines } = item;

		if (selectedLines !== undefined && selectedLines.length > 0)
			return `Discard ${selectedLines.length} selected lines`;

		return '';
	}

	async function discardHunk(item: HunkContextItem) {
		const previousPathBytes =
			change.status.type === 'Rename' ? change.status.subject.previousPathBytes : null;

		unselectAllHunkLines(item.hunk);

		await stackService.discardChanges({
			projectId,
			worktreeChanges: [
				{
					previousPathBytes,
					pathBytes: change.pathBytes,
					hunkHeaders: [item.hunk]
				}
			]
		});
	}

	async function discardHunkLines(item: HunkContextItem) {
		if (item.selectedLines === undefined || item.selectedLines.length === 0) return;
		const previousPathBytes =
			change.status.type === 'Rename' ? change.status.subject.previousPathBytes : null;

		unselectAllHunkLines(item.hunk);

		await stackService.discardChanges({
			projectId,
			worktreeChanges: [
				{
					previousPathBytes,
					pathBytes: change.pathBytes,
					hunkHeaders: lineIdsToHunkHeaders(item.selectedLines, item.hunk.diff, 'discard')
				}
			]
		});
	}

	export function open(e: MouseEvent, item: HunkContextItem) {
		contextMenu?.open(e, item);
	}

	export function close() {
		contextMenu?.close();
	}
</script>

<ContextMenu testId={TestId.HunkContextMenu} bind:this={contextMenu} rightClickTrigger={trigger}>
	{#snippet children(item)}
		{#if isHunkContextItem(item)}
			<ContextMenuSection>
				{#if discardable}
					<ContextMenuItem
						testId={TestId.HunkContextMenu_DiscardChange}
						label="Discard change"
						onclick={() => {
							discardHunk(item);
							contextMenu?.close();
						}}
					/>
				{/if}
				{#if item.selectedLines !== undefined && item.selectedLines.length > 0 && discardable}
					<ContextMenuItem
						testId={TestId.HunkContextMenu_DiscardLines}
						label={getDiscardLineLabel(item)}
						onclick={() => {
							discardHunkLines(item);
							contextMenu?.close();
						}}
					/>
				{/if}
				{#if $ircEnabled}
					{#each ircUsers as ircUser}
						<ContextMenuItem
							label={ircUser}
							onclick={() => {
								const data = btoa(JSON.stringify({ change, diff: item.hunk }));
								if (!data) return;
								ircService.sendToNick(ircUser, change.path, data);
								contextMenu?.close();
							}}
						/>
					{/each}
					{#each ircChannels as ircChannel}
						<ContextMenuItem
							label={ircChannel}
							onclick={() => {
								const data = btoa(JSON.stringify({ change, diff: item.hunk }));
								if (!data) return;
								ircService.sendToGroup(ircChannel, change.path, data);
								contextMenu?.close();
							}}
						/>
					{/each}
				{/if}
				{#if item.beforeLineNumber !== undefined || item.afterLineNumber !== undefined}
					<ContextMenuItem
						testId={TestId.HunkContextMenu_OpenInEditor}
						label="Open in {$userSettings.defaultCodeEditor.displayName}"
						onclick={async () => {
							const project = await projectService.fetchProject(projectId);
							if (project?.path) {
								const path = getEditorUri({
									schemeId: $userSettings.defaultCodeEditor.schemeIdentifer,
									path: [vscodePath(project.path), filePath],
									line: item.beforeLineNumber ?? item.afterLineNumber
								});
								openExternalUrl(path);
							}
							contextMenu?.close();
						}}
					/>
				{/if}
			</ContextMenuSection>
			{#if selectable}
				<ContextMenuSection>
					<ContextMenuItem
						testId={TestId.HunkContextMenu_SelectAll}
						label="Select all"
						onclick={() => {
							selectAllHunkLines(item.hunk);
							contextMenu?.close();
						}}
					/>
					<ContextMenuItem
						testId={TestId.HunkContextMenu_UnselectAll}
						label="Unselect all"
						onclick={() => {
							unselectAllHunkLines(item.hunk);
							contextMenu?.close();
						}}
					/>
					<ContextMenuItem
						testId={TestId.HunkContextMenu_InvertSelection}
						label="Invert selection"
						onclick={() => {
							invertHunkSelection(item.hunk);
							contextMenu?.close();
						}}
					/>
				</ContextMenuSection>
			{/if}
		{:else}
			Malformed item :(
		{/if}
	{/snippet}
</ContextMenu>
