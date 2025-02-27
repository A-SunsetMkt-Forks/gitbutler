<script lang="ts">
	import { type ColumnTypes, type AvatarsType, type ChangesType } from './types';
	import CommitsGraph from '$lib/components/review/CommitsGraph.svelte';
	import CommitStatusBadge, { type CommitStatusType } from '@gitbutler/ui/CommitStatusBadge.svelte';
	import Icon from '@gitbutler/ui/Icon.svelte';
	import AvatarGroup from '@gitbutler/ui/avatar/AvatarGroup.svelte';
	import dayjs from 'dayjs';
	import type { Branch } from '@gitbutler/shared/branches/types';
	import { goto } from '$app/navigation';

	type Props = {
		columns: Array<{
			key: keyof ColumnTypes;
			value?: ColumnTypes[keyof ColumnTypes];
			tooltip?: string;
		}>;
		href?: string;
		separatedTop?: boolean;
		separatedBottom?: boolean;
	};

	let { columns, href, separatedTop, separatedBottom }: Props = $props();

	function handleLinkClick(event: MouseEvent | KeyboardEvent) {
		if (!href) return;
		event.preventDefault();
		goto(href);
	}

	console.log(columns);
</script>

<tr
	class="text-12 dynrow"
	role="button"
	tabIndex={0}
	onclick={handleLinkClick}
	onkeydown={(e) => e.key === 'Enter' && handleLinkClick(e)}
	class:dynrow-separatedTop={separatedTop}
	class:dynrow-separatedBottom={separatedBottom}
>
	{#each columns as { key, value, tooltip }}
		<td class={[`truncate dynclmn-td dynclmn-${key}-td`]}>
			<div
				class={[
					'dynclmn',
					`dynclmn-${key}`,
					{ 'text-13 text-bold truncate': key === 'title' },
					{ 'text-12 truncate': key === 'string' },
					{ 'dynclmn-number': key === 'number' }
				]}
				title={tooltip}
			>
				{#if key === 'title'}
					<div class="truncate" title={tooltip}>
						{value}
					</div>
				{:else if key === 'number'}
					{value}
				{:else if key === 'commitGraph'}
					<CommitsGraph branch={value as Branch} />
				{:else if key === 'avatars'}
					<AvatarGroup avatars={value as Array<AvatarsType>}></AvatarGroup>
				{:else if key === 'reviewers'}
					<div class="dynclmn-reviewers">
						{#if (value as { approvers: Array<AvatarsType> }).approvers.length > 0 || (value as { rejectors: Array<AvatarsType> }).rejectors.length > 0}
							<AvatarGroup
								avatars={(value as { approvers: Array<AvatarsType> }).approvers}
								maxAvatars={2}
								icon="tick-small"
								iconColor="success"
							/>
							<AvatarGroup
								avatars={(value as { rejectors: Array<AvatarsType> }).rejectors}
								maxAvatars={2}
								icon="refresh-small"
								iconColor="warning"
							/>
						{:else}
							<span class="dynclmn-placeholder">No reviews</span>
						{/if}
					</div>
				{:else if key === 'date'}
					{dayjs(value as Date).fromNow()}
				{:else if key === 'status'}
					<CommitStatusBadge status={value as CommitStatusType} />
				{:else if key === 'changes'}
					<div class="dynclmn-changes">
						<span class="dynclmn-changes_additions">+{(value as ChangesType).additions}</span>
						<span class="dynclmn-changes_deletions">-{(value as ChangesType).deletions}</span>
					</div>
				{:else if key === 'comments'}
					<div class="text-12 dynclmn-comments" class:dynclmn-placeholder={!value}>
						<span>{value}</span>
						<div class="dynclmn-comments-icon"><Icon name="comments-small" /></div>
					</div>
				{:else}
					{value}
				{/if}
			</div>
		</td>
	{/each}
</tr>

<style lang="postcss">
	.dynrow {
		cursor: pointer;
		transition: background-color var(--transition-fast);

		&:hover {
			.dynclmn {
				background-color: var(--clr-bg-1-muted);
			}
		}

		&:last-child {
			.dynclmn-td:first-child .dynclmn {
				border-bottom-left-radius: var(--radius-ml);
			}
			.dynclmn-td:last-child .dynclmn {
				border-bottom-right-radius: var(--radius-ml);
			}
		}
	}

	.dynclmn-td {
		padding: 0;

		&:last-child .dynclmn {
			border-right: 1px solid var(--clr-border-2);
		}
		&:first-child .dynclmn {
			border-left: 1px solid var(--clr-border-2);
		}
	}

	.dynclmn {
		display: flex;
		align-items: center;

		padding: 0 var(--cell-padding);
		height: 58px;
		color: var(--clr-text-2);
		background-color: var(--clr-bg-1);
		border-bottom: 1px solid var(--clr-border-2);
	}

	/* CHNAGES CLMN */
	.dynclmn-changes {
		display: flex;
		justify-content: flex-end;
	}
	.dynclmn-changes_additions {
		color: var(--clr-theme-succ-element);
		text-align: right;
	}
	.dynclmn-changes_deletions {
		color: var(--clr-theme-err-element);
		text-align: right;
		padding-left: 6px;
	}

	/* COMMENTS CLMN */
	.dynclmn-comments {
		display: flex;
		gap: 5px;
		justify-content: flex-end;
		align-items: center;
	}

	.dynclmn-comments-icon {
		display: flex;
	}

	/* TYPES */
	.dynclmn-reviewers {
		display: flex;
		gap: 10px;
	}

	.dynclmn-title-td {
		width: 60%;
	}
	.dynclmn-title {
		display: grid;
		color: var(--clr-text-1);
	}

	.dynclmn-number {
		font-family: var(--fontfamily-mono);
		font-size: 12px;
		text-align: right;
		justify-content: flex-end;
	}

	.dynclmn-commitGraph-td {
		min-width: 120px;
	}

	/* MODIFIERS */
	.dynclmn-placeholder {
		opacity: 0.4;
	}

	.dynrow-separatedTop {
		.dynclmn-td {
			padding-top: 2px;

			&:first-child .dynclmn {
				border-top-left-radius: var(--radius-ml);
			}
			&:last-child .dynclmn {
				border-top-right-radius: var(--radius-ml);
			}
		}

		.dynclmn {
			border-top: 1px solid var(--clr-border-2);
		}
	}

	.dynrow-separatedBottom {
		.dynclmn-td {
			padding-bottom: 2px;

			&:first-child .dynclmn {
				border-bottom-left-radius: var(--radius-ml);
			}

			&:last-child .dynclmn {
				border-bottom-right-radius: var(--radius-ml);
			}
		}

		.dynclmn {
			border-bottom: 1px solid var(--clr-border-2);
		}
	}
</style>
