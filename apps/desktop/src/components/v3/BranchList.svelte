<script lang="ts">
	import ReduxResult from '$components/ReduxResult.svelte';
	import Branch from '$components/v3/Branch.svelte';
	import PushButton from '$components/v3/PushButton.svelte';
	import { StackService } from '$lib/stacks/stackService.svelte';
	import { inject } from '@gitbutler/shared/context';

	type Props = {
		projectId: string;
		stackId: string;
	};

	const { projectId, stackId }: Props = $props();
	const [stackService] = inject(StackService);
	const branchesResult = $derived(stackService.branches(projectId, stackId));
</script>

{#if stackId && branchesResult}
	<ReduxResult result={branchesResult.current}>
		{#snippet children(branches)}
			{#each branches as branch, i (branch.name)}
				{@const first = i === 0}
				{@const last = i === branches.length - 1}
				<Branch {projectId} {stackId} branchName={branch.name} {first} {last} />
				{#if branch.prNumber}
					<!-- {@const pr = github.getPullRequest(parseInt(branch.prNumber))}
					<ReduxResult result={pr.current}>
						{#snippet children(pr)}
							{pr.number}
						{/snippet}
					</ReduxResult> -->
				{/if}
			{/each}
			<PushButton {projectId} {stackId} multipleBranches={branches.length > 0} />
		{/snippet}
	</ReduxResult>
{/if}
