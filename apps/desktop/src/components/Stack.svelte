<script lang="ts">
	import CardOverlay from '$components/CardOverlay.svelte';
	import CollapsedLane from '$components/CollapsedLane.svelte';
	import ScrollableContainer from '$components/ConfigurableScrollableContainer.svelte';
	import Dropzone from '$components/Dropzone.svelte';
	import Resizer from '$components/Resizer.svelte';
	import SeriesList from '$components/SeriesList.svelte';
	import UncommittedChanges from '$components/UncommittedChanges.svelte';
	import StackHeader from '$components/v3/StackHeader.svelte';
	import laneNewSvg from '$lib/assets/empty-state/lane-new.svg?raw';
	import noChangesSvg from '$lib/assets/empty-state/lane-no-changes.svg?raw';
	import { BranchStack } from '$lib/branches/branch';
	import { BranchFileDzHandler, BranchHunkDzHandler } from '$lib/branches/dropHandler';
	import { DetailedCommit } from '$lib/commits/commit';
	import { StackPublishingService } from '$lib/history/stackPublishingService';
	import { FileIdSelection } from '$lib/selection/fileIdSelection';
	import { StackService } from '$lib/stacks/stackService.svelte';
	import { getContextStore, inject } from '@gitbutler/shared/context';
	import Button from '@gitbutler/ui/Button.svelte';
	import EmptyStatePlaceholder from '@gitbutler/ui/EmptyStatePlaceholder.svelte';
	import Spacer from '@gitbutler/ui/Spacer.svelte';
	import { intersectionObserver } from '@gitbutler/ui/utils/intersectionObserver';
	import { type Writable } from 'svelte/store';

	const {
		projectId,
		isLaneCollapsed,
		commitBoxOpen
	}: { projectId: string; isLaneCollapsed: Writable<boolean>; commitBoxOpen: Writable<boolean> } =
		$props();

	const [fileIdSelection, stackPublishingService, stackService] = inject(
		FileIdSelection,
		StackPublishingService,
		StackService
	);

	const stackStore = getContextStore(BranchStack);
	const stack = $derived($stackStore);

	const branchHasFiles = $derived(stack.files !== undefined && stack.files.length > 0);
	const branchHasNoCommits = $derived(stack.validSeries.flatMap((s) => s.patches).length === 0);
	const dzFileHandler = $derived(
		new BranchFileDzHandler(stackService, projectId, stack.id, stack.ownership)
	);
	const dzHunkHandler = $derived(new BranchHunkDzHandler(stackService, projectId, stack));

	let rsViewport = $state<HTMLElement>();
	let scrollEndVisible = $state(true);
	let isPushingCommits = $state(false);

	$effect(() => {
		if ($commitBoxOpen && stack.files.length === 0) {
			commitBoxOpen.set(false);
		}
	});

	const { upstreamPatches, branchPatches, hasConflicts } = $derived.by(() => {
		let hasConflicts = false;
		const upstreamPatches: DetailedCommit[] = [];
		const branchPatches: DetailedCommit[] = [];

		stack.validSeries.map((series) => {
			upstreamPatches.push(...series.upstreamPatches);
			branchPatches.push(...series.patches);
			hasConflicts = branchPatches.some((patch) => patch.conflicted);
		});

		return {
			upstreamPatches,
			branchPatches,
			hasConflicts
		};
	});

	const canPush = $derived.by(() => {
		if (upstreamPatches.filter((p) => !p.isIntegrated).length > 0) {
			return true;
		}
		if (branchPatches.some((p) => p.status === 'LocalOnly' || p.status === 'Remote')) {
			return true;
		}
		return false;
	});

	const [pushStack] = stackService.pushStack;

	async function push() {
		isPushingCommits = true;
		try {
			await pushStack({ projectId, stackId: stack.id, withForce: stack.requiresForce });
			await pushButlerReviewStacks();
		} finally {
			isPushingCommits = false;
		}
	}

	async function pushButlerReviewStacks() {
		const topPushableBranch = stack.validSeries.find((series) => series.reviewId);
		if (!topPushableBranch) return;

		await stackPublishingService.upsertStack(stack.id, topPushableBranch.name);
	}
</script>

{#if $isLaneCollapsed}
	<div class="collapsed-lane-container">
		<CollapsedLane uncommittedChanges={stack.files.length} {isLaneCollapsed} />
		<div class="collapsed-lane-divider" data-remove-from-draggable></div>
	</div>
{:else}
	<div class="resizer-wrapper">
		<div class="branch-card hide-native-scrollbar" class:target-branch={stack.selectedForChanges}>
			<ScrollableContainer
				wide
				padding={{
					top: 12,
					bottom: 12
				}}
			>
				<div bind:this={rsViewport} class="branch-card__contents">
					<StackHeader
						{projectId}
						{stack}
						onCollapseButtonClick={() => {
							$isLaneCollapsed = true;
						}}
					/>
					<div class="card-stacking">
						{#if branchHasFiles}
							<UncommittedChanges {commitBoxOpen} />
						{:else}
							<Dropzone handlers={[dzHunkHandler, dzFileHandler]}>
								{#snippet overlay({ hovered, activated })}
									<CardOverlay {hovered} {activated} label="Move here" />
								{/snippet}
								{#if branchHasNoCommits}
									<div class="new-branch">
										<EmptyStatePlaceholder image={laneNewSvg} width={180} bottomMargin={48}>
											{#snippet title()}
												This is a new lane
											{/snippet}
											{#snippet caption()}
												You can drag and drop files<br />or parts of files here.
											{/snippet}
										</EmptyStatePlaceholder>
									</div>
								{:else}
									<div class="no-changes">
										<EmptyStatePlaceholder image={noChangesSvg} width={180}>
											{#snippet caption()}
												No uncommitted<br />changes on this lane
											{/snippet}
										</EmptyStatePlaceholder>
									</div>
								{/if}
							</Dropzone>
						{/if}
						<Spacer dotted />
						<div style:position="relative">
							<div class="lane-branches">
								<SeriesList {projectId} {stack} />
							</div>
							{#if canPush}
								<div
									class="lane-branches__action"
									class:scroll-end-visible={scrollEndVisible}
									use:intersectionObserver={{
										callback: (entry) => {
											if (entry?.isIntersecting) {
												scrollEndVisible = false;
											} else {
												scrollEndVisible = true;
											}
										},
										options: {
											root: null,
											rootMargin: `-100% 0px 0px 0px`,
											threshold: 0
										}
									}}
								>
									<Button
										style="neutral"
										wide
										loading={isPushingCommits}
										disabled={hasConflicts}
										tooltip={hasConflicts
											? 'In order to push, please resolve any conflicted commits.'
											: undefined}
										onclick={push}
									>
										{stack.requiresForce
											? 'Force push'
											: stack.validSeries.length > 1
												? 'Push all'
												: 'Push'}
									</Button>
								</div>
							{/if}
						</div>
					</div>
				</div>
			</ScrollableContainer>
			{#if rsViewport}
				<Resizer
					viewport={rsViewport}
					persistId={'stackWidth_' + projectId}
					defaultValue={24}
					direction="right"
					minWidth={25}
					imitateBorder
					borderColor={$fileIdSelection.length === 1 ? 'trnsparent' : 'var(--clr-border-2)'}
				/>
			{/if}
		</div>
	</div>
{/if}

<style lang="postcss">
	.resizer-wrapper {
		display: flex;
		position: relative;
		height: 100%;
	}

	.branch-card {
		position: relative;
		height: 100%;
		overflow-x: hidden;
		overflow-y: scroll;
	}

	.lane-branches {
		display: flex;
		flex-direction: column;
	}

	.lane-branches__action {
		z-index: var(--z-lifted);
		position: sticky;
		bottom: 0;
		margin: 0 -12px 1px -12px;
		padding: 0 12px 12px;
		transition: background-color var(--transition-fast);

		&:global(.merge-all > button:not(:last-child)) {
			margin-bottom: 8px;
		}

		&:after {
			display: block;
			z-index: -1;
			position: absolute;
			bottom: 0;
			left: 0;
			width: 100%;
			height: calc(100% + 12px);

			transform: translateY(0);
			border-top: 1px solid var(--clr-border-2);
			background-color: var(--clr-bg-1);
			content: '';
			opacity: 0;
			transition: opacity var(--transition-fast);
		}

		&:not(.scroll-end-visible):after {
			opacity: 1;
		}
	}

	.branch-card__contents {
		display: flex;
		position: relative;
		flex: 1;
		flex-direction: column;
		min-height: 100%;
		padding: 12px 12px 0;
	}

	.card-stacking {
		display: flex;
		flex: 1;
		flex-direction: column;
	}

	.no-changes,
	.new-branch {
		border: 1px solid var(--clr-border-2);
		border-top-width: 0;
		border-radius: 0 0 var(--radius-m) var(--radius-m) !important;
		background: var(--clr-bg-1);
	}

	.new-branch,
	.no-changes {
		display: flex;
		flex-grow: 1;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		border-top-width: 0px;
		color: var(--clr-scale-ntrl-60);
		cursor: default; /* was defaulting to text cursor */
	}

	/* COLLAPSED LANE */
	.collapsed-lane-container {
		display: flex;
		position: relative;
		flex-direction: column;
		height: 100%;
		padding: 12px;
	}

	.collapsed-lane-divider {
		position: absolute;
		top: 0;
		right: 0;
		width: 1px;
		height: 100%;
		background-color: var(--clr-border-2);
	}
</style>
